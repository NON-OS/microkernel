# Code style

The rules a NONOS change is held to, which of them a check enforces, and where that check lives.

## Formatting

`rustfmt.toml` sets three options: `edition` 2021, Unix line endings through `newline_style`, and `use_small_heuristics` at `Max` (`rustfmt.toml:1-3`). `.editorconfig` asks every file for LF line endings, a final newline, UTF-8, four-space indents and no trailing whitespace through `trim_trailing_whitespace` (`.editorconfig:1-8`).

`make nonos-mk-fmt` runs `cargo fmt` over the kernel crate and then inside `BOOTLOADER_DIR` (`mk/50-ci.mk:103-105`). It formats whole crates. If that touches files your change does not, leave those out of your commit; [Commits](commits.md) says where a formatting pass goes.

```
make nonos-mk-fmt
```

Not tested in this release.

CI checks formatting on one crate only. `nonos-verify` calls `run_logged` with `cargo fmt --check` on its own manifest, and its comment gives the reason the kernel is left out: rustfmt cannot resolve the kernel's cfg-gated architecture modules (`nonos-verify/src/build.rs:14-22`).

## File header

Source files open with the licence notice, and a new file carries it too. Rust files carry it as `//` comments; Python and shell files carry it as `#` comments after the shebang line, as `tools/arm_kernel_report.py` does. Copy it whole from an existing file, for example `src/lib.rs`:

```
// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.
```

The root `Makefile` uses a single SPDX line instead. No check requires the header. At this commit 7 of the 5761 Rust files under `src/` carry the short form, an SPDX line under the copyright line, and 1222 Rust files under `userland/`, outside its `vendor` and `upstream-src` trees, carry neither form.

## One concern per file

A file holds one concern and stays at or under 75 lines, and a `mod.rs` only declares modules and re-exports names. `src/arch/aarch64/mod.rs` is a plain example: module declarations, then `pub use` lines, nothing else. When a file would grow past the limit, move a part into a child module; `table_geom.rs` does that with `table_span` and says why (`userland/capsule_process_manager/src/pm/ui/table_geom.rs:23-27`).

The static checks enforce this in the bootloader only:

- A `mod.rs` under `nonos-bootloader/src` may hold comments, blank lines, attributes, `mod` declarations and `pub use` re-exports, or `boot_mod_bodies` fails the check (`nonos-ci/run-static-checks.sh:2217-2237`). A plain `use` is not allowed there.
- Those `mod.rs` files stay at or under 75 lines (`boot_mod_oversize`, `nonos-ci/run-static-checks.sh:2240-2246`).
- Files under `nonos-bootloader/src/entry` stay at or under 75 lines and carry no comment after line 15, where the header ends (`boot_entry_oversize`, `boot_entry_comments`, `nonos-ci/run-static-checks.sh:2249-2264`).
- The bootloader's kernel verification module is held to the same two limits (`kernel_verify_oversize`, `nonos-ci/run-static-checks.sh:2267-2282`).

Everywhere else the limit is a house rule with no check behind it, and older code does not meet it: at this commit 790 of the 5761 Rust files under `src/` are longer than 75 lines.
