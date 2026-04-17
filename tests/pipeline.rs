//! End-to-end pipeline tests in a `TempDir`.

use codetwin::config::Config;
use codetwin::pipeline;
use tempfile::TempDir;

#[test]
fn discover_walks_source_dirs_and_sorts_output() {
    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src");
    std::fs::create_dir(&src).unwrap();
    std::fs::write(src.join("b.rs"), "").unwrap();
    std::fs::write(src.join("a.rs"), "").unwrap();

    let config = Config {
        source_dirs: vec![src.clone()],
        ..Config::default()
    };

    let files = pipeline::discover(&config).unwrap();
    assert_eq!(files.len(), 2);
    assert!(
        files.windows(2).all(|w| w[0] <= w[1]),
        "files must be sorted"
    );
}

#[test]
fn discover_tolerates_missing_source_dir() {
    let dir = TempDir::new().unwrap();
    let config = Config {
        source_dirs: vec![dir.path().join("does-not-exist")],
        ..Config::default()
    };

    let files = pipeline::discover(&config).unwrap();
    assert!(files.is_empty());
}

#[test]
#[ignore = "touches the real filesystem; run with --include-ignored"]
fn gen_end_to_end_produces_output_file() {
    use codetwin::pipeline::GenOptions;

    let dir = TempDir::new().unwrap();
    let src = dir.path().join("src");
    std::fs::create_dir_all(&src).unwrap();
    std::fs::write(src.join("main.rs"), "fn main() {}\n").unwrap();
    std::fs::write(dir.path().join("Cargo.toml"), "[package]\n").unwrap();

    let out = dir.path().join("docs/architecture.md");
    let config = Config {
        source_dirs: vec![src],
        output_file: out.clone(),
        ..Config::default()
    };

    let prev = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    let result = pipeline::run(&config, &GenOptions::default(), false);
    std::env::set_current_dir(prev).unwrap();

    result.unwrap();
    assert!(out.exists(), "output file must be written");
}

// TODO(Phase 1.e): exclude_patterns should be honoured by discover.
// TODO(Phase 1.d): merge de-duplication test once the real merge lands.
