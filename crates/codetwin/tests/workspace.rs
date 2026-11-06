//! Workspace inspection and visualization through the executable.

use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use serde_json::Value;
use tempfile::TempDir;

fn fixture() -> TempDir {
    let dir = TempDir::new().unwrap();
    fs::write(
        dir.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"app\", \"model\", \"unused\"]\nresolver = \"3\"\n",
    )
    .unwrap();
    for name in ["app", "model", "unused"] {
        let member = dir.path().join(name);
        fs::create_dir_all(member.join("src")).unwrap();
        let dependencies = if name == "app" {
            "\n[dependencies]\nmodel = { path = \"../model\" }\n"
        } else {
            ""
        };
        fs::write(
            member.join("Cargo.toml"),
            format!(
                "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n{dependencies}"
            ),
        )
        .unwrap();
        let (file, source) = if name == "app" {
            ("main.rs", "fn main() {}\n")
        } else {
            ("lib.rs", "")
        };
        fs::write(member.join("src").join(file), source).unwrap();
    }
    dir
}

fn run(cwd: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_codetwin"))
        .current_dir(cwd)
        .args(args)
        .output()
        .unwrap()
}

fn json(output: &Output) -> Value {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).unwrap()
}

#[test]
fn inspect_inventories_all_packages_and_local_dependencies() {
    let repo = fixture();
    let model = json(&run(repo.path(), &["inspect", ".", "--json"]));
    let packages = model["packages"].as_array().unwrap();
    let names: Vec<_> = packages
        .iter()
        .map(|p| p["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["app", "model", "unused"]);
    assert_eq!(packages[0]["id"], "app/Cargo.toml");
    assert_eq!(packages[0]["manifest_path"], "app/Cargo.toml");
    assert_eq!(packages[0]["targets"][0]["source_path"], "app/src/main.rs");
    assert_eq!(
        packages[0]["targets"][0]["kinds"],
        serde_json::json!(["bin"])
    );
    let edges = model["package_dependencies"].as_array().unwrap();
    assert_eq!(edges.len(), 1);
    assert_eq!(edges[0]["from"], "app/Cargo.toml");
    assert_eq!(edges[0]["to"], "model/Cargo.toml");
    assert_eq!(edges[0]["kind"], "normal");
    assert_eq!(model["language"], "rust");
    assert!(!repo.path().join("docs").exists());
    assert!(!repo.path().join("Cargo.lock").exists());
}

fn markdown_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files.extend(markdown_files(&path));
        } else if path.extension().is_some_and(|ext| ext == "md") {
            files.push(path);
        }
    }
    files.sort();
    files
}

#[test]
fn visualize_produces_an_overview_and_linked_crate_details() {
    let repo = fixture();
    let result = run(repo.path(), &["visualize", ".", "--output", "docs/code"]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let output = repo.path().join("docs/code");
    let index = fs::read_to_string(output.join("index.md")).unwrap();
    assert!(index.contains("```mermaid\nflowchart LR"));
    assert!(index.contains("Declared local dependencies"));
    let mut nodes = std::collections::BTreeMap::new();
    for line in index.lines() {
        if let Some((id, label)) = line.trim().split_once("[\"") {
            nodes.insert(label.trim_end_matches("\"]"), id);
        }
    }
    assert_eq!(nodes.len(), 3);
    assert!(
        nodes.contains_key("unused"),
        "isolated packages must be shown"
    );
    assert_eq!(index.matches(" -->|").count(), 1);
    assert!(index.contains(&format!("{} -->|normal| {}", nodes["app"], nodes["model"])));
    let files = markdown_files(&output);
    assert_eq!(files.len(), 4);
    for path in files {
        let text = fs::read_to_string(&path).unwrap();
        for part in text.split("](").skip(1) {
            let link = part.split(')').next().unwrap();
            assert!(
                path.parent().unwrap().join(link).is_file(),
                "broken link {link} in {}",
                path.display()
            );
        }
    }
    let details: Vec<_> = markdown_files(&output.join("crates"))
        .iter()
        .map(|p| fs::read_to_string(p).unwrap())
        .collect();
    let app = details
        .iter()
        .find(|text| text.contains("# app\n"))
        .unwrap();
    assert!(app.contains("app/src/main.rs"));
    assert!(app.contains("## Local dependencies"));
    let model = details
        .iter()
        .find(|text| text.contains("# model\n"))
        .unwrap();
    assert!(model.contains("## Dependents"));
    assert!(model.contains("[app]"));
}

#[test]
fn visualize_refuses_user_documents_before_writing_any_artifacts() {
    let repo = fixture();
    assert!(run(repo.path(), &["visualize"]).status.success());
    let output = repo.path().join("docs/code");
    let files = markdown_files(&output);
    let conflict = files
        .iter()
        .find(|p| p.parent().unwrap().ends_with("crates"))
        .unwrap()
        .clone();
    for path in files {
        fs::remove_file(path).unwrap();
    }
    fs::write(&conflict, "User-authored notes\n").unwrap();
    fs::write(output.join("notes.txt"), "Keep these notes\n").unwrap();
    let result = run(repo.path(), &["visualize"]);
    assert!(
        !result.status.success(),
        "user-owned destinations must be refused"
    );
    assert!(String::from_utf8_lossy(&result.stderr).contains("user-owned"));
    assert!(
        !output.join("index.md").exists(),
        "conflicts must be checked before any writes"
    );
    assert_eq!(
        fs::read_to_string(conflict).unwrap(),
        "User-authored notes\n"
    );
    assert_eq!(
        fs::read_to_string(output.join("notes.txt")).unwrap(),
        "Keep these notes\n"
    );
}

#[test]
fn inspect_preserves_inherited_aliases_and_dependency_qualifications() {
    let repo = fixture();
    fs::write(repo.path().join("Cargo.toml"),
        "[workspace]\nmembers = [\"app\", \"model\", \"unused\"]\nresolver = \"3\"\n[workspace.dependencies]\nschema = { package = \"model\", path = \"model\" }\n").unwrap();
    fs::write(repo.path().join("app/Cargo.toml"),
        "[package]\nname = \"app\"\nversion = \"0.1.0\"\nedition = \"2024\"\n[dependencies]\nschema = { workspace = true, optional = true }\n[dev-dependencies]\nmodel = { path = \"../model\" }\n[build-dependencies]\nunused = { path = \"../unused\" }\n[target.'cfg(unix)'.dependencies]\nschema = { package = \"model\", path = \"../model\" }\n").unwrap();
    let model = json(&run(repo.path(), &["inspect", "--json"]));
    let edges = model["package_dependencies"].as_array().unwrap();
    assert_eq!(edges.len(), 4);
    let optional = edges.iter().find(|e| e["optional"] == true).unwrap();
    assert_eq!(optional["to"], "model/Cargo.toml");
    assert_eq!(optional["alias"], "schema");
    assert_eq!(optional["kind"], "normal");
    assert!(
        edges
            .iter()
            .any(|e| e["kind"] == "dev" && e["to"] == "model/Cargo.toml")
    );
    assert!(
        edges
            .iter()
            .any(|e| e["kind"] == "build" && e["to"] == "unused/Cargo.toml")
    );
    assert!(
        edges
            .iter()
            .any(|e| e["target"] == "cfg(unix)" && e["alias"] == "schema")
    );
    assert!(run(repo.path(), &["visualize"]).status.success());
    let index = fs::read_to_string(repo.path().join("docs/code/index.md")).unwrap();
    assert_eq!(index.matches(" -->|").count(), 4);
    assert!(index.contains("normal; alias schema; optional"));
    assert!(index.contains("normal; alias schema; conditional"));
    assert!(index.contains("-->|dev|"));
    assert!(index.contains("-->|build|"));
    assert!(
        markdown_files(&repo.path().join("docs/code/crates"))
            .iter()
            .any(|p| fs::read_to_string(p).unwrap().contains("cfg(unix)"))
    );
}

fn documents(root: &Path) -> Vec<(std::path::PathBuf, Vec<u8>)> {
    markdown_files(root)
        .into_iter()
        .map(|path| {
            (
                path.strip_prefix(root).unwrap().to_path_buf(),
                fs::read(path).unwrap(),
            )
        })
        .collect()
}

#[test]
fn equivalent_workspaces_produce_identical_outputs_and_dependency_changes_refresh_them() {
    let first = fixture();
    let second = fixture();
    let original = run(first.path(), &["inspect", "--json"]);
    json(&original);
    assert_eq!(
        original.stdout,
        run(second.path(), &["inspect", "--json"]).stdout
    );
    assert!(
        !String::from_utf8(original.stdout)
            .unwrap()
            .contains(first.path().to_str().unwrap())
    );
    for repo in [&first, &second] {
        assert!(run(repo.path(), &["visualize"]).status.success());
    }
    let output = first.path().join("docs/code");
    let before = documents(&output);
    assert_eq!(before, documents(&second.path().join("docs/code")));
    assert!(run(first.path(), &["visualize"]).status.success());
    assert_eq!(before, documents(&output));

    let manifest = first.path().join("app/Cargo.toml");
    let text = fs::read_to_string(&manifest).unwrap().replace(
        "model = { path = \"../model\" }",
        "unused = { path = \"../unused\" }",
    );
    fs::write(manifest, text).unwrap();
    let updated = json(&run(first.path(), &["inspect", "--json"]));
    assert_eq!(updated["package_dependencies"].as_array().unwrap().len(), 1);
    assert_eq!(
        updated["package_dependencies"][0]["to"],
        "unused/Cargo.toml"
    );
    assert!(run(first.path(), &["visualize"]).status.success());
    assert_ne!(before, documents(&output));
    let details: Vec<_> = markdown_files(&output.join("crates"))
        .iter()
        .map(|p| fs::read_to_string(p).unwrap())
        .collect();
    let app = details
        .iter()
        .find(|text| text.contains("# app\n"))
        .unwrap();
    assert!(app.contains("[unused]"));
    assert!(!app.contains("[model]"));
    let model = details
        .iter()
        .find(|text| text.contains("# model\n"))
        .unwrap();
    assert!(model.contains("## Dependents\n\nNone."));
}

#[test]
fn disconnected_packages_and_similarly_named_packages_have_distinct_pages() {
    let repo = fixture();
    for (directory, name) in [
        ("app", "app"),
        ("model", "domain-model"),
        ("unused", "domain_model"),
    ] {
        fs::write(
            repo.path().join(directory).join("Cargo.toml"),
            format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2024\"\n"),
        )
        .unwrap();
    }
    let model = json(&run(repo.path(), &["inspect", "--json"]));
    assert_eq!(model["packages"].as_array().unwrap().len(), 3);
    assert_eq!(model["package_dependencies"], serde_json::json!([]));
    assert!(run(repo.path(), &["visualize"]).status.success());
    let index = fs::read_to_string(repo.path().join("docs/code/index.md")).unwrap();
    assert_eq!(index.matches("[\"").count(), 3);
    assert!(!index.contains(" -->|"));
    let details: Vec<_> = markdown_files(&repo.path().join("docs/code/crates"))
        .iter()
        .map(|p| fs::read_to_string(p).unwrap())
        .collect();
    assert_eq!(details.len(), 3);
    assert!(details.iter().any(|text| text.contains("# domain-model\n")));
    assert!(details.iter().any(|text| text.contains("# domain_model\n")));
}

#[test]
fn invalid_inputs_and_missing_cargo_fail_without_output() {
    let repo = fixture();
    for args in [
        vec!["visualize", "missing"],
        vec!["visualize", "app/src"],
        vec!["visualize", "--json"],
    ] {
        let result = run(repo.path(), &args);
        assert!(!result.status.success());
        assert_ne!(result.stderr, b"");
        assert_eq!(result.stdout, b"");
        assert!(!repo.path().join("docs").exists());
    }
    let result = Command::new(env!("CARGO_BIN_EXE_codetwin"))
        .current_dir(repo.path())
        .args(["inspect", "--json"])
        .env("CARGO", repo.path().join("absent-cargo"))
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("using Cargo"));
    assert_eq!(result.stdout, b"");
    fs::write(repo.path().join("Cargo.toml"), "[not valid TOML").unwrap();
    let result = run(repo.path(), &["visualize"]);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("using Cargo"));
    assert_eq!(result.stdout, b"");
    assert!(!repo.path().join("docs").exists());
}

#[test]
fn repository_selection_and_member_paths_use_the_workspace_root() {
    let repo = fixture();
    let elsewhere = TempDir::new().unwrap();
    let expected = run(repo.path(), &["inspect", "--json"]);
    let explicit = run(
        elsewhere.path(),
        &["inspect", repo.path().to_str().unwrap(), "--json"],
    );
    assert_eq!(json(&expected), json(&explicit));
    let member = run(
        elsewhere.path(),
        &[
            "-C",
            repo.path().to_str().unwrap(),
            "inspect",
            "app",
            "--json",
        ],
    );
    assert_eq!(json(&expected), json(&member));
    fs::write(
        repo.path().join("Cargo.lock"),
        "# User lockfile\nversion = 4\n",
    )
    .unwrap();
    assert!(
        run(
            elsewhere.path(),
            &[
                "-C",
                repo.path().to_str().unwrap(),
                "visualize",
                "app",
                "--output",
                "generated"
            ]
        )
        .status
        .success()
    );
    assert!(repo.path().join("generated/index.md").is_file());
    assert!(!repo.path().join("app/generated").exists());
    assert!(!elsewhere.path().join("generated").exists());
    assert_eq!(
        fs::read_to_string(repo.path().join("Cargo.lock")).unwrap(),
        "# User lockfile\nversion = 4\n"
    );
}

#[test]
fn output_parent_conflicts_are_detected_before_writing() {
    let repo = fixture();
    let output = repo.path().join("docs/code");
    fs::create_dir_all(&output).unwrap();
    fs::write(output.join("crates"), "Keep this file\n").unwrap();
    let result = run(repo.path(), &["visualize"]);
    assert!(!result.status.success());
    assert!(!output.join("index.md").exists());
    assert_eq!(
        fs::read_to_string(output.join("crates")).unwrap(),
        "Keep this file\n"
    );
}

#[cfg(unix)]
#[test]
fn output_symlinks_are_refused_before_writing() {
    let repo = fixture();
    let elsewhere = TempDir::new().unwrap();
    let output = repo.path().join("docs/code");
    fs::create_dir_all(&output).unwrap();
    std::os::unix::fs::symlink(elsewhere.path(), output.join("crates")).unwrap();
    let result = run(repo.path(), &["visualize"]);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("symlink"));
    assert!(!output.join("index.md").exists());
    assert_eq!(fs::read_dir(elsewhere.path()).unwrap().count(), 0);
}

#[test]
fn source_links_encode_spaces_and_markdown_url_delimiters() {
    let repo = fixture();
    fs::rename(
        repo.path().join("app/src/main.rs"),
        repo.path().join("app/src/main file (#1).rs"),
    )
    .unwrap();
    let manifest = repo.path().join("app/Cargo.toml");
    let mut text = fs::read_to_string(&manifest).unwrap();
    text.push_str("\n[[bin]]\nname = \"app\"\npath = \"src/main file (#1).rs\"\n");
    fs::write(manifest, text).unwrap();
    assert!(run(repo.path(), &["visualize"]).status.success());
    let details: Vec<_> = markdown_files(&repo.path().join("docs/code/crates"))
        .iter()
        .map(|p| fs::read_to_string(p).unwrap())
        .collect();
    let app = details
        .iter()
        .find(|text| text.contains("# app\n"))
        .unwrap();
    assert!(app.contains("../../../app/src/main%20file%20%28%231%29.rs)"));
    assert_eq!(
        json(&run(repo.path(), &["inspect", "--json"]))["packages"][0]["targets"][0]["source_path"],
        "app/src/main file (#1).rs"
    );
}

#[test]
fn dependencies_outside_workspace_members_are_excluded() {
    let repo = fixture();
    let root = repo.path().join("Cargo.toml");
    let text = fs::read_to_string(&root).unwrap() + "exclude = [\"vendor\"]\n";
    fs::write(root, text).unwrap();
    fs::create_dir_all(repo.path().join("vendor/src")).unwrap();
    fs::write(repo.path().join("vendor/src/lib.rs"), "").unwrap();
    fs::write(
        repo.path().join("vendor/Cargo.toml"),
        "[package]\nname = \"vendor\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
    )
    .unwrap();
    let app = repo.path().join("app/Cargo.toml");
    let text = fs::read_to_string(&app).unwrap() + "vendor = { path = \"../vendor\" }\n";
    fs::write(app, text).unwrap();
    let model = json(&run(repo.path(), &["inspect", "--json"]));
    assert_eq!(model["packages"].as_array().unwrap().len(), 3);
    assert_eq!(model["package_dependencies"].as_array().unwrap().len(), 1);
    assert_eq!(model["package_dependencies"][0]["to"], "model/Cargo.toml");
}

#[test]
fn legacy_commands_and_default_generation_still_route_through_the_binary() {
    let repo = fixture();
    for command in ["gen", "init", "list", "snapshot", "diff"] {
        let help = run(repo.path(), &[command, "--help"]);
        assert!(help.status.success());
        assert!(String::from_utf8_lossy(&help.stdout).contains("Usage:"));
    }
    let registry = json(&run(repo.path(), &["list", "--json"]));
    assert!(
        registry["layouts"]
            .as_array()
            .is_some_and(|layouts| !layouts.is_empty())
    );
    let model = json(&run(repo.path(), &["gen", "--dump-ir"]));
    assert!(model["modules"].is_array());
    let elsewhere = TempDir::new().unwrap();
    let default = run(
        elsewhere.path(),
        &["-C", repo.path().to_str().unwrap(), "--quiet"],
    );
    assert!(
        default.status.success(),
        "{}",
        String::from_utf8_lossy(&default.stderr)
    );
    assert!(repo.path().join("docs/architecture.md").is_file());
    assert!(!elsewhere.path().join("docs").exists());
}

#[test]
fn relative_target_paths_cannot_escape_the_workspace() {
    let repo = fixture();
    let outside = TempDir::new_in(repo.path().parent().unwrap()).unwrap();
    fs::write(outside.path().join("outside.rs"), "fn main() {}\n").unwrap();
    let directory = outside.path().file_name().unwrap().to_str().unwrap();
    let manifest = repo.path().join("app/Cargo.toml");
    let text = fs::read_to_string(&manifest).unwrap()
        + &format!("\n[[bin]]\nname = \"app\"\npath = \"../../{directory}/outside.rs\"\n");
    fs::write(manifest, text).unwrap();
    let result = run(repo.path(), &["visualize"]);
    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("outside the workspace"));
    assert!(!repo.path().join("docs").exists());
}

#[test]
fn deep_member_paths_generate_valid_pages_without_filename_length_errors() {
    let repo = fixture();
    let nested = format!("nested/{}/{}/unused", "a".repeat(64), "b".repeat(64));
    let destination = repo.path().join(&nested);
    fs::create_dir_all(destination.parent().unwrap()).unwrap();
    fs::rename(repo.path().join("unused"), destination).unwrap();
    let manifest = repo.path().join("Cargo.toml");
    let text = fs::read_to_string(&manifest)
        .unwrap()
        .replace("\"unused\"", &format!("\"{nested}\""));
    fs::write(manifest, text).unwrap();
    let result = run(repo.path(), &["visualize"]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let output = repo.path().join("docs/code");
    let files = markdown_files(&output);
    assert_eq!(files.len(), 4);
    for path in files {
        let text = fs::read_to_string(&path).unwrap();
        for part in text.split("](").skip(1) {
            let link = part.split(')').next().unwrap();
            assert!(
                path.parent().unwrap().join(link).is_file(),
                "broken link {link}"
            );
        }
    }
}
