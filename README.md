`rust-analyzer` in VSCode
-------------------------

Use the following or similar setup to avoid infinite rebuild loop between
`rust-analyzer` and `cargo` command-line tools:

```json
"rust-analyzer.cargo.extraArgs": [
  "--target-dir=${workspaceFolder}/vscode_target"
]
```
