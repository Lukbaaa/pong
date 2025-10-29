# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

Project overview
- Language/tooling: Rust with Cargo; single binary crate named "pong".
- Rendering/game loop: Piston ecosystem using OpenGL 3.2 via glutin_window and opengl_graphics.
- Entry point: src/main.rs.

Common commands
- Setup (if needed)
  ```sh
  rustup component add clippy rustfmt
  ```
- Build and run
  ```sh
  cargo build
  cargo run
  cargo build --release && ./target/release/pong
  ```
- Lint/format
  ```sh
  cargo fmt --check        # format check
  cargo fmt                # apply formatting
  cargo clippy --all-targets --all-features -- -D warnings
  ```
- Tests
  ```sh
  cargo test                         # run all tests (none exist yet)
  cargo test <pattern>               # run a single test by name pattern
  cargo test -- --nocapture          # show test output
  ```

Architecture and code structure
- App state (struct App)
  - Fields: GlGraphics renderer, pressed_keys (HashSet<Key>), two paddles (Player), one Ball, is_started flag.
  - Responsibilities:
    - render(&RenderArgs): clears to white, draws black ball and paddles using piston2d-graphics primitives.
    - update(&UpdateArgs):
      - Input-driven paddle movement: W/S for Player 1; Up/Down for Player 2; clamped to window bounds.
      - Game start: first W or S press toggles is_started.
      - Ball physics: moves each update when started; reflects on top/bottom walls; paddle collisions adjust outgoing angle based on impact point.
    - key_press/key_release: maintains pressed_keys set from piston events.
- Entities
  - Player { size, ratio, position } with:
    - collided(&Ball) AABB overlap check against ball circle bounds.
    - collision_point(&Ball) maps contact along paddle height to [-1, 1] to shape rebound angle.
  - Ball { size, position, angle } where angle is in radians; horizontal/vertical deltas computed via cos/sin.
- Window and loop
  - WindowSettings("Pong", [WIDTH, HEIGHT]) with OpenGL::V3_2, exit_on_esc.
  - Events loop dispatches render/update and keyboard press/release via piston.
- Coordinates and constants
  - WIDTH/HEIGHT = 800. Y grows downward; ball Y update subtracts sin(angle) accordingly.

Repository highlights
- Cargo.toml
  - Dependencies: piston, piston2d-graphics, pistoncore-glutin_window, piston2d-opengl_graphics.
- src/main.rs
  - Contains all game logic; there are no integration/unit tests or additional modules yet.

Notes for future changes
- When adding new behavior (scoring, AI, pause, etc.), keep rendering in App::render and logic in App::update to preserve the event-driven structure.
