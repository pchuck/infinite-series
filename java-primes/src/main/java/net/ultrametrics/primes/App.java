package net.ultrametrics.primes;

import java.util.Arrays;
import java.util.Scanner;

public class App {

    private static final int PARALLEL_SIEVE_THRESHOLD = 500_000_000;

    public static void main(String[] args) {
        int n = -1;
        boolean showProgress = false;
        boolean parallel = false;
        boolean quiet = false;

        for (int i = 0; i < args.length; i++) {
            switch (args[i]) {
                case "--progress":
                case "-p":
                    showProgress = true;
                    break;
                case "--parallel":
                    parallel = true;
                    break;
                case "--quiet":
                case "-q":
                    quiet = true;
                    break;
                case "--help":
                case "-h":
                    printHelp();
                    return;
                default:
                    if (!args[i].startsWith("-")) {
                        try {
                            n = Integer.parseInt(args[i]);
                        } catch (NumberFormatException e) {
                            System.err.println("Error: Invalid number '" + args[i] + "'");
                            printHelp();
                            return;
                        }
                    }
                    break;
            }
        }

        if (n == -1) {
            System.out.print("Enter a number (n): ");
            Scanner scanner = new Scanner(System.in);
            try {
                n = scanner.nextInt();
            } catch (Exception e) {
                System.out.println("Please enter a valid integer");
                return;
            }
        }

        if (n <= 2) {
            System.out.println("No primes less than " + n);
            return;
        }

        if (parallel && n < PARALLEL_SIEVE_THRESHOLD) {
            System.err.println("[WARN] --parallel ignored: n=" + n + " is below threshold " + PARALLEL_SIEVE_THRESHOLD);
        }

        long startTime = System.nanoTime();
        int[] primes = PrimeGenerator.generatePrimes(n, showProgress, parallel, null);
        long elapsed = System.nanoTime() - startTime;
        double elapsedSeconds = elapsed / 1_000_000_000.0;

        if (primes.length > 0) {
            if (!quiet) {
                System.out.print("Primes less than " + n + ": ");
                System.out.println(Arrays.toString(primes).replace("[", "").replace("]", ""));
                System.out.println("Total primes: " + primes.length);
            } else {
                System.out.println(primes.length);
            }
        } else {
            System.out.println("No primes less than " + n);
        }

        int largestPrime = primes.length > 0 ? primes[primes.length - 1] : 0;
        double primesPerSec = elapsed > 0 ? primes.length / elapsedSeconds : 0;
        System.err.printf("Done! Largest prime < %d is %d. Generated %d primes in %.3fs (%,.0f primes/s).%n",
                n, largestPrime, primes.length, elapsedSeconds, primesPerSec);
    }

    private static void printHelp() {
        System.out.println("Java Prime Generator");
        System.out.println("");
        System.out.println("Usage: java -jar java-primes.jar [OPTIONS] [n]");
        System.out.println("");
        System.out.println("Arguments:");
        System.out.println("  n                   Upper bound (exclusive). If not provided, prompts for input.");
        System.out.println("");
        System.out.println("Options:");
        System.out.println("  -p, --progress      Show progress indicator");
        System.out.println("  --parallel         Use parallel processing (for large n >= 500M)");
        System.out.println("  -q, --quiet        Only print count (no prime list)");
        System.out.println("  -h, --help         Show this help message");
    }
}
