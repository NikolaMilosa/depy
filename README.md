# Dependency analizer

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
dep-analyzer --output ~/Downloads/out rust ../path/to/Cargo.toml
```
To visualize the output one can use a browser.
