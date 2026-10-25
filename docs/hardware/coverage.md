# Hardware coverage map

Which parts that PCs, laptops and servers shipped from 2017 to 2026 NONOS can drive, which it
cannot, and who closes each gap. It is the list of what to write and what to boot next; the
[support matrix](MATRIX.md) is the list of what has been shown to work. The engineering
view, with each driver's blockers and the procedures per driver, is the hardware-breadth
session's [HANDOFF/hardware](../../HANDOFF/hardware/README.md); the owners here match it.

## How to read it

Each row is a part, or a family that one driver serves, with:

- **Priority.** P0: a common machine cannot boot to a working desktop, or loses its only
  network or disk. P1: common hardware with a driver that has never run on a real part, or a
  driver missing for a common part that has a workaround. P2: less common, or a feature
  short of the whole device. P3: servers and niche parts. A dash: nothing to do but report.
- **How common.** Most machines, common, or servers and niche, among machines of its kind.
- **Ids.** PCI vendor:device in hex, a class code, a USB class or an ACPI id. For a family,
  representative ids rather than every one.
- **Named by the driver.** How many of those ids the driver's own match table names, read
  from the tree. "By class": the driver matches the class code, not ids. "No driver": no
  table in the tree names the part.
- **In Standard.** Whether the Standard image carries the driver: "yes" or "no" by the
  kernel's Cargo features, "listed, unsealed" when it is listed but its trust files are not
  committed, "kernel" for kernel code every image has, and "no feature" for a
  driver no build can include.
- **Owner.** The session that closes the gap: A (kernel and driver hardening), B (drivers and
  the block wire), E (Wi-Fi and Bluetooth), F (ACPI and platform), G (graphics), hw (this map,
  new drivers, real-machine reports).

The two columns read from the tree are written by `tools/hardware/coverage.py`, from
[coverage-parts.tsv](coverage-parts.tsv). Change the parts file, run the script and commit
both; `python3 tools/hardware/coverage.py --check` fails while a page is out of date.

## The areas

- [Wired Ethernet](coverage-net.md)
- [Wi-Fi](coverage-wifi.md)
- [Storage](coverage-storage.md)
- [USB](coverage-usb.md)
- [Audio](coverage-audio.md)
- [Input, EC and battery](coverage-input.md)
- [Platform and IOMMU](coverage-platform.md)
- [Graphics](coverage-gpu.md)

## The P0 gaps, 25 October 2026

1. **No IOMMU in service.** Firmware with VT-d or AMD-Vi off refuses every claim and the boot
   stops at the splash. The no-IOMMU posture (hw-1) boots it unconfined and marked.
2. **Drivers listed and not sealed.** e1000e (Intel I219, the onboard port of most Intel
   machines), igb, sdhci and ufs are in full-gui with no trust files committed, so a sealed
   Standard image cannot carry them until the owner seals them; igc (I225/I226) has no
   feature at all. The USB hub driver joined Standard on 25 October.
3. **Intel Wi-Fi.** AX200, AX201 and 9560, in most Intel laptops from 2018 to 2022, are
   recognised and never claimed.
4. **Intel VMD.** The default storage mode on 11th-gen and later laptops; enumerated by the
   kernel and never booted on a real machine.

Every P0 needs a real machine to close; [the real-hardware kit](real-hardware-kit.md) says
which ones and how.

## Not covered here

CPU and SMP bring-up, firmware (UEFI, Secure Boot, TPM) and aarch64 boards are in
[Architectures](../architectures/README.md) and track C's letters. Thunderbolt and USB4
docking beyond xHCI ordering, cameras, fingerprint readers and card readers are left for the
next pass.
