# [**FluxionLib**](https://github.com/kisstp2006/FluxionLib)

FluxionLib is a work-in-progress game engine written in Rust. It uses a focused,
library-first design built as a modern, modular Rust workspace.

The project is in early development and is not ready for production use.

## Design goals

- Provide a focused collection of reusable game-engine capabilities.
- Keep application control flow outside the engine.
- Organize major engine subsystems as separate Rust crates.
- Expose an idiomatic Rust API and an optional C-compatible API.
- Produce a single native library for consumers of the C API.
- Support Windows and Linux.

## Current workspace

- `fluxionlib-math`: scalar, bitwise, and future multidimensional mathematics.
- `fluxionlib`: public Rust facade for the engine.
- `fluxionlib-capi`: C-compatible dynamic library interface.
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

## License

FluxionLib is available under the [MIT No Attribution](LICENSE) license (`MIT-0`).

Attribution is appreciated, but not required.
