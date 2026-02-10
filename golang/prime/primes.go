package prime

import (
	"math"
	"runtime"
	"sort"
	"sync"
	"sync/atomic"
)

const (
	DefaultSegmentSize = 1_000_000
	ParallelThreshold  = 100_000_000
)

func SieveOfEratosthenes(n int) []int {
	if n <= 2 {
		return nil
	}

	sieve := make([]byte, n)
	for i := 2; i < n; i++ {
		sieve[i] = 1
	}

	limit := int(math.Sqrt(float64(n)))
	for i := 2; i <= limit; i++ {
		if sieve[i] == 1 {
			start := i * i
			step := i
			for j := start; j < n; j += step {
				sieve[j] = 0
			}
		}
	}

	primes := make([]int, 0, n/int(math.Log(float64(n))))
	for i := 2; i < n; i++ {
		if sieve[i] == 1 {
			primes = append(primes, i)
		}
	}

	return primes
}

func SegmentedSieve(n int, segmentSize int, progress func(int)) []int {
	if n <= 2 {
		return nil
	}
	if segmentSize <= 0 {
		segmentSize = DefaultSegmentSize
	}

	baseLimit := int(math.Sqrt(float64(n)))
	basePrimes := SieveOfEratosthenes(baseLimit + 1)

	segments := (n + segmentSize - 1) / segmentSize
	primes := make([]int, 0, n/int(math.Log(float64(n))))

	for segIdx := 0; segIdx < segments; segIdx++ {
		low := segIdx * segmentSize
		high := low + segmentSize
		if high > n {
			high = n
		}

		if high <= 2 {
			continue
		}

		segmentLow := low
		if segmentLow < 2 {
			segmentLow = 2
		}
		segLen := high - segmentLow
		isPrime := make([]byte, segLen)
		for i := 0; i < segLen; i++ {
			isPrime[i] = 1
		}

		for _, p := range basePrimes {
			start := ((low + p - 1) / p) * p
			if start < p*p {
				start = p * p
			}

			adjustedStart := start - segmentLow
			if adjustedStart >= segLen {
				continue
			}

			step := p
			for j := adjustedStart; j < segLen; j += step {
				isPrime[j] = 0
			}
		}

		for i := 0; i < segLen; i++ {
			if isPrime[i] == 1 {
				primes = append(primes, segmentLow+i)
			}
		}

		if progress != nil {
			progress(segIdx + 1)
		}
	}

	return primes
}

type segmentWork struct {
	segIdx     int
	low        int
	high       int
	segmentLow int
	segLen     int
}

type segmentResult struct {
	segIdx int
	primes []int
}

func workerProcessSegment(
	workChan <-chan segmentWork,
	resultsChan chan<- segmentResult,
	basePrimes []int,
	wg *sync.WaitGroup,
) {
	defer wg.Done()
	for work := range workChan {
		isPrime := make([]byte, work.segLen)
		for i := 0; i < work.segLen; i++ {
			isPrime[i] = 1
		}

		for _, p := range basePrimes {
			start := ((work.low + p - 1) / p) * p
			if start < p*p {
				start = p * p
			}

			adjustedStart := start - work.segmentLow
			if adjustedStart >= work.segLen {
				continue
			}

			step := p
			for j := adjustedStart; j < work.segLen; j += step {
				isPrime[j] = 0
			}
		}

		primes := make([]int, 0, work.segLen/10)
		for i := 0; i < work.segLen; i++ {
			if isPrime[i] == 1 {
				primes = append(primes, work.segmentLow+i)
			}
		}

		resultsChan <- segmentResult{
			segIdx: work.segIdx,
			primes: primes,
		}
	}
}

func ParallelSegmentedSieve(n int, workers, segmentSize int, progress func(int)) []int {
	if n <= 2 {
		return nil
	}
	if segmentSize <= 0 {
		segmentSize = DefaultSegmentSize
	}
	if workers <= 0 {
		workers = runtime.NumCPU()
	}

	baseLimit := int(math.Sqrt(float64(n)))
	basePrimes := SieveOfEratosthenes(baseLimit + 1)

	segments := (n + segmentSize - 1) / segmentSize
	numWorkers := workers
	if numWorkers > segments {
		numWorkers = segments
	}

	workChan := make(chan segmentWork, segments)
	resultsChan := make(chan segmentResult, segments)
	var wg sync.WaitGroup

	for i := 0; i < numWorkers; i++ {
		wg.Add(1)
		go workerProcessSegment(workChan, resultsChan, basePrimes, &wg)
	}

	go func() {
		for segIdx := 0; segIdx < segments; segIdx++ {
			low := segIdx * segmentSize
			high := low + segmentSize
			if high > n {
				high = n
			}

			if high <= 2 {
				if progress != nil {
					progress(segIdx + 1)
				}
				continue
			}

			segmentLow := low
			if segmentLow < 2 {
				segmentLow = 2
			}
			segLen := high - segmentLow

			workChan <- segmentWork{
				segIdx:     segIdx,
				low:        low,
				high:       high,
				segmentLow: segmentLow,
				segLen:     segLen,
			}
		}
		close(workChan)
	}()

	go func() {
		wg.Wait()
		close(resultsChan)
	}()

	results := make([]segmentResult, 0, segments)
	for result := range resultsChan {
		results = append(results, result)
	}

	allPrimes := make([]int, 0, n/int(math.Log(float64(n))))
	
	sort.Slice(results, func(i, j int) bool {
		return results[i].segIdx < results[j].segIdx
	})
	
	for _, result := range results {
		allPrimes = append(allPrimes, result.primes...)
	}

	return allPrimes
}

func GeneratePrimes(n int, parallel bool, progress func(int)) []int {
	if n <= 2 {
		return nil
	}

	if parallel && n >= ParallelThreshold {
		return ParallelSegmentedSieve(n, 0, DefaultSegmentSize, progress)
	}

	if n >= DefaultSegmentSize {
		return SegmentedSieve(n, DefaultSegmentSize, progress)
	}

	return SieveOfEratosthenes(n)
}

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
	return int(float64(p.completed) / float64(p.total) * 100)
}
