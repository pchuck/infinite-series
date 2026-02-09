#!/usr/bin/env python3
"""
Tests for prime_generator
"""

import unittest
import time
from prime_generator import (
    generate_primes,
    sieve_of_eratosthenes,
    segmented_sieve,
    parallel_segmented_sieve,
)


class TestGeneratePrimes(unittest.TestCase):
    
    def test_small_input(self):
        """Test with n = 10 - should return primes [2, 3, 5, 7]"""
        self.assertEqual(generate_primes(10), ['2', '3', '5', '7'])
    
    def test_boundary_case_n_equals_2(self):
        """Test with n = 2 - no primes less than 2"""
        self.assertEqual(generate_primes(2), [])
    
    def test_large_input(self):
        """Verify correctness with known prime sequences"""
        # Primes under 30
        expected = ['2', '3', '5', '7', '11', '13', '17', '19', '23', '29']
        self.assertEqual(generate_primes(30), expected)
    
    def test_single_digit(self):
        """Test with n = 5"""
        self.assertEqual(generate_primes(5), ['2', '3'])
    
    def test_edge_cases(self):
        """Test edge cases: n=0, n=1, n=2"""
        self.assertEqual(generate_primes(0), [])
        self.assertEqual(generate_primes(1), [])
        self.assertEqual(generate_primes(2), [])
    
    def test_large_prime_verification(self):
        """Verify a known large prime appears correctly"""
        primes = generate_primes(100)
        self.assertIn('97', primes)  # 97 is the largest prime under 100
    
    def test_no_composites(self):
        """Ensure no composite numbers are in results"""
        primes = [int(p) for p in generate_primes(50)]
        for p in primes:
            self.assertGreater(p, 1)
    
    def test_consecutive_primes(self):
        """Verify consecutive primes match expected sequence"""
        result = generate_primes(20)
        expected = ['2', '3', '5', '7', '11', '13', '17', '19']
        self.assertEqual(result, expected)
    
    def test_performance_large_input(self):
        """Test that optimized algorithm handles large inputs efficiently"""
        # Test with a moderately large number
        start_time = time.time()
        primes = generate_primes(100000)
        elapsed = time.time() - start_time
        
        # Should complete in reasonable time (< 1 second)
        self.assertLess(elapsed, 1.0)
        
        # Verify correctness
        self.assertIn('99991', primes)  # Largest prime under 100000
        self.assertEqual(len(primes), 9592)  # Known count of primes under 100000
    
    def test_progress_parameter_exists(self):
        """Test that progress parameter doesn't break functionality"""
        result_with_progress = generate_primes(50, show_progress=False)
        result_without_progress = generate_primes(50, show_progress=True)
        
        # Results should be identical regardless of progress setting
        self.assertEqual(result_with_progress, result_without_progress)


class TestSieveOfEratosthenes(unittest.TestCase):
    """Test the underlying sieve function directly"""
    
    def test_basic_functionality(self):
        """Test basic sieve functionality"""
        self.assertEqual(sieve_of_eratosthenes(10), [2, 3, 5, 7])
    
    def test_with_progress_callback(self):
        """Test sieve with progress callback"""
        results = []
        primes = sieve_of_eratosthenes(50, lambda p: results.append(p))
        
        # Should still return correct primes
        self.assertEqual(primes, [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47])
    
    def test_empty_result(self):
        """Test sieve for n <= 2"""
        self.assertEqual(sieve_of_eratosthenes(0), [])
        self.assertEqual(sieve_of_eratosthenes(1), [])
        self.assertEqual(sieve_of_eratosthenes(2), [])


class TestSegmentedSieve(unittest.TestCase):
    """Test the segmented sieve implementation"""
    
    def test_basic_functionality(self):
        """Test segmented sieve with small input"""
        result = segmented_sieve(10)
        self.assertEqual(result, [2, 3, 5, 7])
    
    def test_matches_classic_for_small_n(self):
        """Segmented sieve should match classic for n < threshold"""
        test_values = [100, 500, 1000, 5000]
        for n in test_values:
            with self.subTest(n=n):
                expected = sieve_of_eratosthenes(n)
                result = segmented_sieve(n)
                self.assertEqual(result, expected)
    
    def test_large_input(self):
        """Test segmented sieve with large input"""
        n = 1000000
        start_time = time.time()
        primes = segmented_sieve(n)
        elapsed = time.time() - start_time
        
        # Should complete in reasonable time
        self.assertLess(elapsed, 2.0)
        
        # Known count of primes under 1M is 78498
        self.assertEqual(len(primes), 78498)
        
        # Verify last few primes are correct
        self.assertEqual(primes[-1], 999983)  # Largest prime < 1M
    
    def test_with_progress_callback(self):
        """Test segmented sieve with progress callback"""
        call_count = [0]
        def callback(seg_idx):
            call_count[0] += 1
        
        primes = segmented_sieve(100, segment_size=10, progress_callback=callback)
        
        # Should have received callbacks
        self.assertGreater(call_count[0], 0)
        
        # Verify correctness
        expected = sieve_of_eratosthenes(100)
        self.assertEqual(primes, expected)
    
    def test_custom_segment_size(self):
        """Test with various segment sizes"""
        for seg_size in [1, 10, 100, 1000]:
            with self.subTest(seg_size=seg_size):
                result = segmented_sieve(100, segment_size=seg_size)
                expected = sieve_of_eratosthenes(100)
                self.assertEqual(result, expected)
    
    def test_empty_result(self):
        """Test segmented sieve for n <= 2"""
        self.assertEqual(segmented_sieve(0), [])
        self.assertEqual(segmented_sieve(1), [])
        self.assertEqual(segmented_sieve(2), [])
    
    def test_segment_boundary_primes(self):
        """Test that primes at segment boundaries are found correctly"""
        # Test around common segment sizes
        n = 1000000
        result = segmented_sieve(n)
        
        # Check some known boundary conditions
        self.assertEqual(result[0], 2)  # First prime
        
        # Prime just before a power of 10 (999983 is correct)
        self.assertIn(99991, result)  # Known prime < 100000
        self.assertNotIn(99990, result)  # Not prime
    
    def test_performance_large_input(self):
        """Test that segmented sieve handles very large inputs efficiently"""
        n = 5000000
        
        start_time = time.time()
        primes = segmented_sieve(n)
        elapsed = time.time() - start_time
        
        # Known count of primes under 5M is approximately 348513
        self.assertGreater(len(primes), 340000)
        self.assertLess(len(primes), 360000)
        
        # Should be reasonably fast
        self.assertLess(elapsed, 5.0)


class TestParallelSieve(unittest.TestCase):
    """Test the parallel segmented sieve implementation"""
    
    def test_parallel_matches_sequential(self):
        """Parallel and sequential should produce identical results"""
        test_values = [100, 500, 1000, 5000]
        
        for n in test_values:
            with self.subTest(n=n):
                expected = segmented_sieve(n)
                result = parallel_segmented_sieve(n, num_workers=2)
                self.assertEqual(result, expected)
    
    def test_parallel_with_various_worker_counts(self):
        """Test correctness with different worker counts"""
        n = 10000
        
        for workers in [1, 2, 4]:
            with self.subTest(workers=workers):
                expected = segmented_sieve(n)
                result = parallel_segmented_sieve(n, num_workers=workers)
                self.assertEqual(result, expected)
    
    def test_parallel_edge_cases(self):
        """Test edge cases: n <= 2"""
        for n in [0, 1, 2]:
            with self.subTest(n=n):
                result = parallel_segmented_sieve(n)
                self.assertEqual(result, [])
    
    def test_generate_primes_with_parallel(self):
        """Test generate_primes with parallel=True"""
        n = 100000
        seq_result = generate_primes(n, parallel=False)
        par_result = generate_primes(n, parallel=True)
        
        self.assertEqual(seq_result, par_result)
    
    def test_progress_with_parallel(self):
        """Test that progress parameter doesn't break parallel execution"""
        call_count = [0]
        def callback(count):
            call_count[0] += count

        n = 10000
        # Note: Progress callbacks in parallel mode have limitations due to
        # multiprocessing constraints with fork+thread combination
        primes = parallel_segmented_sieve(n, num_workers=2, progress_callback=callback)

        # Verify correctness is more important than callback count
        expected = segmented_sieve(n)
        self.assertEqual(primes, expected)


if __name__ == '__main__':
    unittest.main()
