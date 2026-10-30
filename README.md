# kani-nix-example

> This repository is a companion to
> [model-checking/kani#5098](https://github.com/model-checking/kani/pull/5098), which adds a Nix
> flake to Kani. It shows that flake in use, and its CI runs it on Linux and macOS.

[![Kani](https://github.com/lalvesl/kani-nix-example/actions/workflows/kani.yml/badge.svg)](https://github.com/lalvesl/kani-nix-example/actions/workflows/kani.yml)

A small Rust crate verified with [Kani](https://github.com/model-checking/kani), using Kani's
Nix flake to select the Kani version per development shell. There is no rustup, no
`cargo install kani-verifier`, and no `cargo kani setup`.

## Usage

```bash
nix develop .#kani-latest   # the newest Kani release the flake knows (also the default shell)
nix develop .#kani-0_67     # Kani 0.67.0
cargo kani
```

Expected result, with either shell:

```
Checking harness estimate_size::proofs::finds_precondition_violation...
VERIFICATION:- SUCCESSFUL (encountered one or more panics as expected)
Checking harness estimate_size::proofs::verify_success...
VERIFICATION:- SUCCESSFUL
Checking harness initialize_prefix::proofs::check_initialize_prefix...
VERIFICATION:- SUCCESSFUL
Complete - 3 successfully verified harnesses, 0 failures, 3 total.
```

`cargo kani --version` shows which Kani each shell provides.

CI (`.github/workflows/kani.yml`) runs `cargo kani` in both shells on x86_64 Linux, aarch64
Linux and aarch64 macOS, on every push and pull request.

Shell names use `_` instead of `.` (`kani-0_67`), because Nix reads a `.` in `.#kani-0.67` as
an attribute path separator.

## How it works

`flake.nix` applies Kani's overlay and builds one shell per Kani release:

```nix
kani-latest = kaniShell pkgs.kaniReleases.latest;
kani-0_67 = kaniShell pkgs.kaniReleases."0.67.0";
```

Each `pkgs.kaniReleases.<version>` brings that release's Kani, from its official release bundle,
together with the exact nightly toolchain it was built with, from
[oxalica/rust-overlay](https://github.com/oxalica/rust-overlay). You only choose the Kani
version. The crate itself is built with stable Rust: Kani compiles it with its own toolchain, so
the project's toolchain does not have to match.

To add another version, add a shell for it, e.g.
`kani-0_66 = kaniShell pkgs.kaniReleases."0.66.0";`. The versions available are the ones
recorded in Kani's `nix/versions.json`; see `nix/README.md` in the Kani repository.

### If you also installed Kani with `cargo install`

`cargo` looks for subcommands such as `cargo-kani` in `~/.cargo/bin` before `PATH`, unless
`~/.cargo/bin` is on `PATH`. A Kani installed with `cargo install kani-verifier` would then win
over the shell's. The shells append `~/.cargo/bin` to the end of `PATH` to prevent that.

## The proofs

Adapted from the [Kani tutorial](https://model-checking.github.io/kani/kani-tutorial.html):

- `src/estimate_size.rs`: one harness that proves `estimate_size` correct for valid inputs, and
  one (`#[kani::should_panic]`) that shows Kani finding an input that breaks its precondition.
- `src/initialize_prefix.rs`: a loop verified with `#[kani::unwind]`. This is the tutorial's
  example with its off-by-one bug fixed.
