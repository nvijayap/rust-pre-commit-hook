use std::process::Command;

fn main() {
    println!("--- Running Pre-Commit Hooks ---");

    // 1. Run cargo fmt to check formatting
    let fmt_status = Command::new("cargo")
        .args(["fmt", "--", "--check"])
        .status()
        .expect("Failed to execute cargo fmt");

    if !fmt_status.success() {
        eprintln!("❌ Error: Code formatting issues found. Run 'cargo fmt' to fix.");
        std::process::exit(1);
    }

    // 2. Run cargo check to ensure code compiles
    let check_status = Command::new("cargo")
        .args(["check"])
        .status()
        .expect("Failed to execute cargo check");

    if !check_status.success() {
        eprintln!("❌ Error: Code does not compile. Fix errors before committing.");
        std::process::exit(1);
    }

    // 1. Run snyk test to check formatting
    let fmt_status = Command::new("cargo")
        .args(["clippy"])
        .status()
        .expect("Failed to execute cargo clippy");

    if !fmt_status.success() {
        eprintln!("❌ Error: clippy issues found. Run 'cargo clippy' to fix.");
        std::process::exit(1);
    }

    println!("\n‼️  External scripts can be called here. The scripts can trigger (security, policies, compliance, auditing, logging, notifications) checks in the background");

    println!("\n✅ All checks passed! Proceeding with commit...\n");
}
