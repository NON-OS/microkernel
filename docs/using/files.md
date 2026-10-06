# Files

Where your files live on NONOS, how to work with them in Files, and exactly what is kept when the machine powers off.

## The short version

- Every file you see lives in the [file store](../overview/glossary.md#file-store), `vfs_pool`, which holds it in memory. At power off it is gone, unless it was written to the disk.
- NONOS forgets by default. The first mode setup offers is `Amnesic (default)`: RAM only, nothing written to any disk (`userland/capsule_setup_wizard/src/render/screens/mode.rs`). This is an [amnesic boot](../overview/glossary.md#amnesic-boot).
- Even on a system installed to a disk, Files and Editor do not write your files to the disk in this release. The Terminal's `keep` command is the one way to keep a file you made. See [What is kept after power off](#what-is-kept-after-power-off).
