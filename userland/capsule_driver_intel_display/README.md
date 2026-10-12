# capsule_driver_intel_display

## Role

Intel display engine capsule. It takes over firmware's mode on Broadwell
(Gen8), Skylake to Comet Lake (Gen9), Ice Lake (Gen11) and Tiger Lake to
Raptor Lake (Gen12, with Alder Lake P and Raptor Lake P's display 13), and
sets modes on Gen9. Today this directory holds the engine code and the
device id tables only: `src/engine` (takeover, plane programming, vblank
flips, the cursor plane, the GGTT, and Gen9's mode setting: power wells,
DPLLs, DDIs, DisplayPort and HDMI, eDP power and backlight, watermarks),
`src/dp` (DisplayPort over AUX), `src/family.rs` with one id file per
generation in `data/`, and `src/dmc.rs`, the DMC firmware seal. There is
no `Cargo.toml`, `Capsule.mk`, Cargo feature or kernel mirror yet, so no
image carries it; `intel_display_proofs` runs the engine code against a
register-level model of the Gen9 display engine. The capsule binary that
claims the function, maps BAR0 and serves the display wire lands once the
broker's display-primary rules (L18 (c)) and write-combined MMIO mappings
reach this track.

## Microkernel contract

The promoted capsule declares `CAPSULE_REQUIRED_CAPS` with the display
mask of PLAN-coverage section 12 plus Dma, because the engine scans out of
granted memory (expected 0x1B9018, virtio-gpu's, to be confirmed by the
lead), never Irq, Network or InputSource. It uses brokered `MkDeviceList`,
`MkDeviceClaim`, `MkMmioMap` for BAR0 (registers and GGTT) and `MkDmaMap`
for the scanout surfaces, inside its own IOMMU domain with the graphics
RMRR kept mapped.

## Interface contract

Planned: NVGP v1 (`nonos_gfx_wire`) as `driver.intel_display0`, CAPS kind 7,
one output per lit pipe (pipe A is output 0), HW_CURSOR and DEVICE_READS.
No endpoint is exported today.

## Authority

None granted: nothing spawns this capsule yet.

## Privacy and persistence

Pixels pass through; nothing is read back or persisted.

## Runtime lifecycle

Planned: claim, read what firmware lit (no writes), map two surfaces high
in the GGTT clear of firmware's, point the primary plane at the first, then
flip at each vertical blank. On exit, firmware's plane registers are put
back and latched before the driver's GGTT entries are removed.

## Failure model

Every engine check runs before its first write, so a refusal leaves the
display as it was: a tiled or rotated firmware plane, a timing with sync
outside its blanking, a bad address or stride, a GGTT with no room. Every
wait is bounded by a clock deadline; a pipe that never blanks times out with
the old surface still shown. A machine whose id is not in the table stays
on the boot framebuffer.

## Current implemented surface

`src/engine`: `takeover::read_outputs`, `plane::program`, `flip::flip`,
`vblank::wait_vblank`, `cursor::set`, `ggtt::{map, unmap}`,
`ggtt_range::{entries, pick}`, `Scanout::{start, present, stop}`.
`engine::modeset` (Gen9 DP and HDMI), `engine::dc::dc_off`,
`engine::gen::{GEN8, GEN9, GEN11, GEN12_3, GEN12}`.
`src/ids.rs`: `family(vendor, device)` over the id tables; `Family::gen`.
`src/dmc.rs`: `blob(family)`, `admit(family, bytes)` against the sealed
digests in `data/dmc_sealed.txt` (empty: no DMC loads, DC5 and DC6 stay
off).

## Wire format

```text
compositor --NVGP v1--> driver.intel_display0 (planned)
                           | Mmio trait over BAR0
                           v
        transcoders, pipes, planes, cursor  +  GGTT -> IOVA pages
```

## State ownership

The engine owns the primary plane and cursor of each pipe it takes and the
GGTT runs it maps; it never writes a transcoder, DPLL, port or power well
in the takeover (unit 8 does mode setting).

## Operating rules

Register offsets and bits are cited to the family's PRM (Skylake's Volume
2c for most) in each file, or marked unverified. A DMC blob loads only when
its SHA-256 is sealed for its family. No other driver's code is a source.

## Release target

A Gen9 laptop drives its panel at firmware's mode through this capsule,
without tearing while windows move, with a hardware cursor.

## Release evidence

`[DISPLAY] driver.intel_display0`.

## Release checklist

- Capsule crate, `Capsule.mk`, feature, mirror, inventory and spawn entries.
- Device ids checked against Intel's published lists.
- Boot on Skylake, Kaby Lake, Coffee Lake and Comet Lake machines, then on
  Broadwell, Ice Lake, Tiger Lake and Alder Lake machines at firmware's mode.

## Explicit non-goals today

Mode setting beyond Gen9 (the combo and Type-C PHYs, Haswell's WRPLLs),
Haswell, Meteor Lake and Lunar Lake, several outputs on the wire (unit 10),
render acceleration, loading a DMC blob into the engine.

## Verification

```sh
cd userland/intel_display_proofs
RUST_TEST_THREADS=1 cargo test --release --config profile.release.overflow-checks=true
cargo clippy --release --all-targets -- -D warnings
```
