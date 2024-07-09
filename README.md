# Dependency analyzer

A simple CLI tool for visualizing dependencies within projects currently in development.

It relies on [Graphviz](https://graphviz.org/).

## Installation
1. Install [graphviz](https://graphviz.org/download/)
2. Build from source code:
```bash
cargo build --release
```

## Usage
```bash
depy -p ../path/to/Cargo.toml > ~/Downloads/output.svg
```

A great feature of the tool is picking the start of the graph (or the top-most element). In complex workspaces, usually, an explosion of dependencies can happen and usually a developer is mostly concerned about a single, or a couple of packages. To do that one can do the following:
```bash
depy -p ../path/to/Cargo.toml -t name-of-top-level-target > ~/Downloads/output.svg
```

To visualize the output one can use a browser.
