# Capsule isolation: Drivers: the broker and the IOMMU

Part of [Capsule isolation](capsule-isolation.md).

## Drivers: the broker and the IOMMU

A driver is a capsule that holds broker bits, and each broker call asks for its
own bit, starting with `can_device_enum` for the device list
(`src/syscall/contract/cap_table/mk_device.rs:24-37`).

| Bit | Calls | What it grants |
|---|---|---|
| `DeviceEnum` | `MkDeviceList` | the device list |
| `Driver` | `MkDeviceClaim`, `MkDeviceRelease`, `MkPciConfigRead`, `MkPciConfigWrite` | one device, and its config space within the allowlist |
| `Mmio` | `MkMmioMap`, `MkMmioUnmap` | a slice of a claimed device's BAR |
| `Irq` | `MkIrqBind`, `MkIrqUnbind`, `MkIrqWait`, `MkIrqPoll`, `MkIrqAck` | the device's interrupt |
| `Dma` | `MkDmaMap`, `MkDmaUnmap` | a zeroed buffer the device may read and write |
| `Pio` | `MkPioGrant`, `MkPioRead`, `MkPioWrite`, `MkPioRelease` | kernel-run port I/O on the device, x86_64 only |

`claim` refuses a device another capsule already holds, moves the device into
the claiming capsule's [IOMMU domain](../overview/glossary.md#iommu-domain)
before powering it, and sets it so that no request it makes is no-snoop
(`src/hardware/broker/claim/claim.rs:23-45`). The domain maps nothing until
`MkDmaMap` grants a buffer, so the device reaches that capsule's buffers and
faults on everything else (`src/hardware/broker/confine/mod.rs`).

Confinement depends on the hardware. `detect` selects Intel VT-d when the
firmware's DMAR table yields a remapping unit, and with only an IVRS table
selects AMD-Vi, which this kernel drives only when built with the
`nonos-iommu-amdvi` feature (`src/arch/x86_64/iommu/backend/select.rs:43-66`).
No [build profile](../overview/glossary.md#build-profile) in
`tools/nix/config.nix` turns that feature on, while VT-d enforcement
(`nonos-iommu-enforce`) is part of `microkernel-core`, which every profile
builds on. No profile turns on interrupt remapping (`nonos-iommu-intremap`)
either. A device found at boot that no driver claims stays in the VT-d [identity
domain](../overview/glossary.md#identity-domain).

Where confinement is not possible:

- `unconfined_allowed` lets a claim go ahead without a domain only when no
  remapping unit is in service: none was found, none came up, or the unit is
  AMD-Vi and not in service, which is always the case in a kernel built without
  `nonos-iommu-amdvi` (`src/hardware/broker/confine/posture.rs:32-50`). The
  device then reaches all of memory.
- A device that no unit in service `translates` also stays on physical addresses
  (`src/hardware/broker/confine/attach.rs:41-48`).
- A device with no PCI requester id, such as a controller found through ACPI,
  gets no domain, and its claim prints no line: `pci_address` returns nothing
  for it (`src/hardware/broker/confine/table.rs:35-43`).
- With a unit in service, any other failure to give the device a domain refuses
  the claim.

Each claim on a PCI device prints a line on the serial console that starts with
`[VT-D] pid=`, gives the device's address, and says `confined`, `unconfined` or
`refused` with the reason (`say` in
`src/hardware/broker/confine/table.rs:45-53`). `posture_line` prints the summary
as `[IOMMU] <vendor> present, enforcing=<0|1>, unconfined grants=<n>`
(`src/memory/iommu/posture.rs:69-79`). Device DMA is confined only when that
line shows `enforcing=1` and `unconfined grants=0`. On an image built with
`capsule-serial-debug` (the standard, qemu and dev profiles) the kernel keeps a
copy of the serial console in memory, and the Terminal command `log iommu vt-d`
shows these lines from it; a hardened or air-gapped image keeps no copy (`keep`
in `src/sys/serial/tail.rs:51-54`).

The other broker paths are narrow too:
