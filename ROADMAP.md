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

## Bidirectional Sync (Docs -> Code)

1. [ ] Define a layout-agnostic sync block format (structured, marked section) for docs -> code
2. [ ] Emit the sync block from every formatter (including proposed ones) alongside human-friendly
       layout
3. [ ] Implement importer that reads only the sync block to rebuild Blueprints (ignores
       layout-specific prose)
4. [ ] Add tests covering round-trip for multiple formatters (folder_markdown, mirror_tree,
       readme_append)
5. [ ] Document the sync block contract and markers in README and CLI help

## Subcommands

- watch: Not implemented (returns error stub)
- sync: Implemented (code -> docs via formatter selection; docs -> code pending)
- check: Not implemented (returns error stub)
- init: Implemented (writes config, docs folder, template main diagram)
- list: Not implemented (returns error stub)
