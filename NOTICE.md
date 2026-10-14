# Notice

This crate is a standalone copy of `ext/crypto` from
[denoland/deno](https://github.com/denoland/deno) at tag `v2.9.7`
(`deno_crypto` 0.273.0), MIT licensed. See `LICENSE.md`.

Changes from upstream:

- Workspace dependency versions and features are pinned explicitly, because
  the crate is built outside the Deno monorepo.
- `rand` is pinned to `=0.8.6` (upstream: `=0.8.5`). The newer release fixes
  an unsoundness with custom loggers that use `rand::rng()`. This is the only
  functional change.
