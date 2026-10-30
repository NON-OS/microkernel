# Hardware breadth: the support matrix and the plan (2026-10-24)

What NONOS drives on the PCs of the last ten years, measured on main at
1536ccef3, and the order the hardware-breadth session closes the gaps in. The
user-facing 0.9.2 matrix is docs/hardware/MATRIX.md; this is the engineering
view, every class against mainstream chips with its blocker. Procedures for
the owner's machines sit beside it, one per driver (procedure-*.md).

## Evidence, never rounded up

- **hw**: an owner hardware report shows it. One exists: the Gemini Lake
  laptop of 6 October 2026, image commit not recorded (docs/hardware/MATRIX.md).
- **booted**: a QEMU boot in the tree reaches it. No QEMU target in the tree
  asserts a device marker today: mk/40-run.mk calls tests/boot/*.sh, which
  is absent, and tools/nonos_qemu/lane_*.py add devices and check nothing.
  So "booted" below means "runs in the default QEMU machine", never more.
- **host**: a proofs crate runs the shipping driver source against a model.
  Counts are `#[test]` items (`grep -rc --include=*.rs`), not cargo runs.
- **design**: code or docs only.
State: **none**, **partial** (what is missing), **done** (what proves it).
"In the image" means both columns yes: listed in microkernel-full-gui
(Cargo.toml) and signed (its manifest in nonos-data/trust/capsules).
Commonality is an estimate for x86 desktops and laptops sold
2015 to 2025: **very common** (most machines of a vendor or era), **common**,
**niche**. It is an estimate, not a measurement; owner reports correct it.

## Parts

- [coverage-storage.md](coverage-storage.md): AHCI, NVMe, VMD/RST, xHCI, hubs, HID, MSC, SD
- [coverage-net.md](coverage-net.md): Intel and Realtek wired, Intel/Realtek/MediaTek Wi-Fi
- [coverage-gpu.md](coverage-gpu.md): GOP, i915-class, AMD, NVIDIA, PS/2, I2C-HID, HDA
- [coverage-platform.md](coverage-platform.md): ACPI tables, AML, battery, thermal, power, suspend, Thunderbolt, fuzzing

## What blocks the most machines

Two kinds of gap. The first needs no code, only the owner's keys and seal;
it unlocks more machines than any code can:

| Blocked on the owner | Machines it unlocks | Wired by |
|---|---|---|
| e1000e trust files and full-gui (I217/I218/I219) | very common: most Intel laptops and desktops 2013-2025 | B, b/25 |
| usb_hub trust files and full-gui | very common: any keyboard behind a hub, dock or monitor | B, b/47 |
| capsule_acpi seal, Admin-or-Firmware ruling (f/08) | very common: battery, thermal, lid, AML power button, desktop power-off | F |
| gpio seal (touchpad interrupts instead of polling) | common: every I2C-HID touchpad | F |
| ehci trust files (Haswell/Broadwell ports routed to EHCI) | common in 2014-2015 machines | B |
| sdhci trust files (listed in full-gui, unsigned) | common: eMMC laptops, SD slots | B |
| intel_display seal, bootfb's kernel record (g/88) | very common for native modes; GOP covers display meanwhile | G |

The second is code, in the order planned (each line names
the matrix row):

1. Display fallback, done on main: a firmware with no linear GOP mode is
   named by the loader, the kernel and the compositor, which keeps waiting
   for a driver (ad5e07483a; booted with no display device, the
   other reasons host-tested); a surface's cap follows the boot framebuffer
   from 64 MiB to 128 MiB, so 6K and 8K register (ea62f427f; booted
   on the normal lane, the raised range host-tested). Evidence in
   docs/hardware/evidence/2026-10-25/. (display-input-audio.md)
2. Wired: igc (I225/I226) into the kernel, scope row and spawn, after the
   Lead rules on its endpoints; rtl8169's per-revision PHY and ePHY setup
   from Realtek's datasheets; e1000e's NVM-driven LCD configuration.
3. Intel Wi-Fi: the 22000-family transport (AX200, AX201, 9560 on CNVi),
   the bulk of Intel laptops 2018-2022; only SO-platform AX211 boots today.
4. Power-off: the leave phase is on main (8a345b50e, host-tested, not
   booted): NVMe CC.SHN, AHCI flush and standby, flushes for the rest,
   within 15 s (6 s when critical), once per boot. Reached today by the
   reboot and admin shutdown paths; the desktop power-off waits on
   capsule_acpi, and running the leave before \_PTS(5) is a separate task.
5. xHCI: XUSB2PR/USB3_PSSEN switch-over done (host-tested); quirks as reports show.
6. HDA: Conexant, Cirrus and IDT codec setup, CORB/RIRB recovery and the
   codec-walk sweep done (host-tested); per-board pin fixups as boards report.
7. ACPI robustness: DMAR, IVRS and `_DSM` sweeps done (host-tested).
8. rtsx: 5249, 525A, 5260, 5261 bring-up; PS/2 touchpad protocols last.

Every unit follows the plan, writes from datasheets and published
specifications (Linux-derived code is kept out of delivery: e/06, b/29),
and lands with a proof and a mutant per rule and a procedure-*.md here.
Infra first: an asserting QEMU lane (a boot marker per device) so "booted"
means something; the default QEMU devices e1000e, igb, nvme, ahci, xhci,
ehci and intel-hda are what it covers.
