# aarch64

What exists of the aarch64 port of NONOS, how to build it and boot it under QEMU, and what is missing before it is a system you can install.

## Status

aarch64 is a preview. The kernel builds only with the `nonos-arch-preview` feature, which `compile_error!` demands for every architecture but x86_64 (`src/lib.rs:28-35`). It targets QEMU's `virt` board; there is no loader for it and no release image, and it has not been tested on real hardware in this release. CI builds it and boot-tests it on every pull request: `ci.yml` runs on each `pull_request` and calls both aarch64 workflows (`.github/workflows/ci.yml:7-35`). Whether those jobs pass at this commit is not recorded here.

The missing pieces below are read from the code. No page here gives a date for them.
