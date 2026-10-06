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
