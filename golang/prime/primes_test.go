package prime

import (
	"fmt"
	"testing"
)

func TestSieveOfEratosthenes(t *testing.T) {
	tests := []struct {
		name     string
		n        int
		expected []int
	}{
		{
			name:     "n=10",
			n:        10,
			expected: []int{2, 3, 5, 7},
		},
		{
			name:     "n=30",
			n:        30,
			expected: []int{2, 3, 5, 7, 11, 13, 17, 19, 23, 29},
		},
		{
			name:     "n=5",
			n:        5,
			expected: []int{2, 3},
		},
		{
			name:     "n=3",
			n:        3,
			expected: []int{2},
		},
		{
			name:     "n=4",
			n:        4,
			expected: []int{2, 3},
		},
		{
			name:     "n=2",
			n:        2,
			expected: nil,
		},
		{
			name:     "n=1",
			n:        1,
			expected: nil,
		},
		{
			name:     "n=0",
			n:        0,
			expected: nil,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := SieveOfEratosthenes(tt.n)
			if len(result) != len(tt.expected) {
				t.Errorf("SieveOfEratosthenes(%d) = %v, want %v", tt.n, result, tt.expected)
				return
			}
			for i, v := range result {
				if v != tt.expected[i] {
					t.Errorf("SieveOfEratosthenes(%d)[%d] = %d, want %d", tt.n, i, v, tt.expected[i])
				}
			}
		})
	}
}

func TestSegmentedSieve(t *testing.T) {
	tests := []struct {
		name     string
		n        int
		expected []int
	}{
		{
			name:     "n=10",
			n:        10,
			expected: []int{2, 3, 5, 7},
		},
		{
			name:     "n=100",
			n:        100,
			expected: []int{2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97},
		},
		{
			name:     "n=2",
			n:        2,
			expected: nil,
		},
		{
			name:     "n=0",
			n:        0,
			expected: nil,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := SegmentedSieve(tt.n, 10, nil)
			if len(result) != len(tt.expected) {
				t.Errorf("SegmentedSieve(%d) = %v, want %v", tt.n, result, tt.expected)
				return
			}
			for i, v := range result {
				if v != tt.expected[i] {
					t.Errorf("SegmentedSieve(%d)[%d] = %d, want %d", tt.n, i, v, tt.expected[i])
				}
			}
		})
	}
}

func TestSegmentedSieveMatchesClassic(t *testing.T) {
	testValues := []int{100, 500, 1000, 5000, 10000}
	for _, n := range testValues {
		t.Run(fmt.Sprintf("n=%d", n), func(t *testing.T) {
			expected := SieveOfEratosthenes(n)
			result := SegmentedSieve(n, 100, nil)
			if len(result) != len(expected) {
				t.Errorf("SegmentedSieve(%d) length = %d, want %d", n, len(result), len(expected))
				return
			}
			for i, v := range result {
				if v != expected[i] {
					t.Errorf("SegmentedSieve(%d)[%d] = %d, want %d", n, i, v, expected[i])
				}
			}
		})
	}
}

func TestSegmentedSieveLargeInput(t *testing.T) {
	n := 1000000
	result := SegmentedSieve(n, DefaultSegmentSize, nil)

	expectedCount := 78498
	if len(result) != expectedCount {
		t.Errorf("SegmentedSieve(%d) count = %d, want %d", n, len(result), expectedCount)
	}

	if len(result) == 0 {
		t.Fatal("SegmentedSieve returned empty result for n=1000000")
	}

	if result[0] != 2 {
		t.Errorf("First prime = %d, want 2", result[0])
	}

	if result[len(result)-1] != 999983 {
		t.Errorf("Last prime = %d, want 999983", result[len(result)-1])
	}
}

func TestSegmentedSieveWithProgress(t *testing.T) {
	n := 100
	segmentSize := 10
	totalDelta := 0
	callback := func(delta int) {
		totalDelta += delta
	}

	result := SegmentedSieve(n, segmentSize, callback)

	if totalDelta == 0 {
		t.Error("Progress callback was not called")
	}

	expected := SieveOfEratosthenes(n)
	if len(result) != len(expected) {
		t.Errorf("SegmentedSieve with callback = %v, want %v", result, expected)
	}
}

func TestSegmentedSieveCustomSegmentSize(t *testing.T) {
	n := 100
	expected := SieveOfEratosthenes(n)

	segmentSizes := []int{1, 10, 100, 1000}
	for _, segSize := range segmentSizes {
		t.Run(fmt.Sprintf("seg=%d", segSize), func(t *testing.T) {
			result := SegmentedSieve(n, segSize, nil)
			if len(result) != len(expected) {
				t.Errorf("SegmentedSieve(%d, %d) count = %d, want %d", n, segSize, len(result), len(expected))
			}
		})
	}
}

func TestSegmentedSieveEdgeCases(t *testing.T) {
	tests := []struct {
		name string
		n    int
	}{
		{"n=0", 0},
		{"n=1", 1},
		{"n=2", 2},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			result := SegmentedSieve(tt.n, 10, nil)
			if result != nil {
				t.Errorf("SegmentedSieve(%d) = %v, want nil", tt.n, result)
			}
		})
	}
}

func TestParallelSegmentedSieveMatchesSegmented(t *testing.T) {
	testValues := []int{100, 500, 1000, 5000, 10000}
	for _, n := range testValues {
		t.Run(fmt.Sprintf("n=%d", n), func(t *testing.T) {
			expected := SegmentedSieve(n, 100, nil)
			result := ParallelSegmentedSieve(n, 2, 100, nil)
			if len(result) != len(expected) {
				t.Errorf("ParallelSegmentedSieve(%d) length = %d, want %d", n, len(result), len(expected))
				return
			}
			for i, v := range result {
				if v != expected[i] {
					t.Errorf("ParallelSegmentedSieve(%d)[%d] = %d, want %d", n, i, v, expected[i])
				}
			}
		})
	}
}

func TestParallelSegmentedSieveWithVariousWorkers(t *testing.T) {
	n := 10000
	expected := SegmentedSieve(n, 100, nil)
	workerCounts := []int{1, 2, 4}

	for _, workers := range workerCounts {
		t.Run(fmt.Sprintf("workers=%d", workers), func(t *testing.T) {
			result := ParallelSegmentedSieve(n, workers, 100, nil)
			if len(result) != len(expected) {
				t.Errorf("ParallelSegmentedSieve(%d, workers=%d) length = %d, want %d", n, workers, len(result), len(expected))
			}
		})
	}
}

func TestParallelSegmentedSieveEdgeCases(t *testing.T) {
	testValues := []int{0, 1, 2}
	for _, n := range testValues {
		t.Run(fmt.Sprintf("n=%d", n), func(t *testing.T) {
			result := ParallelSegmentedSieve(n, 2, 100, nil)
			if result != nil {
				t.Errorf("ParallelSegmentedSieve(%d) = %v, want nil", n, result)
			}
		})
	}
}

func TestParallelSegmentedSieveWithProgress(t *testing.T) {
	n := 10000
	expected := SegmentedSieve(n, 100, nil)
	totalDelta := 0
	callback := func(delta int) {
		totalDelta += delta
	}

	result := ParallelSegmentedSieve(n, 2, 100, callback)

	if len(result) != len(expected) {
		t.Errorf("ParallelSegmentedSieve with callback = %v, want %v", result, expected)
	}

	// Progress callback should have been called with total segments
	if totalDelta == 0 {
		t.Error("Progress callback was not called in parallel mode")
	}
}

func TestGeneratePrimesSmallInput(t *testing.T) {
	result := GeneratePrimes(10, false, nil)
	expected := []int{2, 3, 5, 7}

	if len(result) != len(expected) {
		t.Errorf("GeneratePrimes(10) = %v, want %v", result, expected)
	}
}

func TestGeneratePrimesLargeInput(t *testing.T) {
	n := 100000
	result := GeneratePrimes(n, false, nil)

	expectedCount := 9592
	if len(result) != expectedCount {
		t.Errorf("GeneratePrimes(%d) count = %d, want %d", n, len(result), expectedCount)
	}

	expectedLast := 99991
	if len(result) > 0 && result[len(result)-1] != expectedLast {
		t.Errorf("GeneratePrimes(%d) last prime = %d, want %d", n, result[len(result)-1], expectedLast)
	}
}

func TestGeneratePrimesWithParallel(t *testing.T) {
	n := 100000
	seqResult := GeneratePrimes(n, false, nil)
	parResult := GeneratePrimes(n, true, nil)

	if len(seqResult) != len(parResult) {
		t.Errorf("Sequential and parallel results have different lengths: %d vs %d", len(seqResult), len(parResult))
		return
	}

	for i, v := range seqResult {
		if v != parResult[i] {
			t.Errorf("Results differ at index %d: %d vs %d", i, v, parResult[i])
		}
	}
}

func TestGeneratePrimesProgressParameter(t *testing.T) {
	resultWithProgress := GeneratePrimes(50, false, func(i int) {})
	resultWithoutProgress := GeneratePrimes(50, false, nil)

	if len(resultWithProgress) != len(resultWithoutProgress) {
		t.Error("Results differ based on progress parameter")
	}
}

func TestNoComposites(t *testing.T) {
	primes := GeneratePrimes(50, false, nil)
	for _, p := range primes {
		if p <= 1 {
			t.Errorf("Found non-prime: %d", p)
		}
		// Actually verify primality
		if p > 2 && p%2 == 0 {
			t.Errorf("Found even composite: %d", p)
		}
		for d := 3; d*d <= p; d += 2 {
			if p%d == 0 {
				t.Errorf("Found composite: %d (divisible by %d)", p, d)
			}
		}
	}
}

func TestConsecutivePrimes(t *testing.T) {
	result := GeneratePrimes(20, false, nil)
	expected := []int{2, 3, 5, 7, 11, 13, 17, 19}

	if len(result) != len(expected) {
		t.Errorf("GeneratePrimes(20) = %v, want %v", result, expected)
	}
}
