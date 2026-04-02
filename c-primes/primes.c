#define _POSIX_C_SOURCE 200809L
#include "primes.h"
#include <stdlib.h>
#include <string.h>
#include <math.h>
#include <stdio.h>
#include <stdint.h>
#include <unistd.h>

/* Integer square root using Newton's method */
size_t isqrt(size_t n) {
    if (n == 0) return 0;
    
    size_t x = (size_t)sqrt((double)n);
    
    /* Refine the estimate */
    while (x > 0 && x * x > n) {
        x--;
    }
    while ((x + 1) <= n / (x + 1)) {
        x++;
    }
    
    return x;
}

/* Estimate the number of primes up to n using Prime Number Theorem */
size_t estimate_prime_count(size_t n) {
    if (n <= 2) return 1;
    
    double ln_n = log((double)n);
    size_t estimated;
    
    if (ln_n > 1.1) {
        estimated = (size_t)((double)n / (ln_n - 1.1));
    } else {
        estimated = n;
    }
    
    return estimated > 0 ? estimated : 1;
}

/* Allocate and initialize a prime_result_t */
static prime_result_t create_result(size_t capacity) {
    prime_result_t result;
    result.capacity = capacity > 0 ? capacity : 1;
    result.primes = (size_t *)malloc(result.capacity * sizeof(size_t));
    result.count = 0;
    result.error = PRIME_OK;
    result.error_msg = NULL;
    return result;
}

/* Add a prime to the result, resizing if necessary */
static void add_prime(prime_result_t *result, size_t prime) {
    if (result->count >= result->capacity) {
        size_t new_capacity = result->capacity * 2;
        size_t *new_primes = (size_t *)realloc(result->primes, new_capacity * sizeof(size_t));
        if (new_primes) {
            result->primes = new_primes;
            result->capacity = new_capacity;
        }
    }
    if (result->count < result->capacity) {
        result->primes[result->count++] = prime;
    }
}



/* Classic Sieve of Eratosthenes (odd-only)
 * Best for n < 1,000,000
 */
prime_result_t sieve_of_eratosthenes(size_t n, progress_callback_t progress) {
    if (n <= 2) {
        prime_result_t result = create_result(0);
        return result;
    }
    
    if (n <= 3) {
        prime_result_t result = create_result(1);
        add_prime(&result, 2);
        return result;
    }
    
    /* Odd-only sieve: index i represents number 2*i + 3 */
    size_t sieve_size = (n - 3) / 2 + 1;
    if (sieve_size < 1) sieve_size = 1;
    
    bool *sieve = (bool *)calloc(sieve_size, sizeof(bool));
    if (!sieve) {
        prime_result_t result = create_result(0);
        result.error = PRIME_ERR_MEMORY_ALLOCATION;
        result.error_msg = strdup("Failed to allocate sieve array");
        return result;
    }
    
    /* Initialize all to true */
    for (size_t i = 0; i < sieve_size; i++) {
        sieve[i] = true;
    }
    
    size_t limit = isqrt(n);
    size_t current = 3;
    
    while (current <= limit) {
        size_t idx = (current - 3) / 2;
        if (idx < sieve_size && sieve[idx]) {
            unsigned long long squared = (unsigned long long)current * (unsigned long long)current;
            if (squared >= (unsigned long long)n) break;
            
            size_t start_idx = (size_t)((squared - 3) / 2);
            size_t step = current;
            
            for (size_t j = start_idx; j < sieve_size; j += step) {
                sieve[j] = false;
            }
        }
        current += 2;
    }
    
    /* Extract primes */
    prime_result_t result = create_result(estimate_prime_count(n));
    add_prime(&result, 2);
    
    for (size_t i = 0; i < sieve_size; i++) {
        if (sieve[i]) {
            size_t prime = 2 * i + 3;
            if (prime < n) {
                add_prime(&result, prime);
            }
        }
    }
    
    free(sieve);
    
    /* Invoke progress callback */
    if (progress) {
        progress(1);
    }
    
    return result;
}

/* Process a single segment using odd-only sieve
 * Shared helper for segmented and parallel sieves
 */
static void sieve_segment_odd_only(
    size_t low, size_t high,
    const size_t *base_primes_odd, size_t base_count,
    bool *is_prime, size_t buffer_size,
    size_t *segment_primes, size_t *seg_count, size_t seg_capacity
) {
    *seg_count = 0;
    
    /* Handle prime 2 if in this segment */
    if (low <= 2 && high > 2) {
        if (*seg_count < seg_capacity) {
            segment_primes[(*seg_count)++] = 2;
        }
    }
    
    /* Find first odd number >= low */
    size_t odd_low = low < 3 ? 3 : low;
    if (odd_low % 2 == 0) odd_low++;
    
    if (odd_low >= high) return;
    
    size_t seg_len = (high - odd_low + 1) / 2;
    if (seg_len == 0) return;
    if (seg_len > buffer_size) seg_len = buffer_size;
    
    /* Reset buffer */
    for (size_t i = 0; i < seg_len; i++) {
        is_prime[i] = true;
    }
    
    /* Sieve with base primes */
    for (size_t i = 0; i < base_count; i++) {
        size_t p = base_primes_odd[i];
        
        /* Find first odd multiple of p in [odd_low, high) */
        size_t start = (low + p - 1) / p * p;
        if (start < p * p) start = p * p;
        if (start % 2 == 0) start += p;
        
        if (start >= high) continue;
        
        size_t adjusted_start = (start - odd_low) / 2;
        size_t step = p;
        
        for (size_t j = adjusted_start; j < seg_len; j += step) {
            is_prime[j] = false;
        }
    }
    
    /* Extract primes from segment */
    for (size_t i = 0; i < seg_len; i++) {
        if (is_prime[i]) {
            size_t prime = odd_low + 2 * i;
            if (prime < high && *seg_count < seg_capacity) {
                segment_primes[(*seg_count)++] = prime;
            }
        }
    }
}

/* Segmented Sieve of Eratosthenes (odd-only)
 * Best for n >= 1,000,000
 */
prime_result_t segmented_sieve(size_t n, size_t segment_size, progress_callback_t progress) {
    if (n <= 2) {
        prime_result_t result = create_result(0);
        return result;
    }
    
    /* Validate segment size */
    size_t segments = n / segment_size + (n % segment_size > 0 ? 1 : 0);
    if (segments == 0) segments = 1;
    
    /* Generate base primes up to sqrt(n) */
    size_t base_limit = isqrt(n);
    prime_result_t base_result = sieve_of_eratosthenes(base_limit + 1, NULL);
    if (base_result.error != PRIME_OK) {
        prime_result_t result = create_result(0);
        result.error = base_result.error;
        result.error_msg = base_result.error_msg;
        return result;
    }
    
    /* Extract odd base primes only */
    size_t *base_primes_odd = NULL;
    size_t base_odd_count = 0;
    
    if (base_result.count > 0) {
        base_primes_odd = (size_t *)malloc(base_result.count * sizeof(size_t));
        if (!base_primes_odd) {
            free_prime_result(&base_result);
            prime_result_t result = create_result(0);
            result.error = PRIME_ERR_MEMORY_ALLOCATION;
            result.error_msg = strdup("Failed to allocate base primes array");
            return result;
        }
        
        for (size_t i = 0; i < base_result.count; i++) {
            if (base_result.primes[i] > 2) {
                base_primes_odd[base_odd_count++] = base_result.primes[i];
            }
        }
    }
    
    free_prime_result(&base_result);
    
    /* Allocate result */
    prime_result_t result = create_result(estimate_prime_count(n));
    
    /* Reusable buffers for segments */
    size_t buffer_size = segment_size / 2 + 1;
    bool *is_prime = (bool *)malloc(buffer_size * sizeof(bool));
    if (!is_prime) {
        free(base_primes_odd);
        result.error = PRIME_ERR_MEMORY_ALLOCATION;
        result.error_msg = strdup("Failed to allocate segment buffer");
        return result;
    }
    
    size_t *seg_primes = (size_t *)malloc(segment_size * sizeof(size_t));
    if (!seg_primes) {
        free(is_prime);
        free(base_primes_odd);
        result.error = PRIME_ERR_MEMORY_ALLOCATION;
        result.error_msg = strdup("Failed to allocate segment primes array");
        return result;
    }
    
    /* Process segments */
    for (size_t seg_idx = 0; seg_idx < segments; seg_idx++) {
        size_t low = seg_idx * segment_size;
        size_t high = low + segment_size;
        if (high > n) high = n;
        
        if (high <= 2) continue;
        
        size_t seg_count = 0;
        sieve_segment_odd_only(low, high, base_primes_odd, base_odd_count,
                              is_prime, buffer_size, seg_primes, &seg_count, segment_size);
        
        /* Add to result */
        for (size_t i = 0; i < seg_count && result.count < result.capacity; i++) {
            add_prime(&result, seg_primes[i]);
        }
        
        /* Progress callback */
        if (progress) {
            progress(1);
        }
    }
    
    free(seg_primes);
    free(is_prime);
    free(base_primes_odd);
    
    return result;
}

/* Worker thread function for parallel segmented sieve
 * Buffers (worker_primes, is_prime_buffer) are pre-allocated by the caller
 */
static void *worker_thread(void *arg) {
    worker_context_t *ctx = (worker_context_t *)arg;
    
    ctx->worker_count = 0;
    size_t buffer_size = ctx->segment_size / 2 + 1;
    
    for (size_t seg_idx = ctx->start_seg; seg_idx < ctx->end_seg; seg_idx++) {
        size_t low = seg_idx * ctx->segment_size;
        size_t high = low + ctx->segment_size;
        if (high > ctx->n) high = ctx->n;
        
        if (high <= 2) continue;
        
        size_t seg_count = 0;
        sieve_segment_odd_only(low, high, ctx->base_primes_odd, ctx->base_primes_count,
                              ctx->is_prime_buffer, buffer_size,
                              ctx->worker_primes + ctx->worker_count, &seg_count, 
                              ctx->worker_capacity - ctx->worker_count);
        
        ctx->worker_count += seg_count;
        
        if (ctx->progress) {
            ctx->progress(1);
        }
    }
    
    ctx->error = PRIME_OK;
    return NULL;
}

/* Parallel Segmented Sieve (odd-only)
 * Best for n >= 5,000,000
 */
prime_result_t parallel_segmented_sieve(size_t n, size_t workers, size_t segment_size, progress_callback_t progress) {
    if (n <= 2) {
        prime_result_t result = create_result(0);
        return result;
    }
    
    /* Auto-detect available parallelism if workers not specified */
    if (workers == 0) {
#ifdef _SC_NPROCESSORS_ONLN
        long ncpu = sysconf(_SC_NPROCESSORS_ONLN);
        workers = (ncpu > 0) ? (size_t)ncpu : 4;
#else
        workers = 4;
#endif
    }
    if (workers > MAX_WORKERS) workers = MAX_WORKERS;
    
    /* Validate segment size */
    size_t segments = n / segment_size + (n % segment_size > 0 ? 1 : 0);
    if (segments == 0) segments = 1;
    
    /* Adjust workers to not exceed segments */
    if (workers > segments) workers = segments;
    
    /* Generate base primes up to sqrt(n) */
    size_t base_limit = isqrt(n);
    prime_result_t base_result = sieve_of_eratosthenes(base_limit + 1, NULL);
    if (base_result.error != PRIME_OK) {
        prime_result_t result = create_result(0);
        result.error = base_result.error;
        result.error_msg = base_result.error_msg;
        return result;
    }
    
    /* Extract odd base primes only */
    size_t *base_primes_odd = NULL;
    size_t base_odd_count = 0;
    
    if (base_result.count > 0) {
        base_primes_odd = (size_t *)malloc(base_result.count * sizeof(size_t));
        if (!base_primes_odd) {
            free_prime_result(&base_result);
            prime_result_t result = create_result(0);
            result.error = PRIME_ERR_MEMORY_ALLOCATION;
            result.error_msg = strdup("Failed to allocate base primes array");
            return result;
        }
        
        for (size_t i = 0; i < base_result.count; i++) {
            if (base_result.primes[i] > 2) {
                base_primes_odd[base_odd_count++] = base_result.primes[i];
            }
        }
    }
    
    free_prime_result(&base_result);
    
    /* Calculate chunk size per worker */
    size_t chunk_size = segments / workers;
    if (segments % workers > 0) chunk_size++;
    
    /* Create worker contexts and threads */
    worker_context_t *contexts = (worker_context_t *)malloc(workers * sizeof(worker_context_t));
    pthread_t *threads = (pthread_t *)malloc(workers * sizeof(pthread_t));
    
    if (!contexts || !threads) {
        free(base_primes_odd);
        prime_result_t result = create_result(0);
        result.error = PRIME_ERR_MEMORY_ALLOCATION;
        result.error_msg = strdup("Failed to allocate worker structures");
        return result;
    }
    
    /* Pre-allocate worker buffers in main thread */
    size_t buffer_size = segment_size / 2 + 1;
    for (size_t w = 0; w < workers; w++) {
        size_t segs_for_worker = chunk_size;
        if (w * chunk_size + chunk_size > segments) {
            segs_for_worker = segments - w * chunk_size;
        }
        size_t worker_capacity = segs_for_worker * segment_size;
        
        contexts[w].worker_primes = (size_t *)malloc(worker_capacity * sizeof(size_t));
        contexts[w].is_prime_buffer = (bool *)malloc(buffer_size * sizeof(bool));
        
        if (!contexts[w].worker_primes || !contexts[w].is_prime_buffer) {
            /* Clean up on allocation failure */
            for (size_t k = 0; k <= w; k++) {
                free(contexts[k].worker_primes);
                free(contexts[k].is_prime_buffer);
            }
            free(contexts);
            free(threads);
            free(base_primes_odd);
            prime_result_t result = create_result(0);
            result.error = PRIME_ERR_MEMORY_ALLOCATION;
            result.error_msg = strdup("Failed to allocate worker buffers");
            return result;
        }
        
        contexts[w].worker_capacity = worker_capacity;
    }
    
    for (size_t w = 0; w < workers; w++) {
        contexts[w].n = n;
        contexts[w].start_seg = w * chunk_size;
        contexts[w].end_seg = contexts[w].start_seg + chunk_size;
        if (contexts[w].end_seg > segments) contexts[w].end_seg = segments;
        contexts[w].segment_size = segment_size;
        contexts[w].base_primes_odd = base_primes_odd;
        contexts[w].base_primes_count = base_odd_count;
        contexts[w].worker_count = 0;
        contexts[w].progress = progress;
        contexts[w].error = PRIME_OK;
        contexts[w].error_msg = NULL;
        
        if (contexts[w].start_seg >= segments) {
            contexts[w].end_seg = contexts[w].start_seg;
        }
    }
    
    /* Launch worker threads */
    size_t launched = 0;
    for (size_t w = 0; w < workers; w++) {
        if (contexts[w].start_seg >= contexts[w].end_seg) continue;
        
        int rc = pthread_create(&threads[launched], NULL, worker_thread, &contexts[w]);
        if (rc != 0) {
            prime_result_t result = create_result(0);
            result.error = PRIME_ERR_WORKER_THREAD_PANIC;
            char msg[64];
            snprintf(msg, sizeof(msg), "pthread_create failed with code %d", rc);
            result.error_msg = strdup(msg);
            
            /* Clean up */
            for (size_t k = 0; k < launched; k++) {
                pthread_join(threads[k], NULL);
            }
            for (size_t k = 0; k < workers; k++) {
                free(contexts[k].worker_primes);
                free(contexts[k].is_prime_buffer);
            }
            free(contexts);
            free(threads);
            free(base_primes_odd);
            return result;
        }
        launched++;
    }
    
    /* Wait for all workers */
    for (size_t w = 0; w < launched; w++) {
        pthread_join(threads[w], NULL);
    }
    
    /* Collect results from workers */
    prime_result_t result = create_result(estimate_prime_count(n));
    
    for (size_t w = 0; w < workers; w++) {
        if (contexts[w].error != PRIME_OK) {
            result.error = contexts[w].error;
            result.error_msg = contexts[w].error_msg;
            break;
        }
        
        for (size_t i = 0; i < contexts[w].worker_count; i++) {
            add_prime(&result, contexts[w].worker_primes[i]);
        }
    }
    
    /* Free pre-allocated worker buffers */
    for (size_t w = 0; w < workers; w++) {
        free(contexts[w].worker_primes);
        free(contexts[w].is_prime_buffer);
    }
    
    free(contexts);
    free(threads);
    free(base_primes_odd);
    
    return result;
}

/* Auto-select algorithm based on n */
prime_result_t generate_primes(size_t n, bool parallel, size_t workers, size_t segment_size, progress_callback_t progress) {
    if (n <= 2) {
        prime_result_t result = create_result(0);
        return result;
    }
    
    /* Check MAX_N */
    if (n > MAX_N) {
        prime_result_t result = create_result(0);
        result.error = PRIME_ERR_INVALID_INPUT;
        char msg[128];
        snprintf(msg, sizeof(msg), "n=%zu exceeds maximum supported value %llu (1 quadrillion)", n, (unsigned long long)MAX_N);
        result.error_msg = strdup(msg);
        return result;
    }
    
    /* Validate workers */
    if (workers == 0) {
        workers = 4; /* Default */
    }
    if (workers > MAX_WORKERS) workers = MAX_WORKERS;
    
    /* Validate segment size */
    if (segment_size == 0) {
        segment_size = DEFAULT_SEGMENT_SIZE;
    }
    
    /* Select algorithm */
    if (parallel && n >= PARALLEL_THRESHOLD) {
        return parallel_segmented_sieve(n, workers, segment_size, progress);
    } else if (n >= DEFAULT_SEGMENT_SIZE) {
        return segmented_sieve(n, segment_size, progress);
    } else {
        return sieve_of_eratosthenes(n, progress);
    }
}

/* Free prime_result_t resources */
void free_prime_result(prime_result_t *result) {
    if (result->primes) {
        free(result->primes);
        result->primes = NULL;
    }
    if (result->error_msg) {
        free(result->error_msg);
        result->error_msg = NULL;
    }
    result->count = 0;
    result->capacity = 0;
}

/* Validate segment size */
prime_error_t validate_segment_size(size_t n, size_t segment_size, char **error_msg) {
    if (segment_size == 0) {
        if (error_msg) {
            *error_msg = strdup("segment_size cannot be zero");
        }
        return PRIME_ERR_INVALID_INPUT;
    }
    
    size_t segments = n / segment_size + (n % segment_size > 0 ? 1 : 0);
    if (segments > 0 && segment_size > UINTPTR_MAX / segments) {
        if (error_msg) {
            *error_msg = strdup("segment_size would cause overflow in calculations");
        }
        return PRIME_ERR_INVALID_INPUT;
    }
    
    return PRIME_OK;
}

/* Validate workers parameter */
prime_error_t validate_workers(size_t workers, char **error_msg) {
    if (workers == 0) {
        if (error_msg) {
            *error_msg = strdup("workers must be >= 1");
        }
        return PRIME_ERR_INVALID_INPUT;
    }
    
    if (workers > MAX_WORKERS) {
        if (error_msg) {
            char msg[64];
            snprintf(msg, sizeof(msg), "workers=%zu exceeds maximum %d", workers, MAX_WORKERS);
            *error_msg = strdup(msg);
        }
        return PRIME_ERR_INVALID_INPUT;
    }
    
    return PRIME_OK;
}
