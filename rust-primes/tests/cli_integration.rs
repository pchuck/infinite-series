//! Integration tests for primes_cli binary
//!
//! Run with: cargo test --test cli_integration

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn test_basic_prime_generation() {
    let mut cmd = Command::cargo_bin("primes_cli").unwrap();
    cmd.args(["-n", "10", "--quiet"])
        .assert()
        .success()
        .stdout("4\n");
}

#[test]
fn test_quiet_mode_outputs_only_count() {
    let mut cmd = Command::cargo_bin("primes_cli").unwrap();
    cmd.args(["-n", "100", "--quiet"])
        .assert()
        .success()
        .stdout("25\n");
}

#[test]
fn test_verbose_mode_outputs_primes() {
    let mut cmd = Command::cargo_bin("primes_cli").unwrap();
    cmd.args(["-n", "10"])
        .assert()
        .success()
        .stdout(predicate::str::contains("2"))
        .stdout(predicate::str::contains("3"))
        .stdout(predicate::str::contains("5"))
        .stdout(predicate::str::contains("7"));
}

#[test]
fn test_help_flag() {
    let mut cmd = Command::cargo_bin("primes_cli").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "High-performance prime number generator",
        ))
        .stdout(predicate::str::contains("-n"))
        .stdout(predicate::str::contains("--quiet"))
        .stdout(predicate::str::contains("--parallel"));
}

#[test]
fn test_version_flag() {
    let mut cmd = Command::cargo_bin("primes_cli").unwrap();
    cmd.arg("--version")
        .assert()
        .success()
        .stdout(predicate::str::contains("primes"));
}

#[test]
fn test_segment_size_validation() {
    let mut cmd = Command::cargo_bin("primes_cli").unwrap();
    cmd.args(["-n", "100", "--segment", "0"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("segment").or(predicate::str::contains("Segment")));
}

#[test]
fn test_parallel_flag_below_threshold() {
    let mut cmd = Command::cargo_bin("primes_cli").unwrap();
    cmd.args(["-n", "1000", "-p", "--quiet"])
        .assert()
        .success()
        .stdout("168\n");
}

#[test]
fn test_custom_segment_size() {
    let mut cmd = Command::cargo_bin("primes_cli").unwrap();
    cmd.args(["-n", "1000", "--segment", "100", "--quiet"])
        .assert()
        .success()
        .stdout("168\n");
}

#[test]
fn test_no_primes_below_2() {
    let mut cmd = Command::cargo_bin("primes_cli").unwrap();
    cmd.args(["-n", "2", "--quiet"])
        .assert()
        .success()
        .stdout(predicate::str::contains("0").or(predicate::str::contains("No primes")));
}

#[test]
fn test_large_input() {
    // Test with 1 million - should complete in reasonable time
    let mut cmd = Command::cargo_bin("primes_cli").unwrap();
    cmd.args(["-n", "1000000", "--quiet"])
        .assert()
        .success()
        .stdout("78498\n");
}

#[test]
fn test_workers_flag() {
    let mut cmd = Command::cargo_bin("primes_cli").unwrap();
    cmd.args(["-n", "10000", "-w", "2", "--quiet"])
        .assert()
        .success()
        .stdout("1229\n");
}

#[test]
fn test_combined_flags() {
    let mut cmd = Command::cargo_bin("primes_cli").unwrap();
    cmd.args(["-n", "100000", "--segment", "10000", "-w", "4", "--quiet"])
        .assert()
        .success()
        .stdout("9592\n");
}
