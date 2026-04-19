#ifndef PRIMES_H
#define PRIMES_H

#include <stddef.h>
#include <stdbool.h>
#include <pthread.h>

/* Default segment size for segmented sieve (1M elements) */
#define DEFAULT_SEGMENT_SIZE 1000000

/* Maximum number of worker threads */
#define MAX_WORKERS 1024

/* Minimum input size for parallel processing (5M) */
#define PARALLEL_THRESHOLD 5000000

/* Maximum input size (1 quadrillion) */
#define MAX_N 1000000000000000ULL

/* Error codes */
typedef enum {
    PRIME_OK = 0,
    PRIME_ERR_INVALID_INPUT,
    PRIME_ERR_WORKER_THREAD_PANIC,
    PRIME_ERR_MEMORY_ALLOCATION
} prime_error_t;

/* Progress callback type */
typedef void (*progress_callback_t)(size_t delta);

/* Prime generation result */
typedef struct {
    size_t *primes;
    size_t count;
    size_t capacity;
    prime_error_t error;
    char *error_msg;
} prime_result_t;

/* Worker thread context for parallel sieve */
typedef struct {
    size_t n;
    size_t start_seg;
    size_t end_seg;
    size_t segment_size;
    size_t *base_primes_odd;
    size_t base_primes_count;
    bool *is_prime_buffer;
    size_t *worker_primes;
    size_t worker_capacity;
    size_t worker_count;
    progress_callback_t progress;
    prime_error_t error;
    char *error_msg;
} worker_context_t;

/* Integer square root using Newton's method */
size_t isqrt(size_t n);

/* Estimate the number of primes up to n using Prime Number Theorem */
size_t estimate_prime_count(size_t n);

/* Classic Sieve of Eratosthenes (odd-only)
 * Best for n < 1,000,000
 */
prime_result_t sieve_of_eratosthenes(size_t n, progress_callback_t progress);

/* Segmented Sieve of Eratosthenes (odd-only)
 * Best for n >= 1,000,000
 */
prime_result_t segmented_sieve(size_t n, size_t segment_size, progress_callback_t progress);

/* Parallel Segmented Sieve (odd-only)
 * Best for n >= 5,000,000
 */
prime_result_t parallel_segmented_sieve(size_t n, size_t workers, size_t segment_size, progress_callback_t progress);

/* Auto-select algorithm based on n
 * Returns primes in result.primes array with count in result.count
 */
prime_result_t generate_primes(size_t n, bool parallel, size_t workers, size_t segment_size, progress_callback_t progress);

/* Free prime_result_t resources */
void free_prime_result(prime_result_t *result);

/* Validate segment size */
prime_error_t validate_segment_size(size_t n, size_t segment_size, char **error_msg);

/* Validate workers parameter */
prime_error_t validate_workers(size_t workers, char **error_msg);

#endif /* PRIMES_H */
