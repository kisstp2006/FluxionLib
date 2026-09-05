# LibFlux

LibFlux is a work-in-progress game engine written in Rust. It is inspired by
[LibPHX](https://github.com/JoshParnell/libphx) and aims to preserve its focused,
library-first design while using a modern, modular Rust workspace.

The project is in early development and is not ready for production use.

## Design goals

- Provide the same broad engine capabilities as LibPHX.
- Keep application control flow outside the engine.
- Organize major engine subsystems as separate Rust crates.
- Expose an idiomatic Rust API and an optional C-compatible API.
- Produce a single native library for consumers of the C API.
- Support Windows and Linux.

## Current workspace

- `flux-math`: scalar, bitwise, and future multidimensional mathematics.
- `libflux`: public Rust facade for the engine.
- `libflux-capi`: C-compatible dynamic library interface.
- `sandbox`: executable used for examples and development experiments.

Additional subsystem crates will be introduced only when their implementation begins.

## Requirements

- Rust 1.98.1, installed through `rustup`.
- The platform's native development tools.

The included `rust-toolchain.toml` selects the required Rust toolchain automatically.

## Build and test

Build the complete workspace:

```text
cargo build --workspace
```

Run all tests:

```text
cargo test --workspace
```

Run formatting, tests, and Clippy on Windows:

```text
.\scripts\verify.ps1
```

Run the same checks on Linux:

```text
./scripts/verify.sh
```

## Origin

LibFlux is an independent Rust project based on the architecture and behavior of the
public-domain LibPHX game engine. LibFlux is not affiliated with the original LibPHX
or Limit Theory projects.

## License

LibFlux is available under the [MIT No Attribution](LICENSE) license (`MIT-0`).

Attribution is appreciated, but not required.
