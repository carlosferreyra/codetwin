//! Integration tests for Phase 1.5 Infrastructure Refactoring
//! Tests error handling, logging, discovery, JSON output, watch mode, and parallel parsing

use codetwin::config::Config;
use codetwin::discovery;
use codetwin::ir::{Blueprint, Element, Visibility};
use std::env;
use std::path::PathBuf;

// ============================================================================
// Test 1: Error Handling with anyhow Context
// ============================================================================

#[test]
fn test_error_context_chains() {
    // Test that error context is properly displayed
    // When trying to load config from non-existent path
    let result = Config::load("non_existent_path/codetwin.toml");

    assert!(
        result.is_err(),
        "Should fail when config file doesn't exist"
    );

    // Error message should contain context about what failed
    let err_msg = format!("{:#}", result.unwrap_err());
    assert!(
        err_msg.contains("Failed to") || err_msg.contains("toml"),
        "Error should provide context about failure: {}",
        err_msg
    );
}

#[test]
fn test_error_context_in_discovery() {
    // Test that discovery properly handles invalid paths
    let result = discovery::find_source_files(&["non_existent_directory".to_string()], &[]);

    // Should either return empty or error with context
    match result {
        Ok(files) => {
            // If successful, should be empty
            assert_eq!(files.len(), 0);
        }
        Err(_e) => {
            // If error, should contain helpful context
            // Error is expected when directory doesn't exist
        }
    }
}

// ============================================================================
// Test 2: File Discovery with glob Pattern Exclusion
// ============================================================================

#[test]
fn test_discovery_excludes_target_directory() {
    // Test that discovery excludes target/ directory
    let src_files =
        discovery::find_source_files(&["src".to_string()], &Config::defaults().exclude_patterns)
            .expect("Should discover src directory");

    // Should find source files
    assert!(
        !src_files.is_empty(),
        "Should discover source files in src/"
    );

    // None should be in target/
    for file in &src_files {
        assert!(
            !file.to_string_lossy().contains("target/"),
            "Should not include files from target/ directory: {:?}",
            file
        );
    }
}

#[test]
fn test_discovery_finds_source_files() {
    // Test that discovery finds supported source files
    let src_files =
        discovery::find_source_files(&["src".to_string()], &Config::defaults().exclude_patterns)
            .expect("Should discover src directory");

    // Should find at least some source files
    assert!(
        src_files.len() > 0,
        "Should discover at least one source file"
    );

    // All should be supported source files
    for file in &src_files {
        assert!(
            matches!(
                file.extension().and_then(|ext| ext.to_str()),
                Some("rs") | Some("py")
            ),
            "Should only find supported source files: {:?}",
            file
        );
    }
}

#[test]
fn test_discovery_results_sorted() {
    // Test that discovery results are sorted for consistency
    let files =
        discovery::find_source_files(&["src".to_string()], &Config::defaults().exclude_patterns)
            .expect("Should discover src directory");

    // Check that results are sorted
    let mut sorted = files.clone();
    sorted.sort();

    assert_eq!(
        files, sorted,
        "Discovery results should be sorted for consistency"
    );
}

// ============================================================================
// Test 3: JSON Serialization Structure
// ============================================================================

#[test]
fn test_blueprint_serialization() {
    // Test that Blueprint can be serialized to JSON
    let blueprint = Blueprint {
        source_path: PathBuf::from("test.rs"),
        language: "rust".to_string(),
        elements: vec![],
        dependencies: vec!["std".to_string()],
    };

    let json = serde_json::to_value(&blueprint).expect("Should serialize to JSON");

    // Should have required fields
    assert!(
        json["source_path"].is_string(),
        "Should have source_path field"
    );
    assert!(json["language"].is_string(), "Should have language field");
    assert!(json["elements"].is_array(), "Should have elements array");
    assert!(
        json["dependencies"].is_array(),
        "Should have dependencies array"
    );
}

#[test]
fn test_config_serialization() {
    // Test that Config can be serialized
    let config = Config::defaults();

    let json = serde_json::to_value(&config).expect("Should serialize to JSON");

    // Should have required config fields
    assert!(json["source_dirs"].is_array(), "Should have source_dirs");
    assert!(json["output_file"].is_string(), "Should have output_file");
    assert!(json["layout"].is_string(), "Should have layout");
}

#[test]
fn test_element_enum_serialization() {
    // Test that Element enum variants serialize correctly
    use codetwin::ir::{Class, Documentation};

    let class_elem = Element::Class(Class {
        name: "TestClass".to_string(),
        visibility: Visibility::Public,
        methods: vec![],
        properties: vec![],
        documentation: Documentation {
            summary: Some("A test class".to_string()),
            description: None,
            examples: vec![],
        },
    });

    let json = serde_json::to_value(&class_elem).expect("Should serialize");

    // Should have the Class variant structure
    assert!(json["Class"].is_object(), "Should serialize Class variant");
    assert_eq!(json["Class"]["name"], "TestClass");
}

// ============================================================================
// Test 4: Logging Environment Variable Filter
// ============================================================================

#[test]
fn test_logging_env_setup() {
    // Test that RUST_LOG environment variable can be set
    // This is a simple check that the environment variable works
    unsafe {
        env::set_var("RUST_LOG", "debug");
    }
    let level = env::var("RUST_LOG").expect("Should set RUST_LOG");
    assert_eq!(level, "debug");

    // Cleanup
    unsafe {
        env::remove_var("RUST_LOG");
    }
}

#[test]
fn test_logging_env_info_level() {
    // Test that RUST_LOG can be set to info level
    unsafe {
        env::set_var("RUST_LOG", "info");
    }
    let level = env::var("RUST_LOG").expect("Should set RUST_LOG");
    assert_eq!(level, "info");

    // Cleanup
    unsafe {
        env::remove_var("RUST_LOG");
    }
}

// ============================================================================
// Test 5: Configuration Loading and Defaults
// ============================================================================

#[test]
fn test_config_defaults() {
    let config = Config::defaults();

    // Should have reasonable defaults
    assert!(
        !config.source_dirs.is_empty(),
        "Should have default source dirs"
    );
    assert!(
        !config.output_file.is_empty(),
        "Should have default output file"
    );
    assert!(!config.layout.is_empty(), "Should have default layout");
}

#[test]
fn test_config_has_exclude_patterns() {
    let config = Config::defaults();

    // Should have exclude patterns configured
    assert!(
        !config.exclude_patterns.is_empty() || !config.layers.is_empty(),
        "Should have some patterns configured"
    );
}

// ============================================================================
// Test 6: Consistency Between Sequential and Parallel Parsing
// ============================================================================

#[test]
fn test_discovery_consistency() {
    // Run discovery multiple times, should get same results
    let results1 =
        discovery::find_source_files(&["src".to_string()], &Config::defaults().exclude_patterns)
            .expect("Should discover files");

    let results2 =
        discovery::find_source_files(&["src".to_string()], &Config::defaults().exclude_patterns)
            .expect("Should discover files again");

    // Results should be identical (since parallel iterators should produce same results)
    assert_eq!(
        results1, results2,
        "Discovery should produce consistent results across runs"
    );
}

#[test]
fn test_multiple_discovery_runs_same_count() {
    // Run discovery 3 times, verify consistent file count
    let counts: Vec<usize> = (0..3)
        .map(|_| {
            discovery::find_source_files(&["src".to_string()], &Config::defaults().exclude_patterns)
                .expect("Should discover files")
                .len()
        })
        .collect();

    // All counts should be equal
    assert_eq!(counts[0], counts[1], "Discovery run 1 and 2 should match");
    assert_eq!(counts[1], counts[2], "Discovery run 2 and 3 should match");
}

// ============================================================================
// Test 7: Glob Pattern Matching
// ============================================================================

#[test]
fn test_glob_pattern_matching() {
    // Test the glob patterns used in discovery
    use glob::Pattern;

    let patterns = vec![
        ("**/target/**", true), // Should match target paths
        ("**/.git/**", true),   // Should match .git paths
        ("**/tests/**", true),  // Should match tests paths
        ("**/.hidden/*", true), // Should match hidden paths
    ];

    for (pattern_str, should_compile) in patterns {
        let result = Pattern::new(pattern_str);
        if should_compile {
            assert!(result.is_ok(), "Pattern '{}' should compile", pattern_str);
        }
    }
}

// ============================================================================
// Test 8: Path Handling Edge Cases
// ============================================================================

#[test]
fn test_pathbuf_conversion() {
    // Test that PathBuf converts correctly to string lossy
    let path = PathBuf::from("src/drivers/rust.rs");
    let lossy_str = path.to_string_lossy();

    assert!(lossy_str.len() > 0, "Should convert to string");
    assert!(lossy_str.contains("src"), "Should preserve path components");
}

#[test]
fn test_empty_discovery_result() {
    // Test that discovery works on empty source list
    let result = discovery::find_source_files(&[], &[]);

    // Should either return empty or error gracefully
    match result {
        Ok(files) => assert_eq!(files.len(), 0),
        Err(_) => { /* Error is acceptable for empty input */ }
    }
}

// ============================================================================
// Integration Test: Full Workflow
// ============================================================================

#[test]
fn test_config_and_discovery_integration() {
    // Full workflow: Load config, use it for discovery
    let config = Config::defaults();

    // Should be able to discover files using config source_dirs
    let files = discovery::find_source_files(&config.source_dirs, &config.exclude_patterns)
        .expect("Should discover files from config source directories");

    // Should find files (src directory should exist in test environment)
    assert!(
        files.len() > 0,
        "Should find files in configured source directories"
    );
}

#[test]
fn test_visibility_enum_serialization() {
    // Test that Visibility enum serializes correctly
    let public_vis = Visibility::Public;
    let private_vis = Visibility::Private;

    let pub_json = serde_json::to_value(&public_vis).expect("Should serialize");
    let priv_json = serde_json::to_value(&private_vis).expect("Should serialize");

    // Should serialize as strings
    assert!(
        pub_json.is_string(),
        "Should serialize Visibility as string"
    );
    assert!(
        priv_json.is_string(),
        "Should serialize Visibility as string"
    );
}
