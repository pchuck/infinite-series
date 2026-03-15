package net.ultrametrics.primes;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.Collections;
import java.util.List;
import java.util.concurrent.ForkJoinPool;
import java.util.concurrent.RecursiveTask;
import java.util.concurrent.atomic.AtomicInteger;

public class PrimeGenerator {

    private static final int DEFAULT_SEGMENT_SIZE = 1_000_000;
    private static final int SEGMENTED_SIEVE_THRESHOLD = 10_000_000;
    private static final int PARALLEL_SIEVE_THRESHOLD = 500_000_000;

    @FunctionalInterface
    public interface ProgressCallback {
        void accept(int count);
    }

    private static int[] sieveSegmentOddOnly(int low, int high, int[] basePrimes, byte[] isPrime) {
        List<Integer> primesList = new ArrayList<>();

        if (low <= 2 && high > 2) {
            primesList.add(2);
        }

        int oddLow = Math.max(low, 3);
        if (oddLow % 2 == 0) {
            oddLow++;
        }

        if (oddLow >= high) {
            return primesList.stream().mapToInt(Integer::intValue).toArray();
        }

        int segLen = (high - oddLow + 1) / 2;
        if (segLen <= 0) {
            return primesList.stream().mapToInt(Integer::intValue).toArray();
        }

        Arrays.fill(isPrime, 0, segLen, (byte) 1);

        for (int p : basePrimes) {
            int start = ((oddLow + p - 1) / p) * p;
            if (start < p * p) {
                start = p * p;
            }
            if (start % 2 == 0) {
                start += p;
            }

            if (start >= high) {
                continue;
            }

            int adjustedStart = (start - oddLow) / 2;
            int step = p;
            for (int j = adjustedStart; j < segLen; j += step) {
                isPrime[j] = 0;
            }
        }

        for (int i = 0; i < segLen; i++) {
            if (isPrime[i] == 1) {
                primesList.add(oddLow + 2 * i);
            }
        }

        return primesList.stream().mapToInt(Integer::intValue).toArray();
    }

    public static int[] sieveOfEratosthenes(int n) {
        if (n < 0) {
            throw new IllegalArgumentException("n must be non-negative, got " + n);
        }

        if (n <= 2) {
            return new int[0];
        }

        if (n <= 3) {
            return new int[]{2};
        }

        int sieveSize = (n - 3 + 1) / 2;
        byte[] sieve = new byte[sieveSize];
        Arrays.fill(sieve, (byte) 1);

        int maxCheck = (int) Math.sqrt(n);
        for (int current = 3; current <= maxCheck; current += 2) {
            int idx = (current - 3) / 2;
            if (sieve[idx] == 1) {
                int startIdx = (current * current - 3) / 2;
                int step = current;
                for (int j = startIdx; j < sieveSize; j += step) {
                    sieve[j] = 0;
                }
            }
        }

        List<Integer> primesList = new ArrayList<>();
        primesList.add(2);

        for (int i = 0; i < sieveSize; i++) {
            if (sieve[i] == 1) {
                primesList.add(2 * i + 3);
            }
        }

        return primesList.stream().mapToInt(Integer::intValue).toArray();
    }

    public static int[] segmentedSieve(int n, int segmentSize, ProgressCallback progressCallback) {
        if (n < 0) {
            throw new IllegalArgumentException("n must be non-negative, got " + n);
        }

        if (n <= 2) {
            return new int[0];
        }

        if (segmentSize <= 0) {
            segmentSize = DEFAULT_SEGMENT_SIZE;
        }

        int baseLimit = (int) Math.sqrt(n);
        int[] allBasePrimes = sieveOfEratosthenes(baseLimit + 1);

        List<Integer> basePrimesOddList = new ArrayList<>();
        for (int p : allBasePrimes) {
            if (p > 2) {
                basePrimesOddList.add(p);
            }
        }
        int[] basePrimesOdd = basePrimesOddList.stream().mapToInt(Integer::intValue).toArray();

        int segments = (n + segmentSize - 1) / segmentSize;
        List<int[]> allSegmentPrimes = new ArrayList<>(segments);

        byte[] isPrime = new byte[segmentSize];

        for (int segIdx = 0; segIdx < segments; segIdx++) {
            int low = segIdx * segmentSize;
            int high = Math.min(low + segmentSize, n);

            if (high <= 2) {
                if (progressCallback != null) {
                    progressCallback.accept(1);
                }
                continue;
            }

            int[] segPrimes = sieveSegmentOddOnly(low, high, basePrimesOdd, isPrime);
            allSegmentPrimes.add(segPrimes);

            if (progressCallback != null) {
                progressCallback.accept(1);
            }
        }

        int totalPrimes = 0;
        for (int[] seg : allSegmentPrimes) {
            totalPrimes += seg.length;
        }

        int[] result = new int[totalPrimes];
        int offset = 0;
        for (int[] seg : allSegmentPrimes) {
            System.arraycopy(seg, 0, result, offset, seg.length);
            offset += seg.length;
        }

        return result;
    }

    private static class SegmentTask extends RecursiveTask<int[]> {
        private final int startSeg;
        private final int endSeg;
        private final int n;
        private final int segmentSize;
        private final int[] basePrimesOdd;

        SegmentTask(int startSeg, int endSeg, int n, int segmentSize, int[] basePrimesOdd) {
            this.startSeg = startSeg;
            this.endSeg = endSeg;
            this.n = n;
            this.segmentSize = segmentSize;
            this.basePrimesOdd = basePrimesOdd;
        }

        @Override
        protected int[] compute() {
            byte[] isPrime = new byte[segmentSize];
            List<int[]> results = new ArrayList<>();

            for (int segIdx = startSeg; segIdx < endSeg; segIdx++) {
                int low = segIdx * segmentSize;
                int high = Math.min(low + segmentSize, n);

                if (high <= 2) {
                    continue;
                }

                int[] segPrimes = sieveSegmentOddOnly(low, high, basePrimesOdd, isPrime);
                results.add(segPrimes);
            }

            int totalPrimes = 0;
            for (int[] seg : results) {
                totalPrimes += seg.length;
            }

            int[] result = new int[totalPrimes];
            int offset = 0;
            for (int[] seg : results) {
                System.arraycopy(seg, 0, result, offset, seg.length);
                offset += seg.length;
            }

            return result;
        }
    }

    public static int[] parallelSegmentedSieve(int n, int numWorkers, int segmentSize, ProgressCallback progressCallback) {
        if (n < 0) {
            throw new IllegalArgumentException("n must be non-negative, got " + n);
        }

        if (n <= 2) {
            return new int[0];
        }

        if (segmentSize <= 0) {
            segmentSize = DEFAULT_SEGMENT_SIZE;
        }

        if (numWorkers <= 0) {
            numWorkers = Runtime.getRuntime().availableProcessors();
        }

        int baseLimit = (int) Math.sqrt(n);
        int[] allBasePrimes = sieveOfEratosthenes(baseLimit + 1);

        List<Integer> basePrimesOddList = new ArrayList<>();
        for (int p : allBasePrimes) {
            if (p > 2) {
                basePrimesOddList.add(p);
            }
        }
        int[] basePrimesOdd = basePrimesOddList.stream().mapToInt(Integer::intValue).toArray();

        int segments = (n + segmentSize - 1) / segmentSize;
        int actualWorkers = Math.min(numWorkers, segments);

        ForkJoinPool pool = ForkJoinPool.commonPool();
        List<RecursiveTask<int[]>> tasks = new ArrayList<>();

        int chunkSize = (segments + actualWorkers - 1) / actualWorkers;

        for (int workerIdx = 0; workerIdx < actualWorkers; workerIdx++) {
            int startSeg = workerIdx * chunkSize;
            int endSeg = Math.min(startSeg + chunkSize, segments);

            if (startSeg >= segments) {
                break;
            }

            SegmentTask task = new SegmentTask(startSeg, endSeg, n, segmentSize, basePrimesOdd);
            tasks.add(task);
            pool.execute(task);
        }

        List<int[]> allResults = new ArrayList<>();
        int completedSegments = 0;
        AtomicInteger lastReported = new AtomicInteger(0);

        if (progressCallback != null) {
            Thread progressThread = new Thread(() -> {
                while (completedSegments < segments) {
                    try {
                        Thread.sleep(100);
                    } catch (InterruptedException e) {
                        break;
                    }

                    int currentCompleted = 0;
                    for (RecursiveTask<int[]> task : tasks) {
                        if (task.isDone()) {
                            int taskStartSeg = (tasks.indexOf(task) * chunkSize);
                            int taskEndSeg = Math.min(taskStartSeg + chunkSize, segments);
                            int taskSegments = taskEndSeg - taskStartSeg;
                            currentCompleted += taskSegments;
                        }
                    }

                    int delta = currentCompleted - lastReported.get();
                    if (delta > 0) {
                        progressCallback.accept(delta);
                        lastReported.set(currentCompleted);
                    }
                }
            });
            progressThread.start();
        }

        for (RecursiveTask<int[]> task : tasks) {
            int[] result = task.join();
            allResults.add(result);
        }

        int totalPrimes = 0;
        for (int[] seg : allResults) {
            totalPrimes += seg.length;
        }

        int[] finalResult = new int[totalPrimes];
        int offset = 0;
        for (int[] seg : allResults) {
            System.arraycopy(seg, 0, finalResult, offset, seg.length);
            offset += seg.length;
        }

        Arrays.sort(finalResult);

        return finalResult;
    }

    public static int[] generatePrimes(int n, boolean showProgress, boolean parallel, String forceAlgorithm) {
        if (n < 0) {
            throw new IllegalArgumentException("n must be non-negative, got " + n);
        }

        if (n <= 2) {
            return new int[0];
        }

        ProgressCallback progressCallback = showProgress ? new ProgressCallback() {
            private int lastProgress = 0;
            private long startTime = System.currentTimeMillis();
            private static final int UPDATE_INTERVAL_MS = 100;
            private long lastUpdate = 0;

            @Override
            public void accept(int count) {
                long now = System.currentTimeMillis();
                if (now - lastUpdate < UPDATE_INTERVAL_MS && lastProgress + count < getTotal()) {
                    return;
                }
                lastUpdate = now;
                lastProgress += count;
                render(lastProgress, getTotal());
            }

            private int getTotal() {
                return (n + DEFAULT_SEGMENT_SIZE - 1) / DEFAULT_SEGMENT_SIZE;
            }

            private void render(int completed, int total) {
                if (total == 0) return;
                double percent = Math.min(1.0, (double) completed / total);
                int barWidth = 40;
                int filled = (int) (percent * barWidth);
                StringBuilder bar = new StringBuilder();
                for (int i = 0; i < filled; i++) bar.append('=');
                for (int i = filled; i < barWidth; i++) bar.append(' ');

                long elapsed = System.currentTimeMillis() - startTime;
                double rate = elapsed > 0 ? (completed * 1000.0 / elapsed) : 0;

                String rateStr;
                if (rate >= 1_000_000) {
                    rateStr = String.format("%.1fM/s", rate / 1_000_000);
                } else if (rate >= 1_000) {
                    rateStr = String.format("%.1fK/s", rate / 1_000);
                } else {
                    rateStr = String.format("%.0f/s", rate);
                }

                int remaining = total - completed;
                String etaStr;
                if (rate > 0 && remaining > 0) {
                    double etaSecs = remaining / rate;
                    if (etaSecs >= 3600) {
                        etaStr = String.format("%dh%02dm", (int) (etaSecs / 3600), (int) ((etaSecs % 3600) / 60));
                    } else if (etaSecs >= 60) {
                        etaStr = String.format("%dm%ds", (int) (etaSecs / 60), (int) (etaSecs % 60));
                    } else {
                        etaStr = String.format("%ds", (int) etaSecs);
                    }
                } else {
                    etaStr = "0s";
                }

                System.err.print(String.format("\rGenerating primes: [%s] %3.0f%% | %d/%d | %s | eta %s    ",
                        bar.toString(), percent * 100, completed, total, rateStr, etaStr));
                System.err.flush();
            }
        } : null;

        boolean useSegmented = "segmented".equals(forceAlgorithm) || "parallel".equals(forceAlgorithm) ||
                (forceAlgorithm == null && n >= SEGMENTED_SIEVE_THRESHOLD);

        boolean useParallel = "parallel".equals(forceAlgorithm) ||
                (parallel && n >= PARALLEL_SIEVE_THRESHOLD);

        int[] primes;
        if (useSegmented) {
            if (useParallel) {
                primes = parallelSegmentedSieve(n, 0, DEFAULT_SEGMENT_SIZE, progressCallback);
            } else {
                primes = segmentedSieve(n, DEFAULT_SEGMENT_SIZE, progressCallback);
            }
        } else {
            primes = sieveOfEratosthenes(n);
        }

        if (showProgress && progressCallback != null) {
            System.err.println();
        }

        return primes;
    }
}
