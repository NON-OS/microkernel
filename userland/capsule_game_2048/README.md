# capsule_game_2048

`game_2048` is the 2048 sliding tile game in a 360 by 480 window, played with the arrow keys. It is
for anyone at the desktop who wants a small game, and it doubles as an example of an upstream
`no_std` crate driving a NONOS window. The `Cargo.toml` header says it is shipped only through the
on-disk package store.

## Role

A `no_std` application on `nonos_app_skeleton` (`src/main.rs` calls `nonos_app_skeleton::run`). The
game rules come from the vendored crate `tools-2048` 0.4.1
(https://github.com/amamic1803/tools-2048-rs) under `userland/upstream-src/tools-2048`, used as a
path dependency; this capsule adds only input mapping, colors and painting. `mk/40-run.mk` packs it
into the QEMU block store under `/capsules/game_2048.*` with the other demo capsules.

## Capabilities

`CAPSULE_REQUIRED_CAPS := 0x1819` (no decoding comment in `Capsule.mk`; decoded from
`userland/nonos_cap/src/bits.rs`):

- `0x0001` CoreExec: run.
- `0x0008` IPC: talk to the compositor, wm, input_router and toolkit.
- `0x0010` Memory: heap.
- `0x0800` GraphicsDisplayQuery and `0x1000` GraphicsSurfaceCreate: its window surface.

Endpoints: service `service:4920:app.game_2048`, reply `reply:4921:endpoint.app.game_2048.reply`.
The domain is `com.example`.

## Interface

It serves no IPC requests of its own. `src/game/manifest.rs` declares the window (id `0x32303438`,
key-down input). `src/game/event.rs` maps Left, Right, Up and Down to `GameMove`, `R` or `r` to a
new board, and Esc to `Close`; moves are ignored once the game is over. `src/game/app.rs` seeds
`Game::<4>::new_seeded` from `mk_time_millis`. `src/game/paint.rs` draws the score, the 4 by 4 board
with colors from `src/game/colors.rs`, and "GAME OVER".

## State and privacy

All state is the board and a game-over flag in memory (`src/game/app.rs`). There is no high score
file and no persistence: closing the window loses the game. It reads no files and holds no
FileSystem, Network or Debug bit.

## Build and test

- `make nonos-mk-game_2048` builds the ELF; `make nonos-mk-game_2048-sign` produces the
  certificate, manifest and trailer.
- The store image that carries it is `$(QEMU_BLK_STORE_STAMP)` in `mk/40-run.mk`.
- The upstream crate keeps its own tests in `userland/upstream-src/tools-2048/tests`.

No proof crate covers this capsule.

## Not done yet

- `tools-2048` keeps the score as `u64` (`userland/upstream-src/tools-2048/src/core.rs:166`), but
  `src/game/paint.rs:38` casts it to `u32` before drawing, so a score past `u32::MAX` would wrap
  on screen.

The app model it is built on is described in
[Applications, window manager and desktop shell](../../docs/handbook/desktop/app-model.md).
