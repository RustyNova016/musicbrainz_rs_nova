# FAQ

## Deal with CI Fails

### Clippy

All pull requests must not have any clippy warnings or errors before merging. If you are unsure how to fix an issue, please tell it in the PR message so we can help

### FMT

All code must be passed through `cargo fmt` before the PR. It is generally better to merge the formating commit with the code, but reformating commits are allowed (Use `style: cargo fmt` for the commit message if needed)

### MSRV

All code must pass the MSRV set. If the MSRV needs a bump, feel free to do it.

Hint:
- Check your new MSRV with [cargo-msrv](https://github.com/foresterre/cargo-msrv): `cargo msrv find`

### Minimum crate versions

The minimum crate versions must fit the code. If newer features are used, update the version of the dependancies to the minimum needed. Only direct dependancies are needed as doing the full tree is unrealistic.

Hint:
- Check if the minimum versions are respected with [cargo-minimal-versions](https://github.com/taiki-e/cargo-minimal-versions): `cargo minimal-versions check --direct`