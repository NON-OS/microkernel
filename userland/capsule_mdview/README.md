# capsule_mdview

`mdview` is a read-only markdown viewer: it reads `/readme.txt` from the vfs capsule, parses it with
the crates.io `pulldown-cmark` parser (0.13, default features off) and paints headings, paragraphs,
bullets, inline code and code blocks in a 680 by 520 window. It is for a desktop user reading the
seeded readme, and it shows that an unmodified parser runs on the NONOS std layer.

## Role

A std application: `Capsule.mk` sets `CAPSULE_BUILD_STD := std,panic_abort` and `src/main.rs` calls
`nonos_app_skeleton::run_loop`. The skeleton waits for a focus control frame from `desktop_shell`
(`userland/app_skeleton/src/runner/idle.rs`) before opening the window. The `Cargo.toml` header says
it ships only through the on-disk package store and is never embedded in the kernel image.

## Capabilities

`CAPSULE_REQUIRED_CAPS := 0x1859`, commented in `Capsule.mk` as
`CoreExec|IPC|Memory|FileSystem|GraphicsDisplayQuery|GraphicsSurfaceCreate`:

- `0x0001` CoreExec: run.
- `0x0008` IPC: vfs, compositor, wm, input_router and toolkit.
- `0x0010` Memory: heap for the document and laid out lines.
- `0x0040` FileSystem: the `vfs::stat` and `vfs::read_file` calls in `src/mdview/load.rs`.
- `0x0800` GraphicsDisplayQuery and `0x1000` GraphicsSurfaceCreate: its window surface.

Endpoints: service `service:4922:app.mdview`, reply `reply:4923:endpoint.app.mdview.reply`. The
domain is `com.example`.

## Interface

It serves no IPC requests of its own. `src/mdview/load.rs` stats `/readme.txt` and refuses a
directory or anything over 64 KiB. It reads with a limit one byte past 64 KiB, so a file the read
cut short is refused rather than shown in part (`src/mdview/verdict.rs`), and it requires UTF-8.
`src/mdview/doc.rs` retries a failed load at most three times. A file that parses to no blocks,
empty or only blank lines, is named in the window as `mdview: /readme.txt is empty` and not read
again. `src/mdview/layout/parse.rs` turns pulldown-cmark events into `Block`s,
`src/mdview/layout/wrap.rs` wraps them to the window width using the TrueType measure in
`src/mdview/measure.rs`, and `src/mdview/paint.rs` with `src/mdview/draw.rs` paints the lines. A
load error is painted in place of the document. Esc closes the window (`src/mdview/event.rs`).

## State and privacy

It holds the parsed blocks and wrapped lines in memory and re-wraps when the width changes
(`src/mdview/app.rs`). It reads exactly one path, `/readme.txt`, and writes nothing. It holds no
Network or Debug bit.

## Build and test

- `make nonos-mk-mdview` builds the ELF; `make nonos-mk-mdview-sign` produces the certificate,
  manifest and trailer.
- `userland/capsule_mdview/layout_tests` is a host crate that includes `src/mdview/layout/` and
  `src/mdview/verdict.rs` through `#[path]` and tests them with `cargo test` against fixtures
  (`tests/markdown.rs`, `tests/nested.rs`, `tests/readme.rs`, `tests/empty.rs`). Its name does
  not end in `_proofs`, so `nix flake check` does not run it (`tools/nix/checks.nix`).

## Not done yet

- There is no scrolling: `src/mdview/paint.rs` stops drawing at the bottom edge, so a longer
  document is cut off.
- The path is the constant `PATH` in `src/mdview/load.rs`; no other file can be opened.
- `mk/40-run.mk` does not pack `mdview` into the store image, and no launch path for it was found
  outside this directory and the signed artifacts under `nonos-data/trust/capsules/`.
- `src/mdview/layout/parse.rs` has styles only for three heading levels, body, bullet and code;
  emphasis, strong text and links come out as plain body text.

Handbook: [app model](../../docs/handbook/desktop/app-model.md),
[capsule catalogue](../../docs/handbook/apps/capsule-catalog.md).
