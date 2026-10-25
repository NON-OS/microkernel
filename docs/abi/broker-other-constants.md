# Broker: Other constants

Part of [Broker](broker.md).

## Other constants

`BUS_KIND_*` says which bus a record came from, and `BAR_KIND_*` and
`BAR_FLAG_*` describe a `Bar`. A `BAR_FLAG_FIXED` bar is an I/O window the
function decodes at a fixed address outside its BARs, put in the record by the
broker: BARs 0 to 3 of an IDE function whose channel runs in compatibility mode
(0x1F0/8 and 0x3F6/1 for the primary, 0x170/8 and 0x376/1 for the secondary),
and VBoxVGA's DISPI ports (0x1CE/3) in its first free slot. `mk_pio_grant`
grants it like any I/O BAR and answers `EBUSY` while another device's claimant
holds a grant sharing a port; `MkIrqBind` takes IRQ 14 or 15 in INTx mode from
the claimant of an IDE function whose primary or secondary channel runs in
compatibility mode (`src/hardware/broker/legacy/`). `DEVICE_FLAG_*` are defined,
but no code sets them: `flags` is 0 on every record, and no record is built with
`BUS_KIND_VIRT`. `DMA_MAP_*` are the `MkDmaMap` flags, and `BIND_*` the
`MkIrqBind` flags. `DMA_MAP_COHERENT` maps uncached; `DMA_MAP_WC` maps
write-combining where the PAT allows, else uncached. `DMA_MAP_TO_DEVICE` gives a
confined device read-only access to the grant and `DMA_MAP_FROM_DEVICE`
write-only access, or read and write on a VT-d unit without CAP.ZLR; neither
changes anything for a device no IOMMU confines
(`src/hardware/broker/dma/flags.rs:19-38`,
`src/hardware/broker/dma/direction.rs`).

| Constant | Value | File |
|---|---|---|
| `BUS_KIND_PCI` | `0x1` | `src/hardware/broker/device/bus.rs` |
| `BUS_KIND_ACPI` | `0x2` | `src/hardware/broker/device/bus.rs` |
| `BUS_KIND_VIRT` | `0x3` | `src/hardware/broker/device/bus.rs` |
| `BUS_KIND_FIRMWARE` | `0x5` | `src/hardware/broker/device/bus.rs` |
| `BAR_KIND_NONE` | `0x0` | `src/hardware/broker/device/bar.rs` |
| `BAR_KIND_MMIO` | `0x1` | `src/hardware/broker/device/bar.rs` |
| `BAR_KIND_PIO` | `0x2` | `src/hardware/broker/device/bar.rs` |
| `BAR_FLAG_PREFETCH` | `0x1` | `src/hardware/broker/device/flags.rs` |
| `BAR_FLAG_MEM64` | `0x2` | `src/hardware/broker/device/flags.rs` |
| `BAR_FLAG_FIXED` | `0x4` | `src/hardware/broker/device/flags.rs` |
| `DEVICE_FLAG_CLAIMED` | `0x1` | `src/hardware/broker/device/flags.rs` |
| `DEVICE_FLAG_DISABLED` | `0x2` | `src/hardware/broker/device/flags.rs` |
| `DMA_MAP_HIGH` | `0x1` | `src/hardware/broker/dma/flags.rs` |
| `DMA_MAP_DMA32` | `0x2` | `src/hardware/broker/dma/flags.rs` |
| `DMA_MAP_COHERENT` | `0x4` | `src/hardware/broker/dma/flags.rs` |
| `DMA_MAP_WC` | `0x8` | `src/hardware/broker/dma/flags.rs` |
| `DMA_MAP_TO_DEVICE` | `0x10` | `src/hardware/broker/dma/flags.rs` |
| `DMA_MAP_FROM_DEVICE` | `0x20` | `src/hardware/broker/dma/flags.rs` |
| `BIND_MSIX` | `0x1` | `src/hardware/broker/irq/types.rs` |
| `BIND_MSI` | `0x2` | `src/hardware/broker/irq/types.rs` |
| `BROKER_VEC_MIN` | `0x81` | `src/arch/x86_64/interrupt/broker/vectors.rs` |
| `BROKER_VEC_MAX` | `0xc0` | `src/arch/x86_64/interrupt/broker/vectors.rs` |

## Limits

- `abi/driver_broker_abi.md` in the tree is the compact record, class and error
  reference beside `abi/syscalls.toml`. The page beside this one,
  [driver_broker_abi.md](driver_broker_abi.md), is the broker call reference the
  static-check gate greps; both are built from the same handlers as this page.
- The argument lists for the broker calls in `abi/syscalls.toml` do not match
  the handlers for `MkPioGrant`, `MkIrqAck` and `MkIrqPoll`; see [The NONOS
  ABI](abi-stability-in-092.md#stability-in-092).
- The allowlist of PCI registers lives in the broker and is not tabled here.

## See also

- [The NONOS ABI](README.md)
- [The broker API for drivers](../drivers/broker-api.md)
- [Writing a driver](../drivers/writing-a-driver.md)
- [The hardware broker in the kernel](../kernel/hardware-broker.md)
- [IOMMU](../kernel/iommu.md)
