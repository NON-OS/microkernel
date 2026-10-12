# capsule_driver_bootfb

## Role

The boot framebuffer as a display driver of its own (PLAN G 5): the linear
framebuffer firmware left (UEFI GOP on x86, a device tree's
`simple-framebuffer` on aarch64) served on the display wire at its one
mode, so the compositor holds no device memory at all. Today this
directory holds the display half only (`src/fb.rs`, `src/describe.rs`,
`src/display.rs`, `src/copy_in.rs`), served through `nonos_gfx_serve`;
there is no `Cargo.toml`, `Capsule.mk`, feature or kernel mirror yet, so no
image carries it. `bootfb_proofs` runs it through the wire server against
a framebuffer in host memory.

## Microkernel contract

The promoted capsule declares `CAPSULE_REQUIRED_CAPS` with section 12's
VRAM display mask (0x39018), no Dma (nothing reads guest memory but the
CPU), never Irq, Network or InputSource. It uses brokered `MkDeviceList` to
find the boot framebuffer's record, `MkDeviceClaim`, and `MkMmioMap` with
`MK_MMIO_MAP_WC` for the framebuffer's range.

## Interface contract

Planned: NVGP v1 as `driver.bootfb0`, CAPS kind 6, one output, no flags (no
MODE_SET, no HW_CURSOR), its one mode listed as preferred, current and
from the device. It is last in `DISPLAYS`: native drivers and the
paravirtual ones come first.

## Authority

None granted: nothing spawns this capsule yet.

## Privacy and persistence

Pixels pass through; nothing is read back or persisted.

## Runtime lifecycle

Planned: claim the framebuffer record, map it write-combined, serve the
wire; on exit the kernel's own present path takes the framebuffer back.

## Failure model

A framebuffer that is not 32-bit blue-first or red-first, a mapping too
small for its mode, or a stride shorter than a line is refused at
mapping: the compositor keeps its GOP path. FLUSH rectangles outside the
mode are refused by the server before any copy.

## Current implemented surface

`Fb::new` (the checked mapping), `Fb::copy_in`, `describe::{caps, mode,
state}`, and `BootFb`, the `nonos_gfx_serve::Display` the capsule serves.
`bring_up::bring_up` runs the kernel's order on the boot framebuffer's record
(lead L31: firmware bus 5 device 0x0001, or on aarch64 C's device-tree
simple-framebuffer node, bus 4 device 4 with aux 0; class display 0x0030, one prefetchable
MMIO bar from the first pixel's page to the last row's end, aux the first
pixel's offset): `driver.bootfb0` announced, the claim, `mk_boot_framebuffer`'s
24-byte mode (`geometry::decode`), bar 0 mapped; any failure releases the
claim. Three-byte pixels are refused with one line. It runs over the
`firmware::Firmware` trait; the binary's is `libc_firmware::Libc`
(`mk_service_register` of `driver.bootfb0` on port 4572, `mk_device_claim`,
`mk_boot_framebuffer`, `mk_mmio_map` of bar 0 write-combined), found by
`discover::find` and served by `serve::run`.

## Wire format

```text
compositor --NVGP v1--> driver.bootfb0 (planned)
                          | nonos_gfx_serve::Server
                          v
            shadow (ARGB8888) --copy, swap R/B if red first--> framebuffer
```

## State ownership

The capsule owns the framebuffer mapping while it serves and the primary
the server allocates; the kernel stops writing the framebuffer while the
capsule holds its claim.

## Operating rules

No mode setting: the firmware's mode is the only one. Every copy is
fenced before FLUSH answers.

## Release target

Every lane where no other display driver answers shows
`[DISPLAY] driver.bootfb0`; an NVIDIA or unknown GPU keeps its desktop.

## Release evidence

`bootfb_proofs` passing; the owner's boot on a GOP-only machine.

## Release checklist

- [x] Capsule crate (binary `driver_bootfb`).
- [x] The kernel's boot framebuffer record and the present path handing over (L31).
- `Capsule.mk`, feature, mirror, inventory, spawn entries (after the seal).
- Owner's keys for the slug `driver.bootfb`.

## Explicit non-goals today

Mode setting, a hardware cursor, framebuffers other than 32-bit.

## Verification

```sh
cd userland/bootfb_proofs
RUST_TEST_THREADS=1 cargo test --release --config profile.release.overflow-checks=true
cargo clippy --release --all-targets -- -D warnings
```
