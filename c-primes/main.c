#include "primes.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include <pthread.h>

#define DEFAULT_SEGMENT_SIZE_CLI 1000000

/* Format number with comma separators */
static void format_number(size_t n, char *buf, size_t bufsize) {
    char temp[32];
    size_t len = snprintf(temp, sizeof(temp), "%zu", n);
    
    size_t j = 0;
    size_t commas = (len - 1) / 3;
    size_t total_len = len + commas;
    
    if (total_len >= bufsize) total_len = bufsize - 1;
    
    for (size_t i = 0; i < len; i++) {
        if (i > 0 && (len - i) % 3 == 0) {
            buf[j++] = ',';
        }
        buf[j++] = temp[i];
    }
    buf[j] = '\0';
}

/* Print usage */
static void print_usage(const char *prog_name) {
    fprintf(stderr, "Usage: %s [OPTIONS]\n", prog_name);
    fprintf(stderr, "\nHigh-performance prime number generator\n\n");
    fprintf(stderr, "Options:\n");
    fprintf(stderr, "  -n, --number NUM       Upper bound (exclusive) for prime generation\n");
    fprintf(stderr, "  -P, --progress         Show progress bar\n");
    fprintf(stderr, "  -p, --parallel         Use parallel processing (for n >= 5M)\n");
    fprintf(stderr, "  -w, --workers NUM      Number of worker threads (default: 4)\n");
    fprintf(stderr, "  -s, --segment NUM      Segment size (default: 1000000)\n");
    fprintf(stderr, "  -q, --quiet            Only print count (no prime list)\n");
    fprintf(stderr, "  -h, --help             Show this help message\n");
    fprintf(stderr, "  -v, --version          Show version\n");
}

/* Print version */
static void print_version(void) {
    fprintf(stderr, "primes 1.0.0\n");
}

/* Parse command line arguments */
typedef struct {
    size_t n;
    bool n_specified;
    bool progress;
    bool parallel;
    size_t workers;
    bool workers_specified;
    size_t segment;
    bool segment_specified;
    bool quiet;
} cli_args_t;

static int parse_args(int argc, char *argv[], cli_args_t *args) {
    args->n = 0;
    args->n_specified = false;
    args->progress = false;
    args->parallel = false;
    args->workers = 4;
    args->workers_specified = false;
    args->segment = DEFAULT_SEGMENT_SIZE_CLI;
    args->segment_specified = false;
    args->quiet = false;
    
    for (int i = 1; i < argc; i++) {
        if (strcmp(argv[i], "-n") == 0 || strcmp(argv[i], "--number") == 0) {
            if (i + 1 >= argc) {
                fprintf(stderr, "Error: -n requires a value\n");
                return -1;
            }
            char *endptr;
            args->n = strtoul(argv[++i], &endptr, 10);
            if (*endptr != '\0') {
                fprintf(stderr, "Error: Invalid number '%s'\n", argv[i]);
                return -1;
            }
            args->n_specified = true;
        } else if (strcmp(argv[i], "-P") == 0 || strcmp(argv[i], "--progress") == 0) {
            args->progress = true;
        } else if (strcmp(argv[i], "-p") == 0 || strcmp(argv[i], "--parallel") == 0) {
            args->parallel = true;
        } else if (strcmp(argv[i], "-w") == 0 || strcmp(argv[i], "--workers") == 0) {
            if (i + 1 >= argc) {
                fprintf(stderr, "Error: -w requires a value\n");
                return -1;
            }
            char *endptr;
            args->workers = strtoul(argv[++i], &endptr, 10);
            if (*endptr != '\0') {
                fprintf(stderr, "Error: Invalid number '%s'\n", argv[i]);
                return -1;
            }
            args->workers_specified = true;
        } else if (strcmp(argv[i], "-s") == 0 || strcmp(argv[i], "--segment") == 0) {
            if (i + 1 >= argc) {
                fprintf(stderr, "Error: -s requires a value\n");
                return -1;
            }
            char *endptr;
            args->segment = strtoul(argv[++i], &endptr, 10);
            if (*endptr != '\0') {
                fprintf(stderr, "Error: Invalid number '%s'\n", argv[i]);
                return -1;
            }
            args->segment_specified = true;
        } else if (strcmp(argv[i], "-q") == 0 || strcmp(argv[i], "--quiet") == 0) {
            args->quiet = true;
        } else if (strcmp(argv[i], "-h") == 0 || strcmp(argv[i], "--help") == 0) {
            print_usage(argv[0]);
            return 0;
        } else if (strcmp(argv[i], "-v") == 0 || strcmp(argv[i], "--version") == 0) {
            print_version();
            return 0;
        } else {
            fprintf(stderr, "Error: Unknown option '%s'\n", argv[i]);
            return -1;
        }
    }
    
    return 1;
}

int main(int argc, char *argv[]) {
    cli_args_t args;
    
    if (parse_args(argc, argv, &args) <= 0) {
        return 0;
    }
    
    /* Get n from command line or interactive input */
    size_t n = args.n;
    if (!args.n_specified) {
        fprintf(stderr, "Enter upper bound (n): ");
        fflush(stderr);
        char input[64];
        if (!fgets(input, sizeof(input), stdin)) {
            fprintf(stderr, "Error: Failed to read input\n");
            return 1;
        }
        char *endptr;
        n = strtoul(input, &endptr, 10);
        if (*endptr != '\0' && *endptr != '\n') {
            fprintf(stderr, "Error: Invalid input '%s'\n", input);
            return 1;
        }
    }
    
    if (n <= 2) {
        printf("No primes less than %zu\n", n);
        return 0;
    }
    
    /* Warn if parallel is specified but below threshold */
    if (args.parallel && n < PARALLEL_THRESHOLD) {
        fprintf(stderr, "[WARN] --parallel ignored: n=%zu is below threshold %d\n", n, PARALLEL_THRESHOLD);
    }
    
    /* Ensure segment size is valid */
    if (args.segment == 0) {
        fprintf(stderr, "Error: --segment must be greater than 0\n");
        return 1;
    }
    
    /* Time the computation */
    clock_t compute_start = clock();
    
    /* Generate primes */
    prime_result_t result = generate_primes(n, args.parallel,
                                            args.workers_specified ? args.workers : 4,
                                            args.segment,
                                            NULL);
    
    clock_t compute_end = clock();
    double compute_secs = (double)(compute_end - compute_start) / CLOCKS_PER_SEC;
    
    /* Check for errors */
    if (result.error != PRIME_OK) {
        fprintf(stderr, "Error: Prime generation failed: %s\n",
                result.error_msg ? result.error_msg : "Unknown error");
        free_prime_result(&result);
        return 1;
    }
    
    /* Output results */
    if (!args.quiet && result.count > 0) {
        printf("Primes less than %zu:\n", n);
        
        /* Stream output with buffering */
        size_t buffer_size = 1024;
        char *output_buffer = (char *)malloc(buffer_size);
        size_t buf_pos = 0;
        
        buf_pos += snprintf(output_buffer + buf_pos, buffer_size - buf_pos, "Primes less than %zu:\n", n);
        
        for (size_t i = 0; i < result.count; i++) {
            if (i > 0) {
                buf_pos += snprintf(output_buffer + buf_pos, buffer_size - buf_pos, ", ");
                if (buf_pos >= buffer_size - 10) {
                    fwrite(output_buffer, 1, buf_pos, stdout);
                    buf_pos = 0;
                }
            }
            buf_pos += snprintf(output_buffer + buf_pos, buffer_size - buf_pos, "%zu", result.primes[i]);
            if (buf_pos >= buffer_size - 10) {
                fwrite(output_buffer, 1, buf_pos, stdout);
                buf_pos = 0;
            }
        }
        
        if (buf_pos > 0) {
            fwrite(output_buffer, 1, buf_pos, stdout);
        }
        printf("\n");
        
        char formatted[32];
        format_number(result.count, formatted, sizeof(formatted));
        printf("Total primes: %s\n", formatted);
        
        free(output_buffer);
    } else if (args.quiet) {
        printf("%zu\n", result.count);
    } else {
        printf("No primes less than %zu\n", n);
    }
    
    /* Print statistics */
    double rate = result.count / compute_secs;
    char formatted_count[32];
    char formatted_rate[32];
    format_number(result.count, formatted_count, sizeof(formatted_count));
    format_number((size_t)rate, formatted_rate, sizeof(formatted_rate));
    
    if (result.count == 0) {
        fprintf(stderr, "Done! Generated 0 primes in %.3fs (0 primes/s).\n", compute_secs);
    } else {
        size_t largest = result.primes[result.count - 1];
        fprintf(stderr, "Done! Largest prime < %zu is %zu. Generated %s primes in %.3fs (%s primes/s).\n",
                n, largest, formatted_count, compute_secs, formatted_rate);
    }
    
    free_prime_result(&result);
    return 0;
}
