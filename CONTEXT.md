# CodeTwin

CodeTwin helps developers understand a repository by presenting its code graphically, without
requiring them to read every file or line of code.

## Language

**Code Visualization**:
A graphical representation of the software contained in a repository, inspired by UML.
_Avoid_: Documentation generation, architecture governance

**Repository**:
The codebase that CodeTwin examines and visualizes.
_Avoid_: Project, workspace

**Repository Overview**:
The first visualization, showing the repository's major components and the dependencies between
them.
_Avoid_: Full code graph, class dump

**Detail View**:
A focused visualization reached from the overview that exposes finer-grained code structure and
relationships.
_Avoid_: Overview, raw source listing

**Code Model**:
The language-neutral representation of a repository's software elements and relationships.
_Avoid_: Markdown, diagram, language AST

**Visualization Set**:
The canonical user-facing Markdown artifact containing a repository overview and linked detail
views.
_Avoid_: Code Model, single diagram, HTML application

## Relationships

- A **Repository** is the source of one or more **Code Visualizations**.
- A **Repository** is translated into one **Code Model**.
- A **Code Model** is presented by a **Visualization Set**.
- A **Visualization Set** opens with a **Repository Overview**.
- A **Repository Overview** leads to one or more **Detail Views**.

## Dogfooding

CodeTwin's own repository is the first real input used to evaluate the tool. As CodeTwin
improves, its visualizations should make the repository itself easier to understand.

The feedback loop is: analyze CodeTwin → inspect its overview and detail views → identify
missing or misleading information → improve the tool → regenerate the visualizations.
The output must reflect the current implementation as the repository changes.

The first overview and detail target is drafted in
[`docs/DOGFOOD_TARGET.md`](docs/DOGFOOD_TARGET.md). Rust package inventory and linked package views are implemented through `inspect` and
`visualize`; module dependencies and final crate boundaries remain future design work.

## Example dialogue

> **Developer:** "I do not want to read every file in this **Repository** just to understand its
> structure."
> **Domain expert:** "Start with its **Repository Overview**, then open a **Detail View** for the
> component you want to investigate."

## Flagged ambiguities

- "Documentation" previously described the primary output; resolved: **Code Visualization** is
  the primary product outcome, while documentation remains an unconfirmed secondary use case.
- The developer entry point is a CLI; whether it also launches a browser-based viewer remains
  unresolved.
- Markdown is the canonical user-facing format for the **Visualization Set**; richer derived
  renderings may be considered later.
