# Roadmap

## Formatter

1. [x] Define `Formatter` trait that takes Vec<Blueprint> and returns Vec<(filename, content)>
2. [x] Add formatter registry/factory (e.g., `formatters::get_formatter(name)`) with a default
3. [x] Move current folder-based markdown logic into `formatters/current.rs`
4. [x] Add alternative formatter stub `one_per_file` (one .md per source file)
5. [x] Wire `codetwin.toml` to choose formatter (config key: `formatter` with default `current`)
6. [x] Update `SyncEngine::sync()` to call the selected formatter instead of markdown directly
7. [x] Add tests: formatter selection, filename outputs, and non-empty content per file
8. [x] Document formatter usage in README (how to switch layouts)

## Diagram-First Formatter

1. [ ] Create `src/formatters/diagram_first.rs` module with struct and constructor
2. [ ] Extract Mermaid diagram generation logic into reusable helper function
3. [ ] Implement `Formatter` trait: generate diagram-only outputs (minimal prose, focused on
       Mermaid/PlantUML)
4. [ ] Register `diagram_first` formatter in `formatters::get_formatter()` factory
5. [ ] Add unit test: verify diagram_first produces diagram blocks and minimal text
6. [ ] Update README with `diagram_first` use case and sample output
