#include "primes.h"
#include <stdio.h>
#include <assert.h>
#include <string.h>

/* Test helper: check if a number is in the primes array */
static int contains(const size_t *primes, size_t count, size_t value) {
    for (size_t i = 0; i < count; i++) {
        if (primes[i] == value) return 1;
    }
    return 0;
}

/* Test helper: verify all numbers in array are prime */
static int all_primes(const size_t *primes, size_t count) {
    for (size_t i = 0; i < count; i++) {
        size_t p = primes[i];
        if (p < 2) return 0;
        if (p > 2 && p % 2 == 0) return 0;
        for (size_t d = 3; d * d <= p; d += 2) {
            if (p % d == 0) return 0;
        }
    }
    return 1;
}

/* Test helper: verify no composites in array */
static int no_composites(const size_t *primes, size_t count) {
    for (size_t i = 0; i < count; i++) {
        size_t p = primes[i];
        if (p == 2) continue;
        for (size_t d = 3; d * d <= p; d += 2) {
            if (p % d == 0) {
                printf("Composite found: %zu (divisible by %zu)\n", p, d);
                return 0;
            }
        }
    }
    return 1;
}

void test_sieve_small(void) {
    printf("Testing sieve_small... ");
    
    prime_result_t result = sieve_of_eratosthenes(10, NULL);
    assert(result.error == PRIME_OK);
    assert(result.count == 4);
    assert(contains(result.primes, result.count, 2));
    assert(contains(result.primes, result.count, 3));
    assert(contains(result.primes, result.count, 5));
    assert(contains(result.primes, result.count, 7));
    free_prime_result(&result);
    
    result = sieve_of_eratosthenes(30, NULL);
    assert(result.error == PRIME_OK);
    assert(result.count == 10);
    free_prime_result(&result);
    
    printf("PASSED\n");
}

void test_sieve_empty(void) {
    printf("Testing sieve_empty... ");
    
    prime_result_t result = sieve_of_eratosthenes(0, NULL);
    assert(result.error == PRIME_OK);
    assert(result.count == 0);
    free_prime_result(&result);
    
    result = sieve_of_eratosthenes(1, NULL);
    assert(result.error == PRIME_OK);
    assert(result.count == 0);
    free_prime_result(&result);
    
    result = sieve_of_eratosthenes(2, NULL);
    assert(result.error == PRIME_OK);
    assert(result.count == 0);
    free_prime_result(&result);
    
    printf("PASSED\n");
}

void test_sieve_boundary(void) {
    printf("Testing sieve_boundary... ");
    
    prime_result_t result = sieve_of_eratosthenes(3, NULL);
    assert(result.error == PRIME_OK);
    assert(result.count == 1);
    assert(result.primes[0] == 2);
    free_prime_result(&result);
    
    result = sieve_of_eratosthenes(4, NULL);
    assert(result.error == PRIME_OK);
    assert(result.count == 2);
    assert(result.primes[0] == 2);
    assert(result.primes[1] == 3);
    free_prime_result(&result);
    
    result = sieve_of_eratosthenes(5, NULL);
    assert(result.error == PRIME_OK);
    assert(result.count == 2);
    free_prime_result(&result);
    
    result = sieve_of_eratosthenes(6, NULL);
    assert(result.error == PRIME_OK);
    assert(result.count == 3);
    assert(contains(result.primes, result.count, 5));
    free_prime_result(&result);
    
    printf("PASSED\n");
}

void test_segmented_matches_classic(void) {
    printf("Testing segmented_matches_classic... ");
    
    size_t test_values[] = {100, 500, 1000, 5000};
    
    for (size_t i = 0; i < sizeof(test_values) / sizeof(test_values[0]); i++) {
        size_t n = test_values[i];
        
        prime_result_t classic = sieve_of_eratosthenes(n, NULL);
        prime_result_t segmented = segmented_sieve(n, 100, NULL);
        
        assert(classic.error == PRIME_OK);
        assert(segmented.error == PRIME_OK);
        assert(classic.count == segmented.count);
        
        for (size_t j = 0; j < classic.count; j++) {
            assert(classic.primes[j] == segmented.primes[j]);
        }
        
        free_prime_result(&classic);
        free_prime_result(&segmented);
    }
    
    printf("PASSED\n");
}

void test_parallel_matches_segmented(void) {
    printf("Testing parallel_matches_segmented... ");
    
    size_t test_values[] = {100, 500, 1000, 5000};
    
    for (size_t i = 0; i < sizeof(test_values) / sizeof(test_values[0]); i++) {
        size_t n = test_values[i];
        
        prime_result_t segmented = segmented_sieve(n, 100, NULL);
        prime_result_t parallel = parallel_segmented_sieve(n, 2, 100, NULL);
        
        assert(segmented.error == PRIME_OK);
        assert(parallel.error == PRIME_OK);
        assert(segmented.count == parallel.count);
        
        for (size_t j = 0; j < segmented.count; j++) {
            assert(segmented.primes[j] == parallel.primes[j]);
        }
        
        free_prime_result(&segmented);
        free_prime_result(&parallel);
    }
    
    printf("PASSED\n");
}

void test_large_input(void) {
    printf("Testing large_input... ");
    
    prime_result_t result = segmented_sieve(1000000, DEFAULT_SEGMENT_SIZE, NULL);
    assert(result.error == PRIME_OK);
    assert(result.count == 78498);
    assert(result.primes[0] == 2);
    assert(result.primes[result.count - 1] == 999983);
    free_prime_result(&result);
    
    printf("PASSED\n");
}

void test_no_composites(void) {
    printf("Testing no_composites... ");
    
    prime_result_t result = sieve_of_eratosthenes(200, NULL);
    assert(result.error == PRIME_OK);
    assert(all_primes(result.primes, result.count));
    assert(no_composites(result.primes, result.count));
    free_prime_result(&result);
    
    printf("PASSED\n");
}

void test_segmented_various_segment_sizes(void) {
    printf("Testing segmented_various_segment_sizes... ");
    
    prime_result_t expected = sieve_of_eratosthenes(1000, NULL);
    size_t segment_sizes[] = {1, 7, 10, 50, 100, 999, 1000, 2000};
    
    for (size_t i = 0; i < sizeof(segment_sizes) / sizeof(segment_sizes[0]); i++) {
        size_t seg_size = segment_sizes[i];
        prime_result_t result = segmented_sieve(1000, seg_size, NULL);
        
        assert(result.error == PRIME_OK);
        assert(result.count == expected.count);
        
        for (size_t j = 0; j < result.count; j++) {
            assert(result.primes[j] == expected.primes[j]);
        }
        
        free_prime_result(&result);
    }
    
    free_prime_result(&expected);
    
    printf("PASSED\n");
}

void test_parallel_various_workers(void) {
    printf("Testing parallel_various_workers... ");
    
    prime_result_t expected = segmented_sieve(10000, 100, NULL);
    
    for (size_t workers = 1; workers <= 4; workers++) {
        prime_result_t result = parallel_segmented_sieve(10000, workers, 100, NULL);
        
        assert(result.error == PRIME_OK);
        assert(result.count == expected.count);
        
        for (size_t j = 0; j < result.count; j++) {
            assert(result.primes[j] == expected.primes[j]);
        }
        
        free_prime_result(&result);
    }
    
    free_prime_result(&expected);
    
    printf("PASSED\n");
}

void test_generate_primes_auto_select(void) {
    printf("Testing generate_primes_auto_select... ");
    
    /* Small input: classic sieve */
    prime_result_t result = generate_primes(100, false, 0, 0, NULL);
    assert(result.error == PRIME_OK);
    assert(result.count == 25);
    free_prime_result(&result);
    
    /* Medium input: segmented sieve */
    result = generate_primes(1000000, false, 0, 0, NULL);
    assert(result.error == PRIME_OK);
    assert(result.count == 78498);
    free_prime_result(&result);
    
    /* Large input with parallel flag */
    result = generate_primes(10000000, true, 4, DEFAULT_SEGMENT_SIZE, NULL);
    assert(result.error == PRIME_OK);
    assert(result.count == 664579);
    free_prime_result(&result);
    
    printf("PASSED\n");
}

void test_isqrt(void) {
    printf("Testing isqrt... ");
    
    assert(isqrt(0) == 0);
    assert(isqrt(1) == 1);
    assert(isqrt(4) == 2);
    assert(isqrt(9) == 3);
    assert(isqrt(16) == 4);
    assert(isqrt(25) == 5);
    assert(isqrt(100) == 10);
    assert(isqrt(10000) == 100);
    assert(isqrt(99) == 9);
    assert(isqrt(1000) == 31);
    
    printf("PASSED\n");
}

void test_estimate_prime_count(void) {
    printf("Testing estimate_prime_count... ");
    
    /* For small n, estimate should return n */
    assert(estimate_prime_count(0) == 1);
    assert(estimate_prime_count(1) == 1);
    assert(estimate_prime_count(2) == 1);
    
    /* For larger n, estimate should be reasonable */
    size_t est_1000 = estimate_prime_count(1000);
    assert(est_1000 > 100 && est_1000 < 200);
    
    size_t est_1000000 = estimate_prime_count(1000000);
    assert(est_1000000 > 70000 && est_1000000 < 85000);
    
    printf("PASSED\n");
}

int main(void) {
    printf("=== C Primes Test Suite ===\n\n");
    
    test_isqrt();
    test_estimate_prime_count();
    test_sieve_small();
    test_sieve_empty();
    test_sieve_boundary();
    test_segmented_matches_classic();
    test_parallel_matches_segmented();
    test_large_input();
    test_no_composites();
    test_segmented_various_segment_sizes();
    test_parallel_various_workers();
    test_generate_primes_auto_select();
    
    printf("\n=== All tests PASSED ===\n");
    
    return 0;
}
