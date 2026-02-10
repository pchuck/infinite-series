package progress

import (
	"fmt"
	"os"
	"runtime"
	"strings"
	"sync"
	"time"
)

type ProgressBar struct {
	total       int64
	completed   int64
	width       int
	startTime   time.Time
	description string
	mu          sync.Mutex
	started     bool
}

func NewProgressBar(total int64, description string) *ProgressBar {
	return &ProgressBar{
		total:       total,
		width:       40,
		description: description,
		startTime:   time.Now(),
	}
}

func (p *ProgressBar) Update(delta int64) {
	p.mu.Lock()
	p.completed += delta
	p.render()
	p.mu.Unlock()
}

func (p *ProgressBar) SetTotal(total int64) {
	p.mu.Lock()
	p.total = total
	p.mu.Unlock()
}

func (p *ProgressBar) SetDescription(desc string) {
	p.mu.Lock()
	p.description = desc
	p.mu.Unlock()
}

func (p *ProgressBar) SetCompleted(completed int64) {
	p.mu.Lock()
	p.completed = completed
	p.render()
	p.mu.Unlock()
}

func (p *ProgressBar) Finish() {
	p.mu.Lock()
	if !p.started {
		p.started = true
		p.startTime = time.Now()
	}
	p.completed = p.total
	p.render()
	fmt.Fprintln(os.Stderr)
	p.mu.Unlock()
}

func (p *ProgressBar) GetCompleted() int64 {
	p.mu.Lock()
	defer p.mu.Unlock()
	return p.completed
}

func (p *ProgressBar) render() {
	if p.total == 0 {
		return
	}

	percent := float64(p.completed) / float64(p.total)
	if percent > 1.0 {
		percent = 1.0
	}

	filled := int(percent * float64(p.width))

	elapsed := time.Since(p.startTime)
	_ = elapsed

	fmt.Fprintf(os.Stderr, "\r%s: [%s%s] %3.0f%% | %d/%d",
		p.description,
		strings.Repeat("=", filled),
		strings.Repeat(" ", p.width-filled),
		percent*100,
		p.completed,
		p.total)
}

type MultiProgress struct {
	bars     []*ProgressBar
	mu       sync.Mutex
	active   int
	maxDescr int
}

func NewMultiProgress() *MultiProgress {
	return &MultiProgress{
		maxDescr: 20,
	}
}

func (m *MultiProgress) AddBar(total int64, description string) *ProgressBar {
	m.mu.Lock()
	defer m.mu.Unlock()

	if len(description) > m.maxDescr {
		description = description[:m.maxDescr-3] + "..."
	} else if len(description) < m.maxDescr {
		description = strings.Repeat(" ", m.maxDescr-len(description)) + description
	}

	bar := &ProgressBar{
		total:       total,
		width:       30,
		description: description,
		startTime:   time.Now(),
		started:     true,
	}
	m.bars = append(m.bars, bar)
	m.active++
	return bar
}

func (m *MultiProgress) RemoveBar(bar *ProgressBar) {
	m.mu.Lock()
	defer m.mu.Unlock()

	for i, b := range m.bars {
		if b == bar {
			m.bars = append(m.bars[:i], m.bars[i+1:]...)
			m.active--
			break
		}
	}
}

func (m *MultiProgress) RenderAll() {
	m.mu.Lock()
	defer m.mu.Unlock()

	for _, bar := range m.bars {
		bar.render()
		fmt.Fprintln(os.Stderr)
	}
}

func GetCPUCount() int {
	return runtime.NumCPU()
}

func FormatNumber(n int64) string {
	if n >= 1_000_000_000 {
		return fmt.Sprintf("%.2fB", float64(n)/1_000_000_000)
	} else if n >= 1_000_000 {
		return fmt.Sprintf("%.2fM", float64(n)/1_000_000)
	} else if n >= 1_000 {
		return fmt.Sprintf("%.2fK", float64(n)/1_000)
	}
	return fmt.Sprintf("%d", n)
}
