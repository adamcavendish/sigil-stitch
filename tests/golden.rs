use std::path::PathBuf;

/// Assert that the output matches the golden file.
/// If BLESS=1 is set, update the golden file instead.
pub fn assert_golden(name: &str, actual: &str) {
    let golden_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden");
    let golden_path = golden_dir.join(name);

    if std::env::var("BLESS").is_ok() {
        // Update golden file.
        if let Some(parent) = golden_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&golden_path, actual).unwrap();
        return;
    }

    if !golden_path.exists() {
        // First run, create the golden file.
        if let Some(parent) = golden_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        std::fs::write(&golden_path, actual).unwrap();
        eprintln!(
            "Golden file created: {}. Re-run to verify.",
            golden_path.display()
        );
        return;
    }

    let expected = std::fs::read_to_string(&golden_path).unwrap();
    if actual != expected {
        // Show diff.
        eprintln!("Golden file mismatch: {}", golden_path.display());
        eprintln!("--- expected ---");
        eprintln!("{expected}");
        eprintln!("--- actual ---");
        eprintln!("{actual}");
        eprintln!("---");
        eprintln!("Run with BLESS=1 to update golden files.");
        panic!("Golden file mismatch: {}", golden_path.display());
    }
}
