package prime

import (
	"fmt"
	"runtime"
	"testing"
)

func BenchmarkSieveOfEratosthenes(b *testing.B) {
	testSizes := []int{1000, 10000, 100000, 1000000}

	for _, n := range testSizes {
		b.Run(fmt.Sprintf("n=%d", n), func(b *testing.B) {
			b.ReportAllocs()
			b.SetBytes(int64(n))
			for i := 0; i < b.N; i++ {
				SieveOfEratosthenes(n)
			}
		})
	}
}

func BenchmarkSegmentedSieve(b *testing.B) {
	testSizes := []int{1000000, 5000000, 10000000, 50000000}
	segmentSizes := []int{100000, 1000000, 10000000}

	for _, n := range testSizes {
		for _, segmentSize := range segmentSizes {
			if segmentSize <= n {
				b.Run(fmt.Sprintf("n=%d/segment=%d", n, segmentSize), func(b *testing.B) {
					b.ReportAllocs()
					b.SetBytes(int64(n))
					for i := 0; i < b.N; i++ {
						SegmentedSieve(n, segmentSize, nil)
					}
				})
			}
		}
	}
}

func BenchmarkParallelSegmentedSieve(b *testing.B) {
	testSizes := []int{10000000, 50000000, 100000000, 500000000}
	workerCounts := []int{2, 4, 8}

	for _, n := range testSizes {
		for _, workers := range workerCounts {
			b.Run(fmt.Sprintf("n=%d/workers=%d", n, workers), func(b *testing.B) {
				b.ReportAllocs()
				b.SetBytes(int64(n))
				for i := 0; i < b.N; i++ {
					ParallelSegmentedSieve(n, workers, DefaultSegmentSize, nil)
				}
			})
		}
	}
}

func BenchmarkGeneratePrimes(b *testing.B) {
	testSizes := []int{1000, 10000, 100000, 1000000, 10000000, 100000000}

	for _, n := range testSizes {
		b.Run(fmt.Sprintf("n=%d", n), func(b *testing.B) {
			b.ReportAllocs()
			b.SetBytes(int64(n))
			for i := 0; i < b.N; i++ {
				GeneratePrimes(n, false, nil)
			}
		})
	}
}

func BenchmarkGeneratePrimesParallel(b *testing.B) {
	testSizes := []int{100000000, 500000000, 1000000000}

	for _, n := range testSizes {
		b.Run(fmt.Sprintf("n=%d", n), func(b *testing.B) {
			b.ReportAllocs()
			b.SetBytes(int64(n))
			for i := 0; i < b.N; i++ {
				GeneratePrimes(n, true, nil)
			}
		})
	}
}

func BenchmarkCompareAlgorithms(b *testing.B) {
	n := 1000000

	b.Run("Classic", func(b *testing.B) {
		b.ReportAllocs()
		for i := 0; i < b.N; i++ {
			SieveOfEratosthenes(n)
		}
	})

	b.Run("Segmented", func(b *testing.B) {
		b.ReportAllocs()
		for i := 0; i < b.N; i++ {
			SegmentedSieve(n, DefaultSegmentSize, nil)
		}
	})

	b.Run("GeneratePrimes", func(b *testing.B) {
		b.ReportAllocs()
		for i := 0; i < b.N; i++ {
			GeneratePrimes(n, false, nil)
		}
	})
}

func BenchmarkCompareParallelism(b *testing.B) {
	n := 100000000

	b.Run("Sequential", func(b *testing.B) {
		b.ReportAllocs()
		for i := 0; i < b.N; i++ {
			SegmentedSieve(n, DefaultSegmentSize, nil)
		}
	})

	b.Run("Parallel-2", func(b *testing.B) {
		b.ReportAllocs()
		for i := 0; i < b.N; i++ {
			ParallelSegmentedSieve(n, 2, DefaultSegmentSize, nil)
		}
	})

	b.Run("Parallel-4", func(b *testing.B) {
		b.ReportAllocs()
		for i := 0; i < b.N; i++ {
			ParallelSegmentedSieve(n, 4, DefaultSegmentSize, nil)
		}
	})

	b.Run("Parallel-NumCPU", func(b *testing.B) {
		b.ReportAllocs()
		workers := runtime.NumCPU()
		for i := 0; i < b.N; i++ {
			ParallelSegmentedSieve(n, workers, DefaultSegmentSize, nil)
		}
	})
}
