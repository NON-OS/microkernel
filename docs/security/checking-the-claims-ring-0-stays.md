# Checking the security claims yourself: Ring 0 stays within its budget

Part of [Checking the security claims yourself](checking-the-claims.md).

## Ring 0 stays within its budget

`tools/nonos-tcb` counts what runs in ring 0. It takes the kernel source files
that rustc's dep-info for the newest kernel build lists, and counts their
non-comment lines under `src/` (`kernel_files` and `code_lines`,
`tools/nonos-tcb:52-60`). A file in that set counts whole, code a `cfg` removes
included, so the number is an upper bound; generated files, `include_bytes!`
payloads and path crates are left out (`tools/nonos-tcb:17-29`).

```sh
make nonos-mk-capsules
make nonos-mk-tcb
```

Not tested in this release.

`nonos-mk-tcb` holds the count to `TCB_BUDGET`, by default
`nonos-ci/baselines/tcb-x86_64-capsules.txt`, then runs
`tools/nonos-proof-coverage` (`mk/40-run.mk:358-364`). Each budget is a
[baseline](../overview/glossary-amnesic-boot.md#baseline) that moves one way: it
is lowered when ring 0 shrinks, and a change that raises it has to say why
(`nonos-ci/baselines/README.md`). `tcb-x86_64-capsules.txt` holds 132664 lines
for the `microkernel-capsules` kernel, and CI gates on it.
`tcb-x86_64-desktop-prod.txt` holds 132739 for the `nonos-mk-desktop-gui-prod`
kernel, the desktop built to boot under QEMU, and no target or workflow reads
it; to check it, build that kernel and pass
`TCB_BUDGET=nonos-ci/baselines/tcb-x86_64-desktop-prod.txt`. The full hardware
kernel, `nonos-mk-zerostate`, which the make files call the image that ships,
has no budget (`nonos_kernel_build`, `mk/20-build.mk:1187-1197`).

A pass prints `[tcb] <files> files, <lines> lines of ring 0 code` with the
dep-info file's name, one line per top-level module, then `[tcb] baseline
132664, delta` and the difference, and exits 0. A count above the budget prints
`::error::tcb grew past its baseline` and exits 1 (`held`,
`tools/ratchets/budget.py:21-35`). Without a built kernel it prints `no kernel
dep-info; build the kernel first` and exits 2 (`kernel_files`,
`tools/ratchets/kernel_files.py:26-31`), which is what it printed in this tree.

`nonos-verify build` runs `make nonos-mk-capsules`, then this count against the
capsules budget, as its `tcb-budget` check (`run_logged`,
`nonos-verify/src/build.rs:41-57`). The `verify` job of `ci-build.yml` runs
`nonos-verify build` (`.github/workflows/ci-build.yml:133-166`), and the `ci`
workflow calls it on every pull request and every push to `main` and `develop`
(`.github/workflows/ci.yml:7-10`). The flake does not run it. No count from a
built kernel is recorded for this commit.
