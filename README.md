# inlay-hints

Embed inlay hints of `rust-analyzer` into Rust code without an interactive LSP editor such as VSCode or NeoVim.

## Installation

```bash
git submodule sync
cargo install --path .
```
or
```bash
cargo install --git https://github.com/yijunyu/inlay-hints
```

## Usage
With a `folder` containing Rust source code files `*.rs`, run
```bash
inlay-hints [<source-folder>] [<output-folder>]
```

From Rust code in the `source-folder`, this command will insert inlay hints
labels, including type declarations, parameter names, chaining types, lifetime
markers, etc. into the Rust code saved into the `output-folder`.

* When the argument `<source-folder>` is not provided, the source folder will
be default to the current folder `.`.

* When the argument `<output-folder>` is not provided, the output folder will
be default to the `./inlay-hints`.

### Counting inlay hints
Since the inlay hints are line-based edits to the original source,
a single recursive diff command could count how many have been inserted:
```bash
diff -r <source-folder> <output-folder> | grep "^---" | wc
```
## Updates
- [ ] to fix: the end marker of a function seems not accurate
