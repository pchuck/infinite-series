package prime

import (
	"bytes"
	"errors"
	"fmt"
	"math"
	"runtime"
	"sync"
	"sync/atomic"
)

const (
	DefaultSegmentSize = 1_000_000
	ParallelThreshold  = 5_000_000

	// MaxN is the maximum supported input size (1 quadrillion)
	MaxN = 1_000_000_000_000
)

// ValidateError represents input validation errors
type ValidateError struct {
	msg string
}

func (e *ValidateError) Error() string {
	return e.msg
}

func (e *ValidateError) String() string {
	return e.msg
}

// NewValidateError creates a new ValidateError with the given message.
func NewValidateError(msg string) error {
	return &ValidateError{msg: msg}
}

func isValidateError(err error) bool {
	var ve *ValidateError
	return errors.As(err, &ve)
}

// validateN validates that n does not exceed the maximum supported input size.
func validateN(n int) error {
	if n < 0 {
		return NewValidateError(fmt.Sprintf("n=%d is invalid: must be >= 0", n))
	}
	if n > MaxN {
		return NewValidateError(fmt.Sprintf("n=%d exceeds maximum supported value %d (1 quadrillion). Generating primes above this limit would require impractical computation time.", n, MaxN))
	}
	return nil
}

// validateWorkers validates the workers parameter.
func validateWorkers(workers int) error {
	if workers < 0 {
		return NewValidateError(fmt.Sprintf("workers=%d is invalid: must be >= 0", workers))
	}
	if workers > 1024 {
		return NewValidateError(fmt.Sprintf("workers=%d exceeds maximum allowed value of 1024", workers))
	}
	return nil
}

// validateSegmentSize validates that segment_size is non-zero and reasonable.
func validateSegmentSize(segmentSize int) error {
	if segmentSize == 0 {
		return NewValidateError("segment_size cannot be zero")
	}
	if segmentSize < 0 {
		return NewValidateError(fmt.Sprintf("segment_size=%d is invalid: must be >= 0", segmentSize))
	}
	return nil
}

// sieveSegmentOddOnly processes a single segment using odd-only sieve.
// Shared helper used by both sequential and parallel segmented sieves.
//
// low/high define the segment range [low, high).
// basePrimes are odd primes up to sqrt(n) (excludes 2).
// isPrime is a reusable buffer (at least (high-low)/2 bytes).
// Returns primes found in [max(low,2), high).
func sieveSegmentOddOnly(low, high int, basePrimes []int, isPrime []byte) []int {
	var primes []int

	// Handle the prime 2 if it falls in this segment
	if low <= 2 && high > 2 {
		primes = append(primes, 2)
	}

	// Odd-only sieve: index i represents number oddLow + 2*i
	oddLow := low
	if oddLow < 3 {
		oddLow = 3
	}
	if oddLow%2 == 0 {
		oddLow++
	}
	if oddLow >= high {
		return primes
	}

	segLen := (high - oddLow + 1) / 2 // count of odd numbers in [oddLow, high)
	if segLen <= 0 {
		return primes
	}

	for i := 0; i < segLen; i++ {
		isPrime[i] = 1
	}

	for _, p := range basePrimes {
		// Find first odd multiple of p in [oddLow, high)
		start := ((low + p - 1) / p) * p
		if start < p*p {
			start = p * p
		}
		if start%2 == 0 {
			start += p
		}

		if start >= high {
			continue
		}

		// Map to index in odd-only array
		adjustedStart := (start - oddLow) / 2
		step := p // step in index space = p (each index step = 2 numbers)
		for j := adjustedStart; j < segLen; j += step {
			isPrime[j] = 0
		}
	}

	// Extract primes using bytes.IndexByte for SIMD-optimized search
	data := isPrime[:segLen]
	idx := 0
	for {
		pos := bytes.IndexByte(data[idx:], 1)
		if pos == -1 {
			break
		}
		idx += pos
		primes = append(primes, oddLow+2*idx)
		idx++
		if idx >= segLen {
			break
		}
	}

	return primes
}

func SieveOfEratosthenes(n int) ([]int, error) {
	if err := validateN(n); err != nil {
		return nil, err
	}

	if n <= 2 {
		return []int{}, nil
	}

	if n <= 3 {
		return []int{2}, nil
	}

	// Odd-only sieve: index i represents number 2*i + 3
	sieveSize := (n - 3 + 1) / 2 // count of odd numbers in [3, n)
	sieve := make([]byte, sieveSize)
	for i := 0; i < sieveSize; i++ {
		sieve[i] = 1
	}

	limit := int(math.Sqrt(float64(n)))
	for current := 3; current <= limit; current += 2 {
		idx := (current - 3) / 2
		if sieve[idx] == 1 {
			// Mark multiples of current starting at current*current
			startIdx := (current*current - 3) / 2
			step := current // step in index space
			for j := startIdx; j < sieveSize; j += step {
				sieve[j] = 0
			}
		}
	}

	// Pre-allocate with prime counting estimate
	estimated := int(float64(n) / math.Log(float64(n)) * 1.1)
	primes := make([]int, 0, estimated)
	primes = append(primes, 2)

	// Extract primes using bytes.IndexByte for SIMD-optimized search
	idx := 0
	for {
		pos := bytes.IndexByte(sieve[idx:], 1)
		if pos == -1 {
			break
		}
		idx += pos
		primes = append(primes, 2*idx+3)
		idx++
		if idx >= sieveSize {
			break
		}
	}

	return primes, nil
}

// PrimeGenError is returned when prime generation fails.
type PrimeGenError struct {
	msg string
}

func (e *PrimeGenError) Error() string {
	return e.msg
}

// WorkerThreadPanic creates an error for worker thread panics.
func WorkerThreadPanic(msg string) error {
	return &PrimeGenError{msg: fmt.Sprintf("worker thread panicked: %s", msg)}
}

// InvalidInput creates an error for invalid input parameters.
func InvalidInput(msg string) error {
	return &PrimeGenError{msg: fmt.Sprintf("invalid input: %s", msg)}
}

// IsValidateError returns true if the error is a validation error.
func IsValidateError(err error) bool {
	return isValidateError(err)
}

func SegmentedSieve(n int, segmentSize int, progress func(int)) ([]int, error) {
	if err := validateN(n); err != nil {
		return nil, err
	}
	if err := validateSegmentSize(segmentSize); err != nil {
		return nil, err
	}

	if n <= 2 {
		return []int{}, nil
	}
	if segmentSize == 0 {
		segmentSize = DefaultSegmentSize
	}

	baseLimit := int(math.Sqrt(float64(n)))
	allBasePrimes, err := SieveOfEratosthenes(baseLimit + 1)
	if err != nil {
		return nil, err
	}
	basePrimesOdd := allBasePrimes[1:]

	segments := (n + segmentSize - 1) / segmentSize
	estimated := int(float64(n) / math.Log(float64(n)) * 1.1)
	primes := make([]int, 0, estimated)

	// Reusable buffer for segments -- allocate once to max segment size
	isPrime := make([]byte, segmentSize)

	for segIdx := 0; segIdx < segments; segIdx++ {
		low := segIdx * segmentSize
		high := low + segmentSize
		if high > n {
			high = n
		}

		if high <= 2 {
			if progress != nil {
				progress(1)
			}
			continue
		}

		segPrimes := sieveSegmentOddOnly(low, high, basePrimesOdd, isPrime)
		primes = append(primes, segPrimes...)

		if progress != nil {
			progress(1)
		}
	}

	return primes, nil
}

type segmentWork struct {
	segIdx int
	low    int
	high   int
}

type segmentResult struct {
	segIdx int
	primes []int
}

func workerProcessSegment(
	workChan <-chan segmentWork,
	resultsChan chan<- segmentResult,
	basePrimes []int,
	segmentSize int,
	wg *sync.WaitGroup,
	completedSegments *int64,
) {
	defer wg.Done()
	// Each worker gets its own reusable buffer
	isPrime := make([]byte, segmentSize)

	for work := range workChan {
		if work.high <= 2 {
			atomic.AddInt64(completedSegments, 1)
			resultsChan <- segmentResult{segIdx: work.segIdx, primes: nil}
			continue
		}

		segPrimes := sieveSegmentOddOnly(work.low, work.high, basePrimes, isPrime)
		atomic.AddInt64(completedSegments, 1)

		resultsChan <- segmentResult{
			segIdx: work.segIdx,
			primes: segPrimes,
		}
	}
}

func ParallelSegmentedSieve(n int, workers, segmentSize int, progress func(int)) ([]int, error) {
	if err := validateN(n); err != nil {
		return nil, err
	}
	if segmentSize == 0 {
		segmentSize = DefaultSegmentSize
	} else if err := validateSegmentSize(segmentSize); err != nil {
		return nil, err
	}
	if err := validateWorkers(workers); err != nil {
		return nil, err
	}

	if n <= 2 {
		return []int{}, nil
	}
	if workers <= 0 {
		workers = runtime.NumCPU()
	}

	baseLimit := int(math.Sqrt(float64(n)))
	allBasePrimes, err := SieveOfEratosthenes(baseLimit + 1)
	if err != nil {
		return nil, err
	}
	basePrimesOdd := allBasePrimes[1:]

	segments := (n + segmentSize - 1) / segmentSize
	numWorkers := workers
	if numWorkers > segments {
		numWorkers = segments
	}

	// Bounded channels to limit memory usage
	workChan := make(chan segmentWork, numWorkers*2)
	resultsChan := make(chan segmentResult, numWorkers*2)
	var wg sync.WaitGroup
	var completedSegments int64

	// Start workers
	for i := 0; i < numWorkers; i++ {
		wg.Add(1)
		go workerProcessSegment(workChan, resultsChan, basePrimesOdd, segmentSize, &wg, &completedSegments)
	}

	// Producer: enqueue all segments
	go func() {
		for segIdx := 0; segIdx < segments; segIdx++ {
			low := segIdx * segmentSize
			high := low + segmentSize
			if high > n {
				high = n
			}

			workChan <- segmentWork{
				segIdx: segIdx,
				low:    low,
				high:   high,
			}
		}
		close(workChan)
	}()

	// Closer: wait for all workers then close results
	go func() {
		wg.Wait()
		close(resultsChan)
	}()

	// Progress monitor goroutine
	var progressDone chan struct{}
	if progress != nil {
		progressDone = make(chan struct{})
		go func() {
			defer close(progressDone)
			var lastSeen int64
			for {
				current := atomic.LoadInt64(&completedSegments)
				if current > lastSeen {
					delta := int(current - lastSeen)
					progress(delta)
					lastSeen = current
				}
				if current >= int64(segments) {
					return
				}
				runtime.Gosched()
			}
		}()
	}

	// Collect results indexed by segment for ordered reassembly
	results := make([][]int, segments)
	totalPrimes := 0
	for result := range resultsChan {
		results[result.segIdx] = result.primes
		totalPrimes += len(result.primes)
	}

	// Wait for progress monitor to finish
	if progressDone != nil {
		<-progressDone
	}

	// Flatten in segment order -- single allocation
	allPrimes := make([]int, 0, totalPrimes)
	for _, segPrimes := range results {
		allPrimes = append(allPrimes, segPrimes...)
	}

	return allPrimes, nil
}

func GeneratePrimes(n int, parallel bool, progress func(int)) ([]int, error) {
	if err := validateN(n); err != nil {
		return nil, err
	}

	if n <= 2 {
		return []int{}, nil
	}

	if parallel && n >= ParallelThreshold {
		return ParallelSegmentedSieve(n, 0, DefaultSegmentSize, progress)
	}

	if n >= DefaultSegmentSize {
		return SegmentedSieve(n, DefaultSegmentSize, progress)
	}

	return SieveOfEratosthenes(n)
}

// ProgressTracker provides thread-safe progress tracking using atomics.
type ProgressTracker struct {
	total     int64
	completed int64
}

func NewProgressTracker(total int64) *ProgressTracker {
	return &ProgressTracker{total: total}
}

func (p *ProgressTracker) AddCompleted(delta int64) {
	atomic.AddInt64(&p.completed, delta)
}

func (p *ProgressTracker) GetCompleted() int64 {
	return atomic.LoadInt64(&p.completed)
}

func (p *ProgressTracker) GetPercent() int {
	if p.total == 0 {
		return 100
	}
	return int(float64(atomic.LoadInt64(&p.completed)) / float64(p.total) * 100)
}
