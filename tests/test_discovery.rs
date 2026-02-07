#[cfg(test)]
mod tests {
    use codetwin::config::Config;

    #[test]
    fn test_config_load() {
        let cfg = Config::load("codetwin.toml").expect("Failed to load config");
        assert!(cfg.output_file.ends_with(".md"));
        assert!(cfg.source_dirs.contains(&"src".to_string()));
        println!(
            "✅ Config loaded: output_file={}, source_dirs={:?}",
            cfg.output_file, cfg.source_dirs
        );
    }

    #[test]
    fn test_find_files_from_config() {
        use codetwin::discovery::find_source_files;
        let cfg = Config::load("codetwin.toml").expect("Failed to load config");
        let files = find_source_files(&cfg.source_dirs, &cfg.exclude_patterns)
            .expect("Failed to find files");
        assert!(!files.is_empty(), "Should find source files");
        println!(
            "✅ Found {} source files in {:?}",
            files.len(),
            cfg.source_dirs
        );
        for f in files.iter().take(5) {
            println!("  - {}", f.display());
        }
    }
}
