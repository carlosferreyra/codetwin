#[cfg(test)]
mod tests {
    use codetwin::config::Config;

    #[test]
    fn test_config_load() {
        let cfg = Config::load("codetwin.toml").expect("Failed to load config");
        assert_eq!(cfg.output_dir, "docs");
        assert_eq!(cfg.main_diagram, "STRUCT.md");
        assert!(cfg.source_dirs.contains(&"src".to_string()));
        println!(
            "✅ Config loaded: output_dir={}, source_dirs={:?}",
            cfg.output_dir, cfg.source_dirs
        );
    }

    #[test]
    fn test_find_files_from_config() {
        use codetwin::discovery::find_rust_files;
        let cfg = Config::load("codetwin.toml").expect("Failed to load config");
        let files = find_rust_files(&cfg.source_dirs).expect("Failed to find files");
        assert!(!files.is_empty(), "Should find .rs files");
        println!("✅ Found {} Rust files in {:?}", files.len(), cfg.source_dirs);
        for f in files.iter().take(5) {
            println!("  - {}", f.display());
        }
    }
}
