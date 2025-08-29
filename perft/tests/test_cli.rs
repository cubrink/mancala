use rstest::rstest;
use std::process::Command;

// Perft(1) = 6
// Perft(2) = 35
// Perft(3) = 184
// Perft(4) = 918
// Perft(5) = 4405
// Perft(6) = 20830
// Perft(7) = 97014
// Perft(8) = 447866
// Perft(9) = 2049412
// Perft(10) = 9326089

#[rstest]
#[case::perft_0("0", "1")]
#[case::perft_1("1", "6")]
#[case::perft_2("2", "35")]
#[case::perft_3("3", "184")]
#[case::perft_4("4", "918")]
#[case::perft_5("5", "4405")]
#[case::perft_6("6", "20830")]
#[case::perft_7("7", "97014")]
#[case::perft_8("8", "447866")]
#[case::perft_9("9", "2049412")]
#[case::perft_10("10", "9326089")]
fn test_perft(#[case] depth: String, #[case] total: usize) {
    let output = Command::new("cargo")
        .args(&["run", "--bin", "perft", "--", &depth])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let gt = format!("Perft({0}) = {1}\n", depth, total);
    assert_eq!(output_str, gt);
}

#[cfg(feature = "parallel")]
#[rstest]
#[case::perft_0("0", "1")]
#[case::perft_1("1", "6")]
#[case::perft_2("2", "35")]
#[case::perft_3("3", "184")]
#[case::perft_4("4", "918")]
#[case::perft_5("5", "4405")]
#[case::perft_6("6", "20830")]
#[case::perft_7("7", "97014")]
#[case::perft_8("8", "447866")]
#[case::perft_9("9", "2049412")]
#[case::perft_10("10", "9326089")]
fn test_perft_parallel(#[case] depth: String, #[case] total: usize) {
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--features",
            "parallel",
            "--bin",
            "perft",
            &depth,
            "-t",
            "2",
        ])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let output_str = String::from_utf8_lossy(&output.stdout).to_string();
    let gt = format!("Perft({0}) = {1}\n", depth, total);
    assert_eq!(output_str, gt);
}
