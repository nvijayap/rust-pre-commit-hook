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

    // 3. Run "cargo clippy" ...
    // Cargo Clippy is a tool that checks Rust code for common mistakes and suggests 
    // improvements to enhance code quality and adherence to idiomatic Rust practices. 
    // It provides warnings across various categories, 
    // such as correctness, style, and performance.
    let clippy_status = Command::new("cargo")
        .args(["clippy"])
        .status()
        .expect("Failed to execute 'cargo clippy'");

    if !clippy_status.success() {
        eprintln!("❌ Error: clippy issues found. Run 'cargo clippy' to fix.");
        std::process::exit(1);
    }

    // 4. Run "cargo audit" ...
    let audit_status = Command::new("cargo")
        .args(["audit"])
        .status()
        .expect("Failed to execute 'cargo audit'");

    if !audit_status.success() {
        eprintln!("❌ Error: audit issues found. Run 'cargo audit' to fix.");
        std::process::exit(1);
    }

    println!("\n‼️  External scripts can be called here. The scripts can trigger (security, policies, compliance, auditing, logging, notifications) checks in the background");

    println!("\n✅ All checks passed! Proceeding with commit...\n");
}
