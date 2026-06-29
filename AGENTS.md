# Repository Guidelines

## Project Structure & Module Organization

This repository is a Rust 2024 Macroquad game crate named `Seppong`. Core source lives in `src/`: `main.rs` wires the game loop, while `game.rs`, `state.rs`, `system.rs`, `entities.rs`, and `draw.rs` separate gameplay, state, systems, entity definitions, and rendering. Bitmap assets used by the game live in `assets/` and are loaded by relative path at runtime. Cargo metadata is in `Cargo.toml`; `.cargo/config.toml` contains the WebAssembly target linker flag.

## Build, Test, and Development Commands

- `cargo run`: builds and runs the desktop version locally.
- `cargo build`: checks that the default debug build compiles.
- `cargo build --release`: creates an optimized desktop build.
- `cargo test`: runs unit and integration tests when present.
- `cargo fmt`: formats Rust code with rustfmt.
- `cargo clippy --all-targets --all-features`: runs lint checks across targets.
- `rustup target add wasm32-unknown-unknown`: installs the WASM target if needed.
- `cargo build --target wasm32-unknown-unknown --release`: builds the game for WebAssembly.

## Coding Style & Naming Conventions

Use standard Rust formatting with 4-space indentation via `cargo fmt`. Keep modules focused and prefer explicit, descriptive names for gameplay concepts, such as `PowerUp`, `Game`, or `draw_pause`. Use `snake_case` for functions, variables, modules, and asset filenames where possible; use `PascalCase` for types and enum variants. Keep rendering code in `draw.rs` unless it is tightly coupled to entity state.

## Commit & Pull Request Guidelines

Git history uses conventional-style messages such as `feat(power-up): ...` and `fix(ball-velocity): ...`. Keep that pattern: `type(scope): short imperative summary`, with common types like `feat`, `fix`, `refactor`, and `test`.

Pull requests should include a concise description, linked issue if applicable, the commands run, and screenshots or a short recording for visual gameplay changes. Call out asset additions, balance changes, and any WASM-specific impact.

## Asset Generation Guidelines

All new game assets should use a pixel-art style consistent with the existing
`assets/` files, especially `assets/yachter.png`. Prefer crisp, readable
silhouettes, limited palettes, strong outlines, and deliberate blocky shading.
Avoid painterly, blurred, photorealistic, or high-detail rendered styles.

Export assets as PNG files with transparent backgrounds unless the game
specifically needs a solid background. Keep objects centered with minimal
empty padding, and name files in lowercase kebab-case, for example `speed-
  power-up.png` or `enemy-paddle.png`.

When requesting generated assets, describe the subject, orientation, scale,
palette, and gameplay purpose. Example: “pixel-art beer bottle paddle, side
view, amber glass, cream-and-gold label, transparent background, matching
`assets/yachter.png`.”
