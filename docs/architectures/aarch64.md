# aarch64

What exists of the aarch64 port of NONOS, how to build it and boot it under QEMU, and what is missing before it is a system you can install.

## Status

aarch64 is a preview. The kernel builds only with the `nonos-arch-preview` feature, which `compile_error!` demands for every architecture but x86_64 (`src/lib.rs:28-35`). It targets QEMU's `virt` board; there is no loader for it and no release image, and it has not been tested on real hardware in this release. CI builds it and boot-tests it on every pull request: `ci.yml` runs on each `pull_request` and calls both aarch64 workflows (`.github/workflows/ci.yml:7-35`). Whether those jobs pass at this commit is not recorded here.

The missing pieces below are read from the code. No page here gives a date for them.

## Build and boot under QEMU

```
make nonos-mk-arm
make nonos-mk-arm-run
```

Not tested in this release.

- Outside the flake's shell, a `nonos-mk-*` target enters `nix develop` first, so these need nothing but Nix (`NONOS_IN_FLAKE`, `Makefile:144-147`).
- `nonos-mk-arm` builds the kernel with `ARM_KERNEL_BUILD_FLAGS` and the `microkernel-core` and `nonos-arch-preview` features, and sets `NONOS_USER_TARGET` so the kernel embeds [capsules](../overview/glossary.md#capsule) built for `aarch64-nonos-user` (`mk/20-build.mk:856-862`).
- The build signs the embedded manifest with an Ed25519 seed. When the file named by `SIGNING_KEY` does not exist, make writes 32 random bytes there first (`mk/10-qemu.mk:163-167`). That seed is for local images only.
- `nonos-mk-arm-run` boots the ELF with `ARM_QEMU_FLAGS`: `-M virt,gic-version=3 -cpu max -m 512 -nographic -serial mon:stdio -device virtio-rng-pci` (`mk/20-build.mk:885-901`). The comment above it gives the reasons: the kernel drives a GICv3 and `virt` defaults to a GICv2, `-cpu max` makes the feature-guarded paths run, and the RNG refuses to seed without an entropy device.

The kernel lands at `target/aarch64-nonos/release/nonos-kernel`. `tools/arm_kernel_report.py` reads it without a cross toolchain and prints its ELF header, segments, section sizes, symbols, the entry path from `_start` to `microkernel_main`, and the capsules built for `aarch64-nonos-user` (`report_entry_path`, `tools/arm_kernel_report.py:284-299`).

```
python3 tools/arm_kernel_report.py --fast
```

With no kernel built, it says so, prints `build one with: make nonos-mk-arm` and exits with status 1 (`KERNEL`, `tools/arm_kernel_report.py:370-374`).

### The desktop

`nonos-mk-arm-gui` builds the desktop base capsules for `aarch64-nonos-user` and a kernel with `microkernel-desktop-base` (`NONOS_USER_TARGET`, `mk/20-build.mk:1171-1182`). `nonos-mk-arm-gui-run` boots it with `ARM_GUI_QEMU_FLAGS`: a virtio GPU, keyboard, tablet and RNG, and 2048 MB (`mk/20-build.mk:926-937`). The kernel gets no device tree on this path either, so it uses its 512 MB default rather than the 2048 MB QEMU provides; the list at the end of this page says why. No CI job boots the desktop, and this release has not tested it.

```
make nonos-mk-arm-gui-run
```

Not tested in this release.
