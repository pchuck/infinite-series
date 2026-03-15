package net.ultrametrics.primes;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.DisplayName;

import static org.junit.jupiter.api.Assertions.*;

class PrimeGeneratorTest {

    private boolean isPrime(int n) {
        if (n < 2) return false;
        if (n < 4) return true;
        if (n % 2 == 0 || n % 3 == 0) return false;
        for (int i = 5; i * i <= n; i += 6) {
            if (n % i == 0 || n % (i + 2) == 0) return false;
        }
        return true;
    }

    @Test
    @DisplayName("Test small input: n=10")
    void testSmallInput() {
        int[] result = PrimeGenerator.generatePrimes(10, false, false, null);
        assertArrayEquals(new int[]{2, 3, 5, 7}, result);
    }

    @Test
    @DisplayName("Test boundary case: n=2")
    void testNEquals2() {
        int[] result = PrimeGenerator.generatePrimes(2, false, false, null);
        assertEquals(0, result.length);
    }

    @Test
    @DisplayName("Test large input: n=30")
    void testLargeInput() {
        int[] expected = {2, 3, 5, 7, 11, 13, 17, 19, 23, 29};
        int[] result = PrimeGenerator.generatePrimes(30, false, false, null);
        assertArrayEquals(expected, result);
    }

    @Test
    @DisplayName("Test single digit: n=5")
    void testSingleDigit() {
        int[] result = PrimeGenerator.generatePrimes(5, false, false, null);
        assertArrayEquals(new int[]{2, 3}, result);
    }

    @Test
    @DisplayName("Test edge cases: n=0, n=1, n=2")
    void testEdgeCases() {
        assertEquals(0, PrimeGenerator.generatePrimes(0, false, false, null).length);
        assertEquals(0, PrimeGenerator.generatePrimes(1, false, false, null).length);
        assertEquals(0, PrimeGenerator.generatePrimes(2, false, false, null).length);
    }

    @Test
    @DisplayName("Test n=3")
    void testNEquals3() {
        int[] result = PrimeGenerator.generatePrimes(3, false, false, null);
        assertArrayEquals(new int[]{2}, result);
    }

    @Test
    @DisplayName("Test n=4")
    void testNEquals4() {
        int[] result = PrimeGenerator.generatePrimes(4, false, false, null);
        assertArrayEquals(new int[]{2, 3}, result);
    }

    @Test
    @DisplayName("Test negative input raises error")
    void testNegativeInputRaisesError() {
        assertThrows(IllegalArgumentException.class, () -> {
            PrimeGenerator.generatePrimes(-1, false, false, null);
        });
    }

    @Test
    @DisplayName("Test large prime verification")
    void testLargePrimeVerification() {
        int[] primes = PrimeGenerator.generatePrimes(100, false, false, null);
        assertTrue(contains(primes, 97));
    }

    @Test
    @DisplayName("Test no composites")
    void testNoComposites() {
        int[] primes = PrimeGenerator.generatePrimes(50, false, false, null);
        for (int p : primes) {
            assertTrue(isPrime(p), p + " is not prime");
        }
    }

    @Test
    @DisplayName("Test consecutive primes")
    void testConsecutivePrimes() {
        int[] result = PrimeGenerator.generatePrimes(20, false, false, null);
        int[] expected = {2, 3, 5, 7, 11, 13, 17, 19};
        assertArrayEquals(expected, result);
    }

    @Test
    @DisplayName("Test force algorithm: classic")
    void testForceAlgorithmClassic() {
        int[] result = PrimeGenerator.generatePrimes(10000, false, false, "classic");
        int[] expected = PrimeGenerator.sieveOfEratosthenes(10000);
        assertArrayEquals(expected, result);
    }

    @Test
    @DisplayName("Test force algorithm: segmented")
    void testForceAlgorithmSegmented() {
        int[] result = PrimeGenerator.generatePrimes(10000, false, false, "segmented");
        int[] expected = PrimeGenerator.sieveOfEratosthenes(10000);
        assertArrayEquals(expected, result);
    }

    private boolean contains(int[] arr, int value) {
        for (int v : arr) {
            if (v == value) return true;
        }
        return false;
    }
}

class SieveOfEratosthenesTest {

    @Test
    @DisplayName("Test basic functionality")
    void testBasicFunctionality() {
        int[] result = PrimeGenerator.sieveOfEratosthenes(10);
        assertArrayEquals(new int[]{2, 3, 5, 7}, result);
    }

    @Test
    @DisplayName("Test empty result")
    void testEmptyResult() {
        assertEquals(0, PrimeGenerator.sieveOfEratosthenes(0).length);
        assertEquals(0, PrimeGenerator.sieveOfEratosthenes(1).length);
        assertEquals(0, PrimeGenerator.sieveOfEratosthenes(2).length);
    }

    @Test
    @DisplayName("Test n=3")
    void testNEquals3() {
        assertArrayEquals(new int[]{2}, PrimeGenerator.sieveOfEratosthenes(3));
    }

    @Test
    @DisplayName("Test n=4")
    void testNEquals4() {
        assertArrayEquals(new int[]{2, 3}, PrimeGenerator.sieveOfEratosthenes(4));
    }

    @Test
    @DisplayName("Test negative input raises error")
    void testNegativeInputRaisesError() {
        assertThrows(IllegalArgumentException.class, () -> {
            PrimeGenerator.sieveOfEratosthenes(-1);
        });
    }

    @Test
    @DisplayName("Test no composites returned")
    void testNoCompositesReturned() {
        int[] primes = PrimeGenerator.sieveOfEratosthenes(200);
        for (int p : primes) {
            assertTrue(isPrime(p), p + " is not prime");
        }
    }

    private boolean isPrime(int n) {
        if (n < 2) return false;
        if (n < 4) return true;
        if (n % 2 == 0 || n % 3 == 0) return false;
        for (int i = 5; i * i <= n; i += 6) {
            if (n % i == 0 || n % (i + 2) == 0) return false;
        }
        return true;
    }
}

class SegmentedSieveTest {

    @Test
    @DisplayName("Test basic functionality")
    void testBasicFunctionality() {
        int[] result = PrimeGenerator.segmentedSieve(10, 10, null);
        assertArrayEquals(new int[]{2, 3, 5, 7}, result);
    }

    @Test
    @DisplayName("Test matches classic for small n")
    void testMatchesClassicForSmallN() {
        int[] testValues = {100, 500, 1000, 5000};
        for (int n : testValues) {
            int[] expected = PrimeGenerator.sieveOfEratosthenes(n);
            int[] result = PrimeGenerator.segmentedSieve(n, 100, null);
            assertArrayEquals(expected, result, "Failed for n=" + n);
        }
    }

    @Test
    @DisplayName("Test empty result")
    void testEmptyResult() {
        assertEquals(0, PrimeGenerator.segmentedSieve(0, 10, null).length);
        assertEquals(0, PrimeGenerator.segmentedSieve(1, 10, null).length);
        assertEquals(0, PrimeGenerator.segmentedSieve(2, 10, null).length);
    }

    @Test
    @DisplayName("Test negative input raises error")
    void testNegativeInputRaisesError() {
        assertThrows(IllegalArgumentException.class, () -> {
            PrimeGenerator.segmentedSieve(-1, 10, null);
        });
    }

    @Test
    @DisplayName("Test custom segment size")
    void testCustomSegmentSize() {
        int[] expected = PrimeGenerator.sieveOfEratosthenes(100);
        for (int segSize : new int[]{1, 10, 100, 1000}) {
            int[] result = PrimeGenerator.segmentedSieve(100, segSize, null);
            assertArrayEquals(expected, result, "Failed for segment size=" + segSize);
        }
    }
}

class ParallelSieveTest {

    @Test
    @DisplayName("Test parallel matches sequential")
    void testParallelMatchesSequential() {
        int[] expected = PrimeGenerator.segmentedSieve(5000, 100, null);
        int[] result = PrimeGenerator.parallelSegmentedSieve(5000, 2, 100, null);
        assertArrayEquals(expected, result);
    }

    @Test
    @DisplayName("Test parallel with various worker counts")
    void testParallelWithVariousWorkerCounts() {
        int n = 10000;
        int[] expected = PrimeGenerator.segmentedSieve(n, 100, null);

        for (int workers : new int[]{1, 2, 4}) {
            int[] result = PrimeGenerator.parallelSegmentedSieve(n, workers, 100, null);
            assertArrayEquals(expected, result, "Failed for workers=" + workers);
        }
    }

    @Test
    @DisplayName("Test parallel edge cases")
    void testParallelEdgeCases() {
        for (int n : new int[]{0, 1, 2}) {
            int[] result = PrimeGenerator.parallelSegmentedSieve(n, 2, 100, null);
            assertEquals(0, result.length, "Failed for n=" + n);
        }
    }

    @Test
    @DisplayName("Test negative input raises error")
    void testNegativeInputRaisesError() {
        assertThrows(IllegalArgumentException.class, () -> {
            PrimeGenerator.parallelSegmentedSieve(-1, 2, 100, null);
        });
    }
}
