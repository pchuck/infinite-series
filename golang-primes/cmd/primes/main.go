package main

import (
	"bufio"
	"flag"
	"fmt"
	"os"
	"runtime"
	"strconv"
	"strings"
	"time"

	"github.com/pchuck/infinite-series/golang/internal/progress"
	"github.com/pchuck/infinite-series/golang/prime"
)

var (
	n           int
	progressBar bool
	parallel    bool
	workers     int
	segment     int
	quiet       bool
)

func init() {
	flag.IntVar(&n, "n", 0, "Upper bound (exclusive) for prime generation")
	flag.BoolVar(&progressBar, "progress", false, "Show progress bar")
	flag.BoolVar(&parallel, "parallel", false, "Use parallel processing (for large n)")
	flag.BoolVar(&quiet, "quiet", false, "Only print count (no prime list)")
	flag.IntVar(&workers, "workers", 0, "Number of worker goroutines (default: NumCPU)")
	flag.IntVar(&segment, "segment", prime.DefaultSegmentSize, "Segment size for segmented sieve")
	flag.Usage = func() {
		fmt.Fprintf(os.Stderr, "Prime Number Generator\n\n")
		fmt.Fprintf(os.Stderr, "Usage: %s [flags] [n]\n\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "Flags:\n")
		flag.PrintDefaults()
		fmt.Fprintf(os.Stderr, "\nExamples:\n")
		fmt.Fprintf(os.Stderr, "  %s 100                       # Generate primes < 100\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "  %s 1000000 --progress        # With progress bar\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "  %s 100000000 --parallel     # Parallel processing\n", os.Args[0])
		fmt.Fprintf(os.Stderr, "  %s 1000000000 --quiet       # Count only, no output\n", os.Args[0])
	}
}

func main() {
	flag.Parse()

	if flag.NArg() > 0 {
		if n == 0 {
			parsed, err := strconv.Atoi(flag.Arg(0))
			if err != nil {
				fmt.Fprintf(os.Stderr, "Error: invalid number %q: %v\n", flag.Arg(0), err)
				os.Exit(1)
			}
			n = parsed
		}
	}

	if n <= 0 {
		fmt.Fprint(os.Stderr, "Enter upper bound (n): ")
		reader := bufio.NewReader(os.Stdin)
		input, _ := reader.ReadString('\n')
		parsed, err := strconv.Atoi(strings.TrimSpace(input))
		if err != nil {
			fmt.Fprintf(os.Stderr, "Error: invalid number %q: %v\n", strings.TrimSpace(input), err)
			os.Exit(1)
		}
		n = parsed
	}

	if n <= 2 {
		fmt.Printf("No primes less than %d\n", n)
		return
	}

	if parallel && n < prime.ParallelThreshold {
		fmt.Fprintf(os.Stderr, "[WARN] --parallel ignored: n=%d is below threshold %d\n", n, prime.ParallelThreshold)
		parallel = false
	}

	workerCount := workers
	if workerCount <= 0 {
		workerCount = runtime.NumCPU()
	}

	segmentSizeForProgress := segment
	if progressBar && segment == prime.DefaultSegmentSize {
		segmentSizeForProgress = 100000
	}

	var progressBarObj *progress.ProgressBar

	if progressBar {
		segments := (n + segmentSizeForProgress - 1) / segmentSizeForProgress
		progressBarObj = progress.NewProgressBar(int64(segments), "Generating primes")
	}

	// Progress callback now receives a delta (number of segments completed since last call)
	var progressCallback func(int)
	if progressBarObj != nil {
		progressCallback = func(delta int) {
			progressBarObj.Update(int64(delta))
		}
	}

	var primes []int
	computeStart := time.Now()

	if parallel {
		primes = prime.ParallelSegmentedSieve(n, workerCount, segmentSizeForProgress, progressCallback)
	} else if n >= prime.DefaultSegmentSize {
		primes = prime.SegmentedSieve(n, segmentSizeForProgress, progressCallback)
	} else {
		primes = prime.SieveOfEratosthenes(n)
	}

	if len(primes) > 0 {
		if !quiet {
			fmt.Printf("Primes less than %d: ", n)
			var sb strings.Builder
			sb.Grow(len(primes) * 8)
			for i, p := range primes {
				if i > 0 {
					sb.WriteString(", ")
				}
				sb.WriteString(strconv.Itoa(p))
			}
			fmt.Println(sb.String())
			fmt.Printf("Total primes: %d\n", len(primes))
		} else {
			fmt.Printf("%d\n", len(primes))
		}
	} else {
		fmt.Printf("No primes less than %d\n", n)
	}

	if progressBarObj != nil {
		progressBarObj.Finish()
		fmt.Fprint(os.Stderr, "\n")
		os.Stderr.Sync()
	}

	totalTime := time.Since(computeStart)
	rate := float64(len(primes)) / totalTime.Seconds()

	if len(primes) > 0 {
		lastPrime := primes[len(primes)-1]
		rateStr := formatRate(rate)
		fmt.Fprintf(os.Stderr, "Done! Largest prime < %d is %d. Generated %d primes in %.3fs (%s primes/s).\n",
			n, lastPrime, len(primes), totalTime.Seconds(), rateStr)
	} else {
		fmt.Fprintf(os.Stderr, "Done! Generated 0 primes in %.3fs (0 primes/s).\n",
			totalTime.Seconds())
	}
}

func formatRate(rate float64) string {
	s := fmt.Sprintf("%.0f", rate)
	n := len(s)
	if n <= 3 {
		return s
	}

	// Insert commas from right to left
	var sb strings.Builder
	sb.Grow(n + n/3)
	offset := n % 3
	if offset == 0 {
		offset = 3
	}
	sb.WriteString(s[:offset])
	for i := offset; i < n; i += 3 {
		sb.WriteByte(',')
		sb.WriteString(s[i : i+3])
	}
	return sb.String()
}
