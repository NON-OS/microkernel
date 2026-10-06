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

## Comments

Say why, not what the next line does. A module opens with a `//!` comment that says what the module is for and what it is not for; the one in `src/arch/time_counter.rs:17-22` names what `x86_64` and `aarch64` read and why callers must not use it as wall-clock time.

- An `unsafe fn` documents its contract in a `# Safety` section, as `enable_interrupts` does (`src/arch/abi.rs:42-46`).
- An `unsafe` block has a `// SAFETY:` comment above it naming the fact that makes it sound, as the call to `rdrand_u64` does (`src/arch/cpu_random/read.rs:33-36`).

No check requires a `SAFETY:` comment, so write one for every new `unsafe` block. `tools/nonos_console.py` counts `unsafe {` sites and `SAFETY:` comments across the tree and prints the second as a share of the first, counted into `documented` (`tools/nonos_console.py:1319-1324`).

Shipping comments carry no markers of unfinished work. The `hygiene` scan rejects the four in `COMMENT_PATTERNS`, among them `FIXME`, `for now` and `placeholder`, in any line that starts as a comment (`nonos-verify/src/hygiene/patterns.rs:28-33`).

## Errors, panics and admissions

Shipping code does not panic. The same scan fails on every pattern in `CODE_PATTERNS`: `.unwrap(`, `.expect(`, `panic!`, `todo!(`, `unimplemented!(`, `unreachable!(` and `#[allow(dead_code)]` (`nonos-verify/src/hygiene/patterns.rs:18-26`). Shipping means the trees in `root_dirs`, the kernel, `userland` and four boot and attestation crates, less what `skip` leaves out: build output, proof crates, tests, vendored and upstream sources, and `build.rs` files (`nonos-verify/src/hygiene/roots.rs:5-31`). Return an error the caller can act on, and when bring-up cannot go on, say on the console why. The scan fails at this commit; [Tests and proofs](tests-and-proofs.md) says where.

Three gates hold sites against a [baseline](../overview/glossary.md#baseline) that may only shrink:

- Lint switches: every `#[allow(` or `#![allow(` in `src` and `userland` matches `MARK` and is listed in `scripts/baselines/allows.txt` (`scripts/check_allows.py:31-34`).
- Admissions: a word such as stub, unsupported or not implemented in shipping Rust matches `MARK` and is listed in `scripts/baselines/stubs.txt` (`scripts/check_stubs.py:30-32`). The gate matches words, not intent: a refusal that names the chip it will not drive passes, and one that says "not implemented" is listed.
- Unreachable exports: a kernel `pub fn` that no other file mentions outside an import is found by `unreachable` and listed in `scripts/baselines/unreachable.txt` (`scripts/check_unreachable.py:26-35`). Its docstring gives the reason: a mechanism with no caller can be reviewed, merged and shipped without one line of it running.

Every [proof crate](../overview/glossary.md#proof-crate) must pass `cargo clippy` with `-D warnings` over all its targets, except the crates named in `lintLib` and `lintNone`, two lists that only shrink and that a new crate never joins (`tools/nix/checks.nix:44-62`). Three proof crates fail that at this commit.

## Architecture boundaries

Shared kernel code reaches the CPU through the `ArchOps` trait and the `Arch` alias, not through `crate::arch::x86_64` paths (`src/arch/abi.rs:17-36`). Two counts are meant only to shrink:

- `cfg(target_arch` sites outside `src/arch`, counted into `cfg_count` against 116 in `nonos-ci/baselines/cfg-target-arch-count.txt` (`nonos-ci/run-static-checks.sh:46-47`).
- `crate::arch::x86_64::` paths outside `src/arch`, counted into `arch_leak_count` against 100 in `nonos-ci/baselines/arch-x86_64-uses.txt` (`nonos-ci/run-static-checks.sh:55-60`).

Both have grown past their baselines. The same `grep` the script runs counts 234 and 135 at this commit, so `static-tree` fails on both. Do not add to either.

[Architectures](../architectures/README.md) describes the boundary.

## Dependencies

The supply-chain job runs `cargo deny check` against `deny.toml` through `run_logged` (`nonos-verify/src/supply_chain.rs:25-27`). The header of `deny.toml` says the policy is written for the kernel crate.

- Licences: the `allow` list holds the project's own AGPL-3.0 and 0BSD, Apache-2.0 (also with the LLVM exception), BSD-2-Clause, BSD-3-Clause, CC0-1.0, ISC, MIT, MIT-0, MPL-2.0, Unicode-3.0, Unicode-DFS-2016, Unlicense and Zlib (`deny.toml:29-49`).
- Bans: the `deny` list refuses `openssl`, `openssl-sys` and `time` older than 0.3, and `wildcards` refuses wildcard version requirements (`deny.toml:58-70`).
- Sources: the `sources` table admits crates.io and one git repository, `NON-OS/STARKs` (`deny.toml:74-78`).

Moving the STARKs pin goes through its own pull request; [Review](review.md) says how.

## See also

- [Tests and proofs](tests-and-proofs.md)
- [Review](review.md)
- [Commits](commits.md)
- [Architectures](../architectures/README.md)
- [Writing a driver](../drivers/writing-a-driver.md)
- [Contributing](README.md)
