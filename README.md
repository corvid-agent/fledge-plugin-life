# fledge-plugin-life

Conway's Game of Life — visual WASM plugin demo for [fledge](https://github.com/CorvidLabs/fledge).

Runs an animated Game of Life simulation directly in your terminal via the
fledge plugin system. The board is seeded with an R-pentomino, two gliders,
and an LWSS (lightweight spaceship) and evolves for 80 generations with
real-time ANSI rendering.

## Install

```bash
fledge plugins install corvid-agent/fledge-plugin-life
```

## Usage

```bash
fledge life
```

The simulation renders a 50x25 board at ~8 fps, using box-drawing borders
and block characters for live cells. The cursor is hidden during playback
and restored on completion.

## Build from source

Requires Rust with the `wasm32-wasip1` target:

```bash
rustup target add wasm32-wasip1
cargo build --target wasm32-wasip1 --release
```

The compiled WASM binary is written to
`target/wasm32-wasip1/release/fledge-plugin-life.wasm`.

## Tests

```bash
cargo test
```

The test suite covers the core game logic: neighbor counting, B3/S23 rules,
still lifes (block), oscillators (blinker), spaceships (glider), birth,
death, pattern placement, and out-of-bounds clipping.

## Plugin manifest

See `plugin.toml` for the fledge-v1 protocol manifest. This plugin requires
no capabilities (no filesystem, network, exec, or store access).

## License

MIT
