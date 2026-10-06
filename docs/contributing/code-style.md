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
