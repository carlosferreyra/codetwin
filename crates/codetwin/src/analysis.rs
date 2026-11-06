//! Rust workspace inventory without compilation or dependency downloads.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use cargo_metadata::{DependencyKind, MetadataCommand};
use codetwin_ir::{
    CodeModel, Package, PackageDependency, PackageDependencyKind, PackageId, PackageTarget,
};

/// Analysis result with a filesystem root kept outside the serializable model.
pub struct RepositoryAnalysis {
    /// Absolute workspace root used for filesystem operations.
    pub root: PathBuf,
    /// Portable extracted workspace facts.
    pub model: CodeModel,
}

/// Inventory all members and local dependency declarations in a Rust workspace.
pub fn inspect(repository: &Path) -> Result<RepositoryAnalysis> {
    let repository = repository
        .canonicalize()
        .with_context(|| format!("cannot open repository {}", repository.display()))?;
    let manifest = repository.join("Cargo.toml");
    if !manifest.is_file() {
        bail!("repository {} has no Cargo.toml", repository.display());
    }
    let metadata = MetadataCommand::new()
        .manifest_path(&manifest)
        .current_dir(&repository)
        .no_deps()
        .other_options(vec!["--offline".into(), "--locked".into()])
        .exec()
        .with_context(|| {
            format!(
                "cannot inspect Rust workspace at {} using Cargo",
                manifest.display()
            )
        })?;
    let root = metadata
        .workspace_root
        .as_std_path()
        .canonicalize()
        .context("cannot open Cargo workspace root")?;
    let mut model = CodeModel::new("rust");
    let mut identities = BTreeMap::new();
    for package in &metadata.packages {
        let manifest = package.manifest_path.as_std_path();
        let manifest_path = relative_path(manifest, &root)?;
        let id = PackageId(manifest_path.clone());
        identities.insert(manifest.canonicalize()?, id.clone());
        let mut targets = Vec::new();
        for target in &package.targets {
            let mut kinds: Vec<_> = target.kind.iter().map(ToString::to_string).collect();
            kinds.sort();
            targets.push(PackageTarget {
                name: target.name.clone(),
                kinds,
                source_path: relative_path(target.src_path.as_std_path(), &root)?,
            });
        }
        targets.sort();
        model.packages.push(Package {
            id,
            name: package.name.to_string(),
            manifest_path,
            targets,
        });
    }
    for package in &metadata.packages {
        let from = identities
            .get(&package.manifest_path.as_std_path().canonicalize()?)
            .context("workspace package identity is missing")?;
        for dependency in &package.dependencies {
            let Some(path) = &dependency.path else {
                continue;
            };
            let manifest = path
                .as_std_path()
                .join("Cargo.toml")
                .canonicalize()
                .with_context(|| {
                    format!("cannot open dependency manifest for {}", dependency.name)
                })?;
            let Some(to) = identities.get(&manifest) else {
                continue;
            };
            let kind = match dependency.kind {
                DependencyKind::Normal => PackageDependencyKind::Normal,
                DependencyKind::Development => PackageDependencyKind::Dev,
                DependencyKind::Build => PackageDependencyKind::Build,
                _ => bail!("unsupported dependency kind for {}", dependency.name),
            };
            model.package_dependencies.push(PackageDependency {
                from: from.clone(),
                to: to.clone(),
                kind,
                alias: dependency.rename.clone(),
                optional: dependency.optional,
                target: dependency.target.as_ref().map(ToString::to_string),
            });
        }
    }
    model.packages.sort_by(|a, b| a.id.cmp(&b.id));
    model.package_dependencies.sort();
    model.package_dependencies.dedup();
    Ok(RepositoryAnalysis { root, model })
}

fn relative_path(path: &Path, root: &Path) -> Result<String> {
    let path = path
        .canonicalize()
        .with_context(|| format!("cannot open workspace path {}", path.display()))?;
    let relative = path
        .strip_prefix(root)
        .with_context(|| format!("source path {} is outside the workspace", path.display()))?;
    relative
        .iter()
        .map(|part| part.to_str().context("workspace path is not UTF-8"))
        .collect::<Result<Vec<_>>>()
        .map(|parts| parts.join("/"))
}
