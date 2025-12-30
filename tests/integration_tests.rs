use std::process::Command;

fn run_calc(args: &[&str]) -> (String, bool) {
    let output = Command::new("cargo")
        .arg("run")
        .arg("--quiet")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to execute command");

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    (stdout, output.status.success())
}

#[test]
fn test_simple_addition() {
    let (output, success) = run_calc(&["2h", "+", "30m"]);
    assert!(success);
    assert_eq!(output, "2h 30m 0s");
}

#[test]
fn test_subtraction() {
    let (output, success) = run_calc(&["3h", "-", "45m"]);
    assert!(success);
    assert_eq!(output, "2h 15m 0s");
}

#[test]
fn test_compound_duration() {
    let (output, success) = run_calc(&["1h", "30m", "+", "2h", "15m"]);
    assert!(success);
    assert_eq!(output, "3h 45m 0s");
}

#[test]
fn test_multiplication() {
    let (output, success) = run_calc(&["2h", "*", "3"]);
    assert!(success);
    assert_eq!(output, "6h 0m 0s");
}

#[test]
fn test_division() {
    let (output, success) = run_calc(&["5h", "/", "2"]);
    assert!(success);
    assert_eq!(output, "2h 30m 0s");
}

#[test]
fn test_minutes_output() {
    let (output, success) = run_calc(&["2h", "30m", "--minutes"]);
    assert!(success);
    assert_eq!(output, "150 minutes");
}

#[test]
fn test_hours_output() {
    let (output, success) = run_calc(&["2h", "30m", "--hours"]);
    assert!(success);
    assert_eq!(output, "2.50 hours");
}

#[test]
fn test_iso_output() {
    let (output, success) = run_calc(&["2h", "30m", "--iso"]);
    assert!(success);
    assert_eq!(output, "PT2H30M0S");
}

#[test]
fn test_chain_operations() {
    let (output, success) = run_calc(&["8h", "-", "30m", "+", "1h"]);
    assert!(success);
    assert_eq!(output, "8h 30m 0s");
}

#[test]
fn test_days_and_weeks() {
    let (output, success) = run_calc(&["1w", "+", "2d"]);
    assert!(success);
    assert_eq!(output, "1w 2d 0h 0m 0s");
}

#[test]
fn test_division_by_zero() {
    let (_, success) = run_calc(&["5h", "/", "0"]);
    assert!(!success);
}
