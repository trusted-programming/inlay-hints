# inlay-hints

Embed inlay hints of `rust-analyzer` into Rust code without an interactive LSP
editor such as VSCode or NeoVim.

## installation

```bash
cargo install --path crates/ide
```

# usage
With a `folder` containing Rust source code files `*.rs`, run
```bash
ide [<folder>]
```

This command will insert the inferred type declarations, parammeter names,
chaining types, and lifetime annotations. Any inlay labels produced to LSP
editor such as Visual Studio Code or NeoVim, can be inserted into the source as
comments or redundant information of the Rust code.

## Updates
- [ ] to fix: the end marker of a function seems not accurate
