# Capsule isolation: Drivers: the broker and the IOMMU

Part of [Capsule isolation](capsule-isolation.md).

## Drivers: the broker and the IOMMU

A driver is a capsule that holds broker bits, and each broker call asks for its
own bit, starting with `can_device_enum` for the device list
(`src/syscall/contract/cap_table/mk_device.rs:24-38`).

| Bit | Calls | What it grants |
|---|---|---|
| `DeviceEnum` | `MkDeviceList` | the device list |
| `Driver` | `MkDeviceClaim`, `MkDeviceRelease`, `MkPciConfigRead`, `MkPciConfigWrite` | one device, and its config space within the allowlist |
| `Mmio` | `MkMmioMap`, `MkMmioUnmap` | a slice of a claimed device's BAR |
| `Irq` | `MkIrqBind`, `MkIrqUnbind`, `MkIrqWait`, `MkIrqPoll`, `MkIrqAck` | the device's interrupt |
| `Dma` | `MkDmaMap`, `MkDmaUnmap` | a zeroed buffer the device may read and write |
| `Pio` | `MkPioGrant`, `MkPioRead`, `MkPioWrite`, `MkPioRelease` | kernel-run port I/O on the device, x86_64 only |

`claim` refuses a device another capsule already holds, moves the device into
the claiming capsule's [IOMMU domain](../overview/glossary-hardware-broker.md#iommu-domain)
before powering it, and sets it so that no request it makes is no-snoop
(`src/hardware/broker/claim/claim.rs:23-45`). The domain maps nothing until
`MkDmaMap` grants a buffer, so the device reaches that capsule's buffers and
faults on everything else (`src/hardware/broker/confine/mod.rs`).

Confinement depends on the hardware. `detect` selects Intel VT-d when the
firmware's DMAR table yields a remapping unit, and with only an IVRS table
selects AMD-Vi (`src/arch/x86_64/iommu/backend/select.rs`). Both are driven
in `microkernel-core`, which every [build
profile](../overview/glossary-boot-root-record.md#build-profile) builds on, each with interrupt
remapping (`nonos-iommu-intremap`, `nonos-iommu-amdvi`). A device found at
boot that no driver claims is blocked, but for the memory firmware reserved
for it (RMRR, IVMD), which it keeps one to one.

Where confinement is not possible:

- With no remapping unit in service, or no interrupt remapping, a claim is
  refused (`src/hardware/broker/confine/decide.rs`, `interrupts.rs`), unless
  the kernel is built with `nonos-dev-unconfined-dma` or
  `nonos-dev-unremapped-interrupts`, which a release build refuses. Under the
  first the device then reaches all of memory.
- A device that no unit in service `translates` is refused the same way
  (`src/hardware/broker/confine/attach.rs`).
- A device with no PCI requester id, such as a controller found through ACPI,
  gets no domain, and its claim prints no line: `pci_address` returns nothing
  for it (`src/hardware/broker/confine/table.rs:35-43`).
- With a unit in service, any other failure to give the device a domain refuses
  the claim.

Each claim on a PCI device prints a line on the serial console that starts with
`[VT-D]` and says `confined`, `unconfined` or `refused` with the reason. It
names neither the process nor the device's address, since the log leaves the
machine (`say` in `src/hardware/broker/confine/table.rs:50-55`). `posture_line` prints the summary
as `[IOMMU] <vendor> present, enforcing=<0|1>, unconfined grants=<n>`
(`src/memory/iommu/posture_line.rs`). Device DMA is confined only when that
line shows `enforcing=1` and `unconfined grants=0`. On an image built with
`capsule-serial-debug` (the standard, qemu and dev profiles) the kernel keeps a
copy of the serial console in memory, and the Terminal command `log iommu vt-d`
shows these lines from it; a hardened or air-gapped image keeps no copy (`keep`
in `src/sys/serial/tail.rs:51-54`).

The other broker paths are narrow too:
