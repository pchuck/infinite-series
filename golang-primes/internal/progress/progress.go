package progress

import (
	"fmt"
	"os"
	"runtime"
	"strings"
	"sync"
	"time"
)

// ProgressBar provides a simple terminal progress bar that writes to stderr.
type ProgressBar struct {
	total       int64
	completed   int64
	width       int
	startTime   time.Time
	description string
	mu          sync.Mutex
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
	rate := float64(p.completed) / elapsed.Seconds()
	var rateStr string
	if rate >= 1_000_000 {
		rateStr = fmt.Sprintf("%.1fM/s", rate/1_000_000)
	} else if rate >= 1_000 {
		rateStr = fmt.Sprintf("%.1fK/s", rate/1_000)
	} else {
		rateStr = fmt.Sprintf("%.0f/s", rate)
	}

	fmt.Fprintf(os.Stderr, "\r%s: [%s%s] %3.0f%% | %d/%d | %s",
		p.description,
		strings.Repeat("=", filled),
		strings.Repeat(" ", p.width-filled),
		percent*100,
		p.completed,
		p.total,
		rateStr)
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
