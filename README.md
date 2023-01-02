# inlay-hints

Embed inlay hints of `rust-analyzer` into Rust code without an interactive LSP
editor such as VSCode or NeoVim.

## installation

```bash
git submodule sync
cargo install --path .
```

# usage
With a `folder` containing Rust source code files `*.rs`, run
```bash
inlay-hints [<source-folder>] [<output-folder>]
```

From Rust code from `source-folder`, this command will insert `rust-analyzer`
inlay hint labels, for type declarations, parameter names, chaining types,
lifetime markers, etc. into the Rust code  saved into the `output-folder`.

When the argument `<output-folder>` is not provided, the output folder will
be default to the `./inlay-hints` subfolder.

When the argument `<source-folder>` is not provided, the source folder will
be default to the current folder `.`.

## Updates
- [ ] to fix: the end marker of a function seems not accurate
