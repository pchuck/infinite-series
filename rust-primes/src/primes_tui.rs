use std::sync::Arc;
use std::time::{Duration, Instant};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Gauge, List, Paragraph, Tabs};
use ratatui::Terminal;

use primes::ProgressCallback;
use primes::{generate_primes, ProgressBar, DEFAULT_SEGMENT_SIZE, PARALLEL_THRESHOLD};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Tab {
    Generate,
    Primes,
    Stats,
}

impl Tab {
    fn all() -> Vec<Self> {
        vec![Tab::Generate, Tab::Primes, Tab::Stats]
    }

    fn label(&self) -> &'static str {
        match self {
            Tab::Generate => " Generate ",
            Tab::Primes => " Primes ",
            Tab::Stats => " Stats ",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum InputField {
    N,
    Workers,
    SegmentSize,
}

impl InputField {
    fn label(&self) -> &'static str {
        match self {
            InputField::N => "n",
            InputField::Workers => "workers",
            InputField::SegmentSize => "segment",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum ActiveInput {
    None,
    Field(InputField),
}

struct AppState {
    n_input: String,
    workers: usize,
    segment_size_input: String,
    parallel: bool,
    show_progress: bool,

    active_tab: Tab,
    active_input: ActiveInput,

    generation_state: GenerationState,
    primes: Vec<usize>,
    prime_search: String,

    elapsed: Duration,
    last_prime: Option<usize>,
    error_message: Option<String>,

    start_time: Instant,
}

impl AppState {
    fn new() -> Self {
        let workers = std::thread::available_parallelism()
            .map(|p| p.get())
            .unwrap_or(4);

        Self {
            n_input: "100000".to_string(),
            workers,
            segment_size_input: DEFAULT_SEGMENT_SIZE.to_string(),
            parallel: false,
            show_progress: true,

            active_tab: Tab::Generate,
            active_input: ActiveInput::None,

            generation_state: GenerationState::Idle,
            primes: Vec::new(),
            prime_search: String::new(),

            elapsed: Duration::ZERO,
            last_prime: None,
            error_message: None,

            start_time: Instant::now(),
        }
    }

    fn n_value(&self) -> Result<usize, String> {
        self.n_input
            .parse::<usize>()
            .map_err(|e| format!("Invalid n: {}", e))
    }

    fn segment_size_value(&self) -> Result<usize, String> {
        self.segment_size_input
            .parse::<usize>()
            .map_err(|e| format!("Invalid segment size: {}", e))
    }

    fn effective_workers(&self) -> usize {
        if self.workers == 0 {
            std::thread::available_parallelism()
                .map(|p| p.get())
                .unwrap_or(4)
        } else {
            self.workers
        }
    }

    fn reset(&mut self) {
        self.primes.clear();
        self.elapsed = Duration::ZERO;
        self.last_prime = None;
        self.error_message = None;
        self.start_time = Instant::now();
        self.generation_state = GenerationState::Idle;
    }

    fn start_generation(&mut self) -> Result<(), String> {
        let n = self.n_value()?;
        if n <= 2 {
            return Err("n must be greater than 2".to_string());
        }

        let segment_size = self.segment_size_value()?;
        if segment_size == 0 {
            return Err("segment size cannot be zero".to_string());
        }

        if self.parallel && n < PARALLEL_THRESHOLD {
            eprintln!("Warning: --parallel ignored for n < {}", PARALLEL_THRESHOLD);
        }

        self.reset();
        self.generation_state = GenerationState::Running;
        Ok(())
    }

    fn update_primes(&mut self, new_primes: Vec<usize>) {
        let elapsed = self.start_time.elapsed();

        if new_primes.len() > self.primes.len() {
            let added = &new_primes[self.primes.len()..];
            if !added.is_empty() {
                self.last_prime = Some(*added.last().unwrap());
            }
        }

        self.primes = new_primes;
        self.elapsed = elapsed;
    }

    fn finish_generation(&mut self, result: Result<Vec<usize>, String>) {
        match result {
            Ok(primes) => {
                self.update_primes(primes);
                self.generation_state = GenerationState::Complete;
            }
            Err(e) => {
                self.generation_state = GenerationState::Error;
                self.error_message = Some(e);
            }
        }
    }

    fn prime_rate(&self) -> f64 {
        let secs = self.elapsed.as_secs_f64();
        if secs > 0.0 {
            self.primes.len() as f64 / secs
        } else {
            0.0
        }
    }

    fn filtered_primes(&self) -> Vec<usize> {
        if self.prime_search.is_empty() {
            return self.primes.clone();
        }

        let search = self.prime_search.to_lowercase();
        self.primes
            .iter()
            .filter(|&&p| p.to_string().contains(&search))
            .copied()
            .collect()
    }

    fn algorithm_name(&self) -> &'static str {
        let n = self.n_value().unwrap_or(0);
        if self.parallel && n >= PARALLEL_THRESHOLD {
            "Parallel Segmented Sieve"
        } else if n >= DEFAULT_SEGMENT_SIZE {
            "Segmented Sieve"
        } else {
            "Classic Sieve of Eratosthenes"
        }
    }

    fn progress(&self) -> f64 {
        let n = self.n_value().unwrap_or(0);
        let segment_size = self.segment_size_value().unwrap_or(DEFAULT_SEGMENT_SIZE);
        let total_segments = n.div_ceil(segment_size);

        match self.generation_state {
            GenerationState::Running => {
                let completed = total_segments.saturating_sub(1);
                completed as f64 / total_segments.max(1) as f64
            }
            GenerationState::Complete => 1.0,
            _ => 0.0,
        }
    }

    fn format_rate(rate: f64) -> String {
        if rate >= 1_000_000.0 {
            format!("{:.2}M/s", rate / 1_000_000.0)
        } else if rate >= 1_000.0 {
            format!("{:.2}K/s", rate / 1_000.0)
        } else {
            format!("{:.2}/s", rate)
        }
    }

    fn format_number(n: usize) -> String {
        let s = n.to_string();
        let len = s.len();
        if len <= 3 {
            return s;
        }

        let mut result = String::with_capacity(len + len / 3);
        for (i, ch) in s.chars().enumerate() {
            if i > 0 && (len - i).is_multiple_of(3) {
                result.push(',');
            }
            result.push(ch);
        }
        result
    }

    fn format_duration(d: Duration) -> String {
        let secs = d.as_secs_f64();
        if secs < 1.0 {
            format!("{:.3}s", secs)
        } else if secs < 60.0 {
            format!("{:.2}s", secs)
        } else {
            let mins = secs / 60.0;
            format!("{:.2}m", mins)
        }
    }

    fn prime_density(&self) -> f64 {
        let n = self.n_value().unwrap_or(1);
        if n == 0 {
            return 0.0;
        }
        (self.primes.len() as f64 / n as f64) * 100.0
    }

    fn prime_gap_stats(&self) -> (Option<usize>, Option<usize>, Option<f64>) {
        if self.primes.len() < 2 {
            return (None, None, None);
        }

        let mut min_gap = usize::MAX;
        let mut max_gap = 0usize;
        let mut total_gap: f64 = 0.0;
        let mut gaps = 0usize;

        for i in 1..self.primes.len() {
            let gap = self.primes[i] - self.primes[i - 1];
            if gap > 0 {
                total_gap += gap as f64;
                gaps += 1;
            }
            if gap < min_gap {
                min_gap = gap;
            }
            if gap > max_gap {
                max_gap = gap;
            }
        }

        (
            Some(min_gap),
            if gaps > 0 { Some(max_gap) } else { None },
            if gaps > 0 {
                Some(total_gap / gaps as f64)
            } else {
                None
            },
        )
    }

    fn twin_primes_count(&self) -> usize {
        if self.primes.len() < 2 {
            return 0;
        }

        let mut count = 0usize;
        for i in 1..self.primes.len() {
            if self.primes[i] - self.primes[i - 1] == 2 {
                count += 1;
            }
        }
        count
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum GenerationState {
    Idle,
    Running,
    Complete,
    Error,
}

fn handle_key_after_running(
    app: &mut AppState,
    key: event::KeyEvent,
    prime_thread: &mut Option<std::thread::JoinHandle<Result<Vec<usize>, primes::PrimeGenError>>>,
    progress_bar: &mut Option<Arc<ProgressBar>>,
) {
    let was_running = app.generation_state == GenerationState::Running;
    handle_key(app, key);

    if was_running {
        let n = app.n_value().unwrap_or(0);
        let segment_size = app.segment_size_value().unwrap_or(DEFAULT_SEGMENT_SIZE);
        let workers = app.effective_workers();
        let parallel = app.parallel;

        *progress_bar = if app.show_progress {
            let total_segments = n.div_ceil(segment_size);
            Some(Arc::new(ProgressBar::new(
                total_segments.max(1),
                "Generating primes",
                segment_size,
            )))
        } else {
            None
        };

        let progress_callback = progress_bar.as_ref().map(|bar| {
            let bar = Arc::clone(bar);
            Arc::new(move |delta: usize| bar.update(delta)) as ProgressCallback
        });

        let handle = std::thread::spawn(move || {
            generate_primes(
                n,
                parallel,
                Some(workers),
                Some(segment_size),
                progress_callback,
            )
        });

        *prime_thread = Some(handle);
    }

    if let Some(handle) = prime_thread.take() {
        match handle.join() {
            Ok(Ok(primes)) => app.finish_generation(Ok(primes)),
            Ok(Err(e)) => app.finish_generation(Err(e.to_string())),
            Err(_) => app.finish_generation(Err("worker thread panicked".to_string())),
        }
    }

    if matches!(app.active_input, ActiveInput::None) && key.code == KeyCode::Esc {
        std::process::exit(0);
    }
}

fn run(app: &mut AppState) -> Result<(), String> {
    use crossterm::cursor::Hide;
    use crossterm::execute;
    use crossterm::terminal::{DisableLineWrap, EnterAlternateScreen};

    let stdout = std::io::stdout();
    crossterm::terminal::enable_raw_mode()
        .map_err(|e| format!("Failed to enable raw mode: {}", e))?;
    let mut guard = stdout.lock();
    execute!(guard, EnterAlternateScreen, Hide, DisableLineWrap)
        .map_err(|e| format!("Failed to set terminal: {}", e))?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal =
        Terminal::new(backend).map_err(|e| format!("Failed to create terminal: {}", e))?;

    let (tx, rx) = std::sync::mpsc::channel();
    let event_thread = std::thread::spawn(move || loop {
        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                if key.kind == KeyEventKind::Press && tx.send(key).is_err() {
                    break;
                }
            }
        }
    });

    let mut prime_thread: Option<
        std::thread::JoinHandle<Result<Vec<usize>, primes::PrimeGenError>>,
    > = None;
    let mut progress_bar: Option<Arc<ProgressBar>> = None;

    loop {
        terminal
            .draw(|frame| draw(frame, app, progress_bar.as_ref()))
            .unwrap();

        let key = match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(k) => k,
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
            Err(_) => break,
        };

        if app.generation_state == GenerationState::Running {
            handle_key_after_running(app, key, &mut prime_thread, &mut progress_bar);
            continue;
        }

        handle_key(app, key);

        if app.generation_state == GenerationState::Running {
            let n = app.n_value().unwrap_or(0);
            let segment_size = app.segment_size_value().unwrap_or(DEFAULT_SEGMENT_SIZE);
            let workers = app.effective_workers();
            let parallel = app.parallel;

            progress_bar = if app.show_progress {
                let total_segments = n.div_ceil(segment_size);
                Some(Arc::new(ProgressBar::new(
                    total_segments.max(1),
                    "Generating primes",
                    segment_size,
                )))
            } else {
                None
            };

            let progress_callback = progress_bar.as_ref().map(|bar| {
                let bar = Arc::clone(bar);
                Arc::new(move |delta: usize| bar.update(delta)) as ProgressCallback
            });

            let handle = std::thread::spawn(move || {
                generate_primes(
                    n,
                    parallel,
                    Some(workers),
                    Some(segment_size),
                    progress_callback,
                )
            });

            prime_thread = Some(handle);
        }

        if let Some(handle) = prime_thread.take() {
            match handle.join() {
                Ok(Ok(primes)) => app.finish_generation(Ok(primes)),
                Ok(Err(e)) => app.finish_generation(Err(e.to_string())),
                Err(_) => app.finish_generation(Err("worker thread panicked".to_string())),
            }
        }

        if matches!(app.active_input, ActiveInput::None) && key.code == KeyCode::Esc {
            break;
        }
    }

    drop(terminal);
    let _ = crossterm::terminal::disable_raw_mode();
    let _ = event_thread.join();

    Ok(())
}

fn handle_key(app: &mut AppState, key: event::KeyEvent) {
    match app.active_input {
        ActiveInput::None => match key.code {
            KeyCode::Char('1') => {
                app.active_tab = Tab::Generate;
            }
            KeyCode::Char('2') => {
                app.active_tab = Tab::Primes;
            }
            KeyCode::Char('3') => {
                app.active_tab = Tab::Stats;
            }
            KeyCode::Tab if app.generation_state == GenerationState::Idle => match app.active_tab {
                Tab::Generate => app.active_tab = Tab::Primes,
                Tab::Primes => app.active_tab = Tab::Stats,
                Tab::Stats => app.active_tab = Tab::Generate,
            },
            KeyCode::Char('g') | KeyCode::Enter
                if app.generation_state == GenerationState::Idle =>
            {
                let _ = app.start_generation();
            }
            KeyCode::Char('r') if app.generation_state != GenerationState::Running => {
                let _ = app.start_generation();
            }
            KeyCode::Char('p') => {
                if app.generation_state == GenerationState::Idle
                    || app.generation_state == GenerationState::Complete
                {
                    app.parallel = !app.parallel;
                }
            }
            KeyCode::Char('q') | KeyCode::Esc => {
                std::process::exit(0);
            }
            KeyCode::Char('n') if app.generation_state == GenerationState::Idle => {
                app.active_input = ActiveInput::Field(InputField::N);
            }
            KeyCode::Char('w') if app.generation_state == GenerationState::Idle => {
                app.active_input = ActiveInput::Field(InputField::Workers);
            }
            KeyCode::Char('s') if app.generation_state == GenerationState::Idle => {
                app.active_input = ActiveInput::Field(InputField::SegmentSize);
            }
            KeyCode::Char('/') => {
                app.active_tab = Tab::Primes;
                app.prime_search.clear();
            }
            _ => {}
        },
        ActiveInput::Field(field) => {
            match key.code {
                KeyCode::Enter | KeyCode::Tab => {
                    app.active_input = ActiveInput::None;

                    if field == InputField::Workers && app.n_value().is_ok() {
                        let n = app.n_value().unwrap();
                        if app.workers == 0 || app.segment_size_input.parse::<usize>().is_err() {
                            // keep workers as-is
                        } else if app.workers == 0 && n < PARALLEL_THRESHOLD {
                            // auto workers is fine for small inputs
                        }
                    }

                    if field == InputField::N && app.start_generation().is_err() {
                        // error already set, keep in input mode
                        app.active_input = ActiveInput::Field(field);
                    }
                }
                KeyCode::Esc => {
                    app.active_input = ActiveInput::None;
                }
                KeyCode::Char(c) => {
                    match field {
                        InputField::N => {
                            if c.is_ascii_digit() && app.n_input.len() < 12 {
                                app.n_input.push(c);
                            } else if c == '\x08' || c == '\x7f' {
                                app.n_input.pop();
                            } else if c == '0' && app.n_input.is_empty() {
                                // allow leading zero but don't add it
                            } else if ('1'..='9').contains(&c) && app.n_input.is_empty() {
                                app.n_input.push(c);
                            }
                        }
                        InputField::Workers => {
                            if c.is_ascii_digit() && app.workers.to_string().len() < 4 {
                                let current = app.workers;
                                if current == 0 {
                                    app.workers = c.to_digit(10).unwrap() as usize;
                                } else {
                                    app.workers = current * 10 + c.to_digit(10).unwrap() as usize;
                                }
                            } else if c == '\x08' || c == '\x7f' {
                                app.workers /= 10;
                            } else if c == 'a' || c == 'A' {
                                app.workers = 0; // auto
                            }
                        }
                        InputField::SegmentSize => {
                            if c.is_ascii_digit() && app.segment_size_input.len() < 12 {
                                app.segment_size_input.push(c);
                            } else if c == '\x08' || c == '\x7f' {
                                app.segment_size_input.pop();
                            } else if c == '0' && app.segment_size_input.is_empty() {
                                // allow leading zero but don't add it
                            } else if ('1'..='9').contains(&c) && app.segment_size_input.is_empty()
                            {
                                app.segment_size_input.push(c);
                            }
                        }
                    }
                }
                KeyCode::Backspace => match field {
                    InputField::N => {
                        app.n_input.pop();
                    }
                    InputField::SegmentSize => {
                        app.segment_size_input.pop();
                    }
                    InputField::Workers => {
                        if app.workers > 0 {
                            app.workers /= 10;
                        } else {
                            app.workers = 0;
                        }
                    }
                },
                _ => {}
            }
        }
    }

    if app.active_tab == Tab::Primes && !app.prime_search.is_empty() {
        match key.code {
            KeyCode::Char(c) => {
                app.prime_search.push(c);
            }
            KeyCode::Backspace | KeyCode::Delete => {
                app.prime_search.pop();
            }
            KeyCode::Esc => {
                app.prime_search.clear();
            }
            _ => {}
        }
    }
}

fn draw(frame: &mut ratatui::Frame, app: &AppState, progress_bar: Option<&Arc<ProgressBar>>) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(if app.active_tab == Tab::Generate {
                4
            } else {
                1
            }),
            Constraint::Min(0),
        ])
        .split(frame.area());

    // Header
    let header = build_header(app);
    frame.render_widget(header, chunks[0]);

    // Tab bar
    let tabs = Tabs::new(Tab::all().iter().map(|t| Line::from(t.label())))
        .block(Block::default().borders(Borders::BOTTOM))
        .select(match app.active_tab {
            Tab::Generate => 0,
            Tab::Primes => 1,
            Tab::Stats => 2,
        })
        .style(Style::default().fg(Color::Cyan))
        .highlight_style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );
    frame.render_widget(tabs, chunks[1]);

    // Main content
    let main_chunk = Layout::default()
        .direction(Direction::Vertical)
        .constraints(match app.active_tab {
            Tab::Generate => vec![Constraint::Length(12), Constraint::Min(0)],
            Tab::Primes => vec![
                Constraint::Length(if app.prime_search.is_empty() { 3 } else { 2 }),
                Constraint::Min(0),
            ],
            Tab::Stats => vec![Constraint::Min(0)],
        })
        .split(chunks[2]);

    match app.active_tab {
        Tab::Generate => draw_generate_tab(frame, main_chunk[0], app, progress_bar),
        Tab::Primes => draw_primes_tab(frame, main_chunk[0], app, main_chunk[1]),
        Tab::Stats => draw_stats_tab(frame, main_chunk[0], app),
    }

    // Footer / input line
    draw_input_line(frame, &chunks[2], app);
}

fn build_header(app: &AppState) -> Paragraph<'_> {
    let algorithm = app.algorithm_name();
    let rate = app.prime_rate();

    let status_text = match &app.generation_state {
        GenerationState::Idle => Span::raw(" Press 'g' or Enter to generate primes "),
        GenerationState::Running => {
            Span::raw(" Generating... ").style(Style::default().fg(Color::Yellow))
        }
        GenerationState::Complete => Span::raw(" Complete! "),
        GenerationState::Error => {
            Span::raw(" Error - see below ").style(Style::default().fg(Color::Red))
        }
    };

    let title = vec![
        Span::styled(
            " Prime Generator ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" | "),
        Span::styled(algorithm, Style::default().fg(Color::Green)),
    ];

    let mut stat_spans = vec![Span::raw(" ")];

    if app.generation_state == GenerationState::Running
        || app.generation_state == GenerationState::Complete
    {
        stat_spans.push(Span::raw(format!(
            "{} primes | {} | ",
            AppState::format_number(app.primes.len()),
            AppState::format_rate(rate)
        )));
    }

    stat_spans.push(status_text);

    let header = Paragraph::new(vec![
        Line::from(title),
        Line::from(stat_spans).alignment(Alignment::Center),
    ]);

    header
}

fn draw_generate_tab(
    frame: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    app: &AppState,
    progress_bar: Option<&Arc<ProgressBar>>,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),
            Constraint::Length(if app.show_progress { 5 } else { 0 }),
            Constraint::Min(0),
        ])
        .split(area);

    // Settings block
    let parallel_display = if app.parallel { "YES" } else { "no" };
    let parallel_note = if app.parallel && app.n_value().unwrap_or(0) < PARALLEL_THRESHOLD {
        "ignored for n < 5M"
    } else {
        ""
    };

    let workers_display = if app.workers == 0 {
        "auto".to_string()
    } else {
        app.workers.to_string()
    };

    let mut settings = vec![
        Line::from(Span::styled(
            format!(" n: {}", app.n_input),
            if matches!(app.active_input, ActiveInput::Field(InputField::N)) {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            },
        )),
        Line::from(Span::raw(" ")),
        Line::from(Span::styled(
            format!(" workers: {}", workers_display),
            if matches!(app.active_input, ActiveInput::Field(InputField::Workers)) {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            },
        )),
        Line::from(Span::styled(
            format!(" segment: {}", app.segment_size_input),
            if matches!(
                app.active_input,
                ActiveInput::Field(InputField::SegmentSize)
            ) {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            },
        )),
    ];

    let parallel_line = format!(
        " parallel: {} (press 'p' to toggle) [{}]",
        parallel_display, parallel_note
    );
    settings.push(Line::from(parallel_line.as_str()));

    let settings_block = Block::default()
        .title(" Configuration ")
        .borders(Borders::ALL)
        .style(Style::default().fg(Color::White));

    let settings_widget = Paragraph::new(settings).block(settings_block);
    frame.render_widget(settings_widget, chunks[0]);

    // Progress bar
    if app.show_progress
        && (app.generation_state == GenerationState::Running
            || app.generation_state == GenerationState::Complete)
    {
        let progress = app.progress();
        let gauge = Gauge::default()
            .block(Block::default().title(" Progress ").borders(Borders::ALL))
            .gauge_style(Style::default().fg(Color::Cyan))
            .ratio(progress)
            .label(format!("{:.1}%", progress * 100.0));

        frame.render_widget(gauge, chunks[1]);
    } else if progress_bar.is_none() {
        let block = Block::default().title(" Progress ").borders(Borders::ALL);
        let widget = Paragraph::new(" Not generating ").block(block);
        frame.render_widget(widget, chunks[1]);
    }

    // Prime list preview
    let mut prime_lines: Vec<Line> = Vec::new();

    if app.primes.is_empty() && app.generation_state != GenerationState::Running {
        prime_lines.push(Line::from(" No primes generated yet. Press 'g' to start."));
    } else if app.generation_state == GenerationState::Running {
        prime_lines.push(Line::from(Span::styled(
            " Generating... showing live results below",
            Style::default().fg(Color::Yellow),
        )));
    } else if app.generation_state == GenerationState::Complete {
        prime_lines.push(Line::from(Span::styled(
            " Generation complete",
            Style::default().fg(Color::Green),
        )));
    }

    // Show first 20 primes
    let preview_count = app
        .primes
        .iter()
        .take(20)
        .map(|p| Line::from(Span::raw(AppState::format_number(*p))))
        .collect::<Vec<_>>();

    prime_lines.extend(preview_count);

    if !app.primes.is_empty() && app.primes.len() > 20 {
        prime_lines.push(Line::from(Span::styled(
            format!(
                " ... and {} more (press Tab/2 to view Primes tab)",
                app.primes.len() - 20
            ),
            Style::default().fg(Color::Gray),
        )));
    }

    let list =
        List::new(prime_lines).block(Block::default().title(" Primes ").borders(Borders::ALL));

    frame.render_widget(list, chunks[2]);
}

fn draw_primes_tab(
    frame: &mut ratatui::Frame,
    area: ratatui::layout::Rect,
    app: &AppState,
    bottom_area: ratatui::layout::Rect,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Min(0)])
        .split(area);

    let filtered = app.filtered_primes();

    if app.primes.is_empty() {
        let block = Block::default().title(" Primes ").borders(Borders::ALL);
        let widget = Paragraph::new(vec![Line::from(" No primes to display.")]).block(block);
        frame.render_widget(widget, chunks[0]);
        return;
    }

    let mut lines: Vec<Line> = Vec::new();

    if !app.prime_search.is_empty() {
        lines.push(Line::from(Span::styled(
            format!(
                " Search: \"{}\" ({} results) - press Esc to clear",
                app.prime_search,
                filtered.len()
            ),
            Style::default().fg(Color::Yellow),
        )));
    }

    lines.push(Line::from(Span::styled(
        format!(" Showing {} of {} primes", filtered.len(), app.primes.len()),
        Style::default().fg(Color::Gray),
    )));

    lines.push(Line::from(""));

    // Show primes in rows of 10
    let display_count = filtered.len().min(bottom_area.height as usize - 5);

    for chunk in filtered
        .iter()
        .take(display_count)
        .collect::<Vec<_>>()
        .chunks(10)
    {
        let primes_str: Vec<Span> = chunk
            .iter()
            .map(|p| Span::raw(format!("{:>12}", AppState::format_number(**p))))
            .collect();

        let mut line = Vec::new();
        for (i, span) in primes_str.into_iter().enumerate() {
            if i > 0 {
                line.push(Span::raw(" | "));
            }
            line.push(span);
        }

        lines.push(Line::from(line));
    }

    if filtered.len() > 20 {
        lines.push(Line::from(Span::styled(
            format!(" ... showing first {} of {}", display_count, filtered.len()),
            Style::default().fg(Color::Gray),
        )));
    }

    let list = List::new(lines).block(Block::default().title(" Primes ").borders(Borders::ALL));

    frame.render_widget(list, chunks[0]);
}

fn draw_stats_tab(frame: &mut ratatui::Frame, area: ratatui::layout::Rect, app: &AppState) {
    let mut lines: Vec<Line> = Vec::new();

    if app.primes.is_empty() && app.generation_state == GenerationState::Idle {
        lines.push(Line::from(
            " No stats available yet. Generate some primes first.",
        ));
    } else {
        let (min_gap, max_gap, avg_gap) = app.prime_gap_stats();

        lines.push(Line::from(Span::styled(
            " General Statistics ",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )));
        lines.push(Line::from(""));

        let n = app.n_value().unwrap_or(0);
        lines.push(Line::from(format!(
            "  Upper bound (n):     {}",
            AppState::format_number(n)
        )));
        lines.push(Line::from(format!(
            "  Primes found:        {}",
            AppState::format_number(app.primes.len())
        )));
        lines.push(Line::from(format!(
            "  Prime density:       {:.4}%",
            app.prime_density()
        )));
        lines.push(Line::from(format!(
            "  Algorithm:           {}",
            app.algorithm_name()
        )));
        lines.push(Line::from(format!(
            "  Parallel:            {}",
            if app.parallel { "yes" } else { "no" }
        )));
        let workers_text = if app.workers == 0 {
            "auto".to_string()
        } else {
            app.workers.to_string()
        };
        lines.push(Line::from(format!(
            "  Workers:             {}",
            workers_text
        )));
        lines.push(Line::from(format!(
            "  Segment size:        {}",
            AppState::format_number(app.segment_size_value().unwrap_or(DEFAULT_SEGMENT_SIZE))
        )));
        lines.push(Line::from(""));

        if app.generation_state != GenerationState::Idle {
            lines.push(Line::from(Span::styled(
                " Performance ",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(format!(
                "  Elapsed time:        {}",
                AppState::format_duration(app.elapsed)
            )));
            lines.push(Line::from(format!(
                "  Generation rate:     {}",
                AppState::format_rate(app.prime_rate())
            )));
            lines.push(Line::from(format!(
                "  Largest prime:       {}",
                AppState::format_number(app.last_prime.unwrap_or(0))
            )));
            lines.push(Line::from(""));
        }

        if let (Some(min), Some(max), Some(avg)) = (min_gap, max_gap, avg_gap) {
            lines.push(Line::from(Span::styled(
                " Prime Gaps ",
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(format!("  Minimum gap:         {}", min)));
            lines.push(Line::from(format!("  Maximum gap:         {}", max)));
            lines.push(Line::from(format!("  Average gap:         {:.2}", avg)));
            lines.push(Line::from(""));
        }

        let twin_count = app.twin_primes_count();
        if twin_count > 0 {
            lines.push(Line::from(Span::styled(
                " Twin Primes ",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )));
            lines.push(Line::from(""));
            lines.push(Line::from(format!("  Twin prime pairs:    {}", twin_count)));
        }
    }

    let paragraph =
        Paragraph::new(lines).block(Block::default().title(" Statistics ").borders(Borders::ALL));

    frame.render_widget(paragraph, area);
}

fn draw_input_line(frame: &mut ratatui::Frame, main_area: &ratatui::layout::Rect, app: &AppState) {
    let footer_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Min(0), Constraint::Length(65)])
        .split(*main_area);

    let mut help_lines = vec![Line::from("")];

    match app.active_input {
        ActiveInput::None => {
            help_lines.push(Line::from(vec![
                Span::styled(" [g/Enter] Generate ", Style::default().fg(Color::Yellow)),
                Span::raw(" | "),
                Span::styled("[1/Tab] Generate tab ", Style::default().fg(Color::Yellow)),
                Span::raw(" | "),
                Span::styled("[2/Tab] Primes ", Style::default().fg(Color::Yellow)),
                Span::raw(" | "),
                Span::styled("[3/Tab] Stats ", Style::default().fg(Color::Yellow)),
            ]));
            help_lines.push(Line::from(vec![
                Span::styled("[n] Edit n ", Style::default().fg(Color::Yellow)),
                Span::raw(" | "),
                Span::styled("[w] Workers ", Style::default().fg(Color::Yellow)),
                Span::raw(" | "),
                Span::styled("[s] Segment ", Style::default().fg(Color::Yellow)),
                Span::raw(" | "),
                Span::styled("[p] Parallel ", Style::default().fg(Color::Yellow)),
                Span::raw(" | "),
                Span::styled("[/] Search primes ", Style::default().fg(Color::Yellow)),
            ]));
            help_lines.push(Line::from(vec![Span::styled(
                "[q/Esc] Quit ",
                Style::default().fg(Color::Red),
            )]));
        }
        ActiveInput::Field(field) => {
            help_lines.push(Line::from(vec![
                Span::styled(
                    format!(" Editing {}: ", field.label()),
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled("Enter/Tab: confirm ", Style::default().fg(Color::Green)),
                Span::styled("Esc: cancel ", Style::default().fg(Color::Red)),
            ]));
        }
    }

    let help = Paragraph::new(help_lines)
        .block(Block::default().borders(Borders::TOP))
        .style(Style::default().fg(Color::Gray));

    frame.render_widget(help, footer_chunks[1]);
}

fn app_state_from_args() -> AppState {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        return AppState::new();
    }

    // Simple arg parsing - look for -n value
    let mut n = 100000u64;
    let mut parallel = false;

    for i in 1..args.len() {
        if args[i] == "-n" || args[i] == "--n" {
            if i + 1 < args.len() {
                n = args[i + 1].parse().unwrap_or(100000);
            }
        } else if args[i] == "-p" || args[i] == "--parallel" {
            parallel = true;
        } else if args[i] == "-P" || args[i] == "--progress" {
            // progress is enabled by default in TUI
        } else if args[i] == "-q" || args[i] == "--quiet" {
            // quiet mode not applicable for TUI
        } else if i > 0 && (args[i - 1] == "-n" || args[i - 1] == "--n") {
            // already handled above
        } else if args[i].parse::<u64>().is_ok() && n == 100000 {
            // positional argument for n
            if args.len() == 2 || (i == args.len() - 1 && i > 0) {
                n = args[i].parse().unwrap_or(100000);
            }
        }
    }

    AppState {
        n_input: n.to_string(),
        parallel,
        ..AppState::new()
    }
}

fn main() {
    let mut app = app_state_from_args();

    println!("Starting Prime Generator TUI...");
    println!("Press q or Esc to quit, g/Enter to generate primes.");

    if let Err(e) = run(&mut app) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_format_rate_zero() {
        assert_eq!(AppState::format_rate(0.0), "0.00/s");
    }

    #[test]
    fn test_format_rate_small() {
        assert_eq!(AppState::format_rate(42.5), "42.50/s");
    }

    #[test]
    fn test_format_rate_thousands() {
        assert_eq!(AppState::format_rate(1500.0), "1.50K/s");
    }

    #[test]
    fn test_format_rate_millions() {
        assert_eq!(AppState::format_rate(2_500_000.0), "2.50M/s");
    }

    #[test]
    fn test_format_rate_boundary_k() {
        assert_eq!(AppState::format_rate(999.99), "999.99/s");
    }

    #[test]
    fn test_format_rate_boundary_m() {
        assert_eq!(AppState::format_rate(999_000.0), "999.00K/s");
    }

    #[test]
    fn test_format_number_small() {
        assert_eq!(AppState::format_number(42), "42");
    }

    #[test]
    fn test_format_number_thousands() {
        assert_eq!(AppState::format_number(1234), "1,234");
    }

    #[test]
    fn test_format_number_millions() {
        assert_eq!(AppState::format_number(1000000), "1,000,000");
    }

    #[test]
    fn test_format_number_billions() {
        assert_eq!(AppState::format_number(1234567890), "1,234,567,890");
    }

    #[test]
    fn test_format_number_max() {
        assert_eq!(
            AppState::format_number(usize::MAX),
            "18,446,744,073,709,551,615"
        );
    }

    #[test]
    fn test_format_duration_subsecond() {
        let d = Duration::from_millis(500);
        assert_eq!(AppState::format_duration(d), "0.500s");
    }

    #[test]
    fn test_format_duration_seconds() {
        let d = Duration::from_secs(45);
        assert_eq!(AppState::format_duration(d), "45.00s");
    }

    #[test]
    fn test_format_duration_minutes() {
        let d = Duration::from_secs(120);
        assert_eq!(AppState::format_duration(d), "2.00m");
    }

    #[test]
    fn test_format_duration_zero() {
        assert_eq!(AppState::format_duration(Duration::ZERO), "0.000s");
    }

    #[test]
    fn test_tab_all() {
        let tabs = Tab::all();
        assert_eq!(tabs.len(), 3);
        assert_eq!(tabs[0], Tab::Generate);
        assert_eq!(tabs[1], Tab::Primes);
        assert_eq!(tabs[2], Tab::Stats);
    }

    #[test]
    fn test_tab_labels() {
        assert_eq!(Tab::Generate.label(), " Generate ");
        assert_eq!(Tab::Primes.label(), " Primes ");
        assert_eq!(Tab::Stats.label(), " Stats ");
    }

    #[test]
    fn test_input_field_label() {
        assert_eq!(InputField::N.label(), "n");
        assert_eq!(InputField::Workers.label(), "workers");
        assert_eq!(InputField::SegmentSize.label(), "segment");
    }

    #[test]
    fn test_active_input() {
        assert!(matches!(ActiveInput::None, ActiveInput::None));
        assert!(!matches!(
            ActiveInput::Field(InputField::N),
            ActiveInput::None
        ));

        let mut state = AppState::new();
        state.active_input = ActiveInput::Field(InputField::Workers);
    }

    #[test]
    fn test_app_state_new() {
        let app = AppState::new();
        assert_eq!(app.n_input, "100000");
        assert_eq!(app.segment_size_input, "1000000");
        assert!(!app.parallel);
        assert!(app.show_progress);
        assert_eq!(app.active_tab, Tab::Generate);
        assert_eq!(app.generation_state, GenerationState::Idle);
        assert!(app.primes.is_empty());
    }

    #[test]
    fn test_n_value_valid() {
        let mut app = AppState::new();
        assert!(app.n_value().is_ok());

        app.n_input = "100".to_string();
        assert_eq!(app.n_value().unwrap(), 100);

        app.n_input = "999999".to_string();
        assert_eq!(app.n_value().unwrap(), 999999);
    }

    #[test]
    fn test_n_value_invalid() {
        let mut app = AppState::new();
        app.n_input = "abc".to_string();
        assert!(app.n_value().is_err());

        app.n_input = "".to_string();
        assert!(app.n_value().is_err());
    }

    #[test]
    fn test_segment_size_value_valid() {
        let mut app = AppState::new();
        assert!(app.segment_size_value().is_ok());

        app.segment_size_input = "500000".to_string();
        assert_eq!(app.segment_size_value().unwrap(), 500000);
    }

    #[test]
    fn test_segment_size_value_invalid() {
        let mut app = AppState::new();
        app.segment_size_input = "abc".to_string();
        assert!(app.segment_size_value().is_err());

        app.segment_size_input = "".to_string();
        assert!(app.segment_size_value().is_err());
    }

    #[test]
    fn test_reset() {
        let mut app = AppState::new();
        app.primes = vec![2, 3, 5];
        app.last_prime = Some(5);

        app.reset();

        assert!(app.primes.is_empty());
        assert_eq!(app.elapsed, Duration::ZERO);
        assert_eq!(app.last_prime, None);
        assert_eq!(app.generation_state, GenerationState::Idle);
    }

    #[test]
    fn test_update_primes() {
        let mut app = AppState::new();
        let start = Instant::now();

        app.start_time = start;
        app.update_primes(vec![2, 3, 5]);

        assert_eq!(app.primes, vec![2, 3, 5]);
        assert_eq!(app.last_prime, Some(5));
    }

    #[test]
    fn test_finish_generation_ok() {
        let mut app = AppState::new();

        app.finish_generation(Ok(vec![2, 3, 5]));
        assert_eq!(app.generation_state, GenerationState::Complete);
        assert!(app.error_message.is_none());
    }

    #[test]
    fn test_finish_generation_err() {
        let mut app = AppState::new();

        app.finish_generation(Err("something went wrong".to_string()));
        assert_eq!(app.generation_state, GenerationState::Error);
        assert_eq!(app.error_message.as_deref(), Some("something went wrong"));
    }

    #[test]
    fn test_prime_rate() {
        let mut app = AppState::new();
        app.primes = vec![2, 3, 5];
        app.elapsed = Duration::from_secs(1);

        let rate = app.prime_rate();
        assert!((rate - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_prime_rate_zero_seconds() {
        let mut app = AppState::new();
        app.primes = vec![2, 3, 5];

        let rate = app.prime_rate();
        assert_eq!(rate, 0.0);
    }

    #[test]
    fn test_filtered_primes_empty_search() {
        let mut app = AppState::new();
        app.primes = vec![2, 3, 5, 7, 11, 13];

        let filtered = app.filtered_primes();
        assert_eq!(filtered, vec![2, 3, 5, 7, 11, 13]);
    }

    #[test]
    fn test_filtered_primes_with_search() {
        let mut app = AppState::new();
        app.primes = vec![2, 3, 5, 7, 11, 13, 17, 19, 23];
        app.prime_search = "1".to_string();

        let filtered = app.filtered_primes();
        assert!(filtered.contains(&11));
        assert!(filtered.contains(&13));
        assert!(filtered.contains(&17));
    }

    #[test]
    fn test_algorithm_name_classic() {
        let mut app = AppState::new();
        app.n_input = "100".to_string();

        let name = app.algorithm_name();
        assert_eq!(name, "Classic Sieve of Eratosthenes");
    }

    #[test]
    fn test_algorithm_name_segmented() {
        let mut app = AppState::new();
        app.n_input = "2000000".to_string();

        let name = app.algorithm_name();
        assert_eq!(name, "Segmented Sieve");
    }

    #[test]
    fn test_algorithm_name_parallel() {
        let mut app = AppState::new();
        app.n_input = "10000000".to_string();
        app.parallel = true;

        let name = app.algorithm_name();
        assert_eq!(name, "Parallel Segmented Sieve");
    }

    #[test]
    fn test_progress_running() {
        let mut app = AppState::new();
        app.n_input = "100".to_string();
        app.segment_size_input = "50".to_string();
        app.generation_state = GenerationState::Running;

        let progress = app.progress();
        assert!(progress > 0.0);
        assert!(progress < 1.0);
    }

    #[test]
    fn test_progress_complete() {
        let mut app = AppState::new();
        app.n_input = "100".to_string();
        app.generation_state = GenerationState::Complete;

        let progress = app.progress();
        assert_eq!(progress, 1.0);
    }

    #[test]
    fn test_prime_density() {
        let mut app = AppState::new();
        app.n_input = "10".to_string();
        app.primes = vec![2, 3, 5, 7];

        let density = app.prime_density();
        assert!((density - 40.0).abs() < 0.01);
    }

    #[test]
    fn test_prime_gap_stats() {
        let mut app = AppState::new();
        app.primes = vec![2, 3, 5, 7, 11, 13];

        let (min_gap, max_gap, avg_gap) = app.prime_gap_stats();
        assert_eq!(min_gap, Some(1)); // 3-2=1, 5-3=2, etc min is 2
        assert!(max_gap.is_some());
        assert!(avg_gap.is_some());
    }

    #[test]
    fn test_prime_gap_stats_insufficient() {
        let mut app = AppState::new();
        app.primes = vec![2];

        let (min_gap, max_gap, avg_gap) = app.prime_gap_stats();
        assert_eq!(min_gap, None);
        assert_eq!(max_gap, None);
        assert_eq!(avg_gap, None);
    }

    #[test]
    fn test_twin_primes_count() {
        let mut app = AppState::new();
        // (3,5), (5,7), (11,13) are twin primes
        app.primes = vec![2, 3, 5, 7, 11, 13, 17, 19];

        assert_eq!(app.twin_primes_count(), 4);
    }

    #[test]
    fn test_twin_primes_count_none() {
        let mut app = AppState::new();
        app.primes = vec![2];

        assert_eq!(app.twin_primes_count(), 0);
    }

    #[test]
    fn test_generation_state_clone() {
        let state = GenerationState::Running;
        let cloned = state.clone();
        assert_eq!(state, cloned);
    }

    #[test]
    fn test_tab_clone() {
        let tab = Tab::Stats;
        assert_eq!(tab, tab.clone());
    }

    #[test]
    fn test_input_field_clone() {
        let field = InputField::SegmentSize;
        assert_eq!(field, field.clone());
    }

    #[test]
    fn test_active_input_clone() {
        let input = ActiveInput::Field(InputField::N);
        assert_eq!(input, input.clone());
    }

    #[test]
    fn test_format_rate_edge_cases() {
        assert_eq!(AppState::format_rate(1.0), "1.00/s");
        assert_eq!(AppState::format_rate(999.0), "999.00/s");
        assert_eq!(AppState::format_rate(1000.0), "1.00K/s");
        assert_eq!(AppState::format_rate(999_000.0), "999.00K/s");
        assert_eq!(AppState::format_rate(1_000_000.0), "1.00M/s");
    }

    #[test]
    fn test_format_number_edge_cases() {
        assert_eq!(AppState::format_number(0), "0");
        assert_eq!(AppState::format_number(100), "100");
        assert_eq!(AppState::format_number(999), "999");
        assert_eq!(AppState::format_number(1000), "1,000");
        assert_eq!(AppState::format_number(12345), "12,345");
    }

    #[test]
    fn test_format_duration_edge_cases() {
        assert_eq!(
            AppState::format_duration(Duration::from_millis(1)),
            "0.001s"
        );
        assert_eq!(
            AppState::format_duration(Duration::from_millis(999)),
            "0.999s"
        );
        assert_eq!(AppState::format_duration(Duration::from_secs(59)), "59.00s");
        assert_eq!(AppState::format_duration(Duration::from_secs(60)), "1.00m");
        assert_eq!(
            AppState::format_duration(Duration::from_secs(3600)),
            "60.00m"
        );
    }

    #[test]
    fn test_app_state_from_args_no_args() {
        // Can't easily change env::args in tests, so just verify the function
        // returns a valid AppState with defaults when args are present
        let app = AppState::new();
        assert_eq!(app.n_input, "100000");
    }

    #[test]
    fn test_prime_search_editing() {
        let mut app = AppState::new();
        assert!(app.prime_search.is_empty());

        app.prime_search.push('1');
        assert_eq!(app.prime_search, "1");

        app.prime_search.push('2');
        assert_eq!(app.prime_search, "12");

        app.prime_search.pop();
        assert_eq!(app.prime_search, "1");
    }

    #[test]
    fn test_effective_workers_auto() {
        let mut app = AppState::new();
        app.workers = 0;

        let workers = app.effective_workers();
        assert!(workers > 0);
    }

    #[test]
    fn test_effective_workers_manual() {
        let mut app = AppState::new();
        app.workers = 8;

        assert_eq!(app.effective_workers(), 8);
    }
}
