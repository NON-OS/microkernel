# capsule_driver_intel_display

## Role

Intel display engine capsule. It takes over firmware's mode on Haswell
(Gen7.5), Broadwell (Gen8), Skylake to Comet Lake (Gen9), Ice Lake (Gen11) and Tiger Lake to
Raptor Lake (Gen12, with Alder Lake P and Raptor Lake P's display 13), and
sets modes on Gen9. The crate's library is the engine with no kernel
calls in it: `src/engine` (takeover, plane programming, vblank flips, the
cursor plane, the GGTT, and Gen9's mode setting: power wells, DPLLs, DDIs,
DisplayPort and HDMI, eDP power and backlight, watermarks), `src/dp`
(DisplayPort over AUX), `src/family.rs` with one id file per generation in
`data/`, `src/dmc.rs` (the DMC firmware seal) and `src/display` (the engine
as a display-wire device). The binary (`src/main.rs`, `src/setup`) claims
the function, maps BAR0, scans out of two DMA-granted surfaces and serves
the display wire. There is no `Capsule.mk`, Cargo feature or kernel mirror
yet, so no image carries it: the spawn waits on the owner's seal, as bga's
does. `intel_display_proofs` runs the library against a register-level
model of the Gen9 display engine.

## Microkernel contract

The promoted capsule declares `CAPSULE_REQUIRED_CAPS` 0xB9018: section
12's VRAM mask 0x39018 plus Dma (0x80000), because the engine scans out of
granted memory; no Pio, Irq, Network or InputSource. It uses brokered
`MkDeviceList`, `MkDeviceClaim`, `MkPciConfigRead` (Command, and GGC at
0x50), `MkPciConfigWrite` (Memory Space and Bus Master on), `MkMmioMap`
for BAR0 uncached and for one write-combined window of the aperture
(BAR2) while firmware's picture is read, `MkDmaMap` (high,
write-combining) for the two scanout surfaces, and surfaces through
`nonos_gfx_pool`, inside its own IOMMU domain with the graphics RMRR kept
mapped. The kernel must not free a display function's scanout grants while
its engine may still read them (`keep_at_exit`, section 12).

## Interface contract

NVGP v1 (`nonos_gfx_wire`) as `driver.intel_display0`, CAPS kind 7, one
wire output per lit pipe, pipe A's first (up to four). Each output's
scanout surfaces live in its own window of the GGTT, stacked down from the
top with room for two surfaces of the largest mode its monitor offers and
all of them above firmware's surfaces; an output that does not fit, or
fails, is left as firmware lit it. On Gen9, an output firmware lit over
DisplayPort or HDMI through a pipe transcoder is a list device (MODE_SET,
no MODE_RANGE): MODES are the monitor's, and a MODE_SET maps and fills two
new surfaces from the new primary, darkens the output, relights its port at
the monitor's timing (its own detail, or the DMT's for a mode named by size
and rate), writes the plane's buffer and watermarks, and rebases the plane;
any failure puts the old mode back on the untouched old surfaces. eDP,
DVI and the other generations keep firmware's mode (no MODE_SET). No
HW_CURSOR yet (CURSOR answers ENOSYS), DEVICE_READS off because FLUSH
copies, `physical_mm`
from the monitor's EDID (Gen9: read over the lit port's AUX channel for
DisplayPort and eDP, over GMBUS for HDMI and DVI, the route found from the
transcoder's DDI function), MODES the shown mode, marked FROM_EDID when the
monitor lists it. STATE reports a
hung flip or an underrun as LOST after putting the plane back, and a
stopped transcoder as LOST, not scanning out. The service is not exported
until the capsule is spawned.

## Authority

None granted: nothing spawns this capsule yet.

## Privacy and persistence

Pixels pass through; nothing is read back or persisted.

## Runtime lifecycle

Claim and turn on bus mastering, map BAR0, read GGC for the GGTT's size,
switch DC5 and DC6 off (Gen9 on), read what firmware lit (no writes), map
two DMA surfaces, copy firmware's picture into both through the aperture,
map them high in the GGTT clear of firmware's, point the primary plane at
the first, then serve: each FLUSH copies into the back surface, flips at
the vertical blank, and copies the same rectangles into the new back.
`Scanout::stop` puts firmware's plane back and latches it before the
driver's GGTT entries are removed; the serving binary never stops, so the
kernel's hold on the grants (above) covers its exit.

## Failure model

Every engine check runs before its first write, so a refusal leaves the
display as it was: a tiled or rotated firmware plane, a timing with sync
outside its blanking, a bad address or stride, a GGTT with no room. Every
wait is bounded by a clock deadline; a pipe that never blanks times out with
the old surface still shown. A machine whose id is not in the table stays
on the boot framebuffer.

## Current implemented surface

`src/display`: `Intel` (the `nonos_gfx_serve::Display`), `Frames`, `copy`,
`red_first`, `Monitor` and `panel_of` (the monitor's EDID), `Modeset`,
`pick` and `matching` (timings), `Intel::change_mode`; `engine::route`,
`engine::relight::{relight, buffer}`, `Scanout::rebase`, `ggtt_run::map_run`. The binary: `discover::find`, `setup::run`, `bar::Bar0`
(the engine's `Mmio`), `clock::Uptime`, `frames::DmaFrames`, `serve::run`.
`src/engine`: `takeover::read_outputs`, `plane::program`, `flip::flip`,
`vblank::wait_vblank`, `cursor::set`, `ggtt::{map, unmap}`,
`ggtt_range::{entries, pick}`, `Scanout::{start, present, stop}`.
`engine::modeset` (Gen9 DP and HDMI), `engine::dc::dc_off`,
`engine::gen::{GEN7_5, GEN8, GEN9, GEN11, GEN12_3, GEN12}`, `ggtt_pte::Pte`.
`src/ids.rs`: `family(vendor, device)` over the id tables; `Family::gen`.
`src/dmc.rs`: `blob(family)`, `admit(family, bytes)` against the sealed
digests in `data/dmc_sealed.txt` (empty: no DMC loads, DC5 and DC6 stay
off).

## Wire format

```text
compositor --NVGP v1--> driver.intel_display0
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

`intel_display_proofs` passing; the owner's boot on a Gen9 laptop showing
`[DISPLAY] driver.intel_display0`.

## Release checklist

- `Capsule.mk`, feature, mirror, inventory and spawn entries (the crate is in).
- `driver.intel_display0` is in the kernel's `DISPLAYS`, before bootfb; the spawn joins it.
- Owner's keys for the slug `driver.intel_display`.
- Device ids checked against Intel's published lists.
- Boot on Skylake, Kaby Lake, Coffee Lake and Comet Lake machines, then on
  Haswell, Broadwell, Ice Lake, Tiger Lake and Alder Lake machines at firmware's mode.
- The owner's choice of DMC blobs, sealed by digest.

## Explicit non-goals today

Mode setting beyond Gen9 (the combo and Type-C PHYs, Haswell's WRPLLs),
Meteor Lake and Lunar Lake, several outputs on the wire (unit 10),
render acceleration, loading a DMC blob into the engine.

## Verification

```sh
cd userland/intel_display_proofs
RUST_TEST_THREADS=1 cargo test --release --config profile.release.overflow-checks=true
cargo clippy --release --all-targets -- -D warnings
```
