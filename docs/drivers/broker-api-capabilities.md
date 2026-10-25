# Broker API: Capabilities

Part of [Broker API](broker-api.md).

## Capabilities

Each call needs one bit of the driver's [capability
word](../overview/glossary-boot-root-record.md#capability-word), from `DeviceEnum` to `Pio`
(`src/capabilities/types/defs.rs:37-47`). The contract table maps each call to
its bit, starting with `MkDeviceList`
(`src/syscall/contract/cap_table/mk_device.rs:24-38`):

| Bit | Name | Calls it admits |
|---|---|---|
| 15 | `DeviceEnum` | `MkDeviceList` |
| 16 | `Driver` | `MkDeviceClaim`, `MkDeviceRelease`, `MkPciConfigRead`, `MkPciConfigWrite` |
| 17 | `Mmio` | `MkMmioMap`, `MkMmioUnmap` |
| 18 | `Irq` | `MkIrqBind`, `MkIrqUnbind`, `MkIrqAck`, `MkIrqPoll`, `MkIrqWait` |
| 19 | `Dma` | `MkDmaMap`, `MkDmaUnmap` |
| 20 | `Pio` | `MkPioGrant`, `MkPioRead`, `MkPioWrite`, `MkPioRelease` |

None of these checks passes for a holder of `Admin` alone: `can_driver` and its
siblings ask for their own bit
(`src/capabilities/token/types/authority_broker.rs:29-51`). The two PCI
configuration handlers ask for `Driver` once more themselves
(`sys_pci_config_read` in `src/syscall/microkernel/pci/config.rs:27-34`). A driver also
holds `IPC` (bit 3) to serve and `Memory` (bit 4) to allocate
(`src/capabilities/types/defs.rs:24-25`). Holding `Irq` does not let a capsule
post input events: `can_input_source` takes `InputSource` alone
(`src/capabilities/token/types/authority_broker.rs:59-62`).

## The device record

`MkDeviceList` copies out `DeviceRecord` entries of 176 bytes, a size
`DeviceRecord` asserts at compile time
(`src/hardware/broker/device/record.rs:21-61`):

| Field | Type | Meaning |
|---|---|---|
| `device_id` | u64 | broker id, used by every later call |
| `bus_kind` | u8 | 1 PCI, 2 ACPI, 3 virtual |
| `pci_class`, `pci_subclass`, `pci_progif` | u8 | the raw PCI class code |
| `class` | u32 | the broker class id below |
| `vendor`, `device` | u16 | PCI ids, or PNP-style ids for a platform record |
| `flags` | u32 | `DEVICE_FLAG_CLAIMED`, `DEVICE_FLAG_DISABLED` |
| `bar_count` | u8 | one past the last present BAR |
| `irq_line`, `irq_pin` | u8 | legacy line (0xFF for none) and pin |
| `irq_source` | u32 | the line to pass to `MkIrqBind` for INTx |
| `bars` | 6 `Bar` | 24 bytes each: base, size, aux, kind, flags |

A `Bar` has kind 1 for memory and 2 for port I/O, and `aux` carries the
DesignWare source clock of an ACPI I2C controller, zero otherwise
(`src/hardware/broker/device/bar.rs:17-49`). `DEVICE_FLAG_CLAIMED` and
`DEVICE_FLAG_DISABLED` are defined but nothing sets them, so the list never
shows a device as claimed (`src/hardware/broker/device/flags.rs:19-20`).

`classify_pci` turns the PCI class into the broker class
(`src/hardware/broker/class.rs:57-84`):

| Class id | Name | Source |
|---|---|---|
| 0x0001 | RNG | no PCI rule |
| 0x0010 | BLOCK | PCI class 0x01, and an SD host controller (0x08, 0x05) |
| 0x0020 | NETWORK | class 0x02 |
| 0x0030 | DISPLAY | class 0x03 |
| 0x0040 | INPUT | class 0x09, and the PS/2 records |
| 0x0041 | I2C_HID | ACPI-declared touchpads |
| 0x0050 | AUDIO | class 0x04, subclass 0x01 or 0x03 |
| 0x0060 | SERIAL | class 0x07 subclass 0x00, and ACPI I2C controllers |
| 0x0070 | USB_HOST | class 0x0C subclass 0x03 |
| 0x0071 | USB_HOST_XHCI | the same with prog-if 0x30 |
| 0x0080 | GPIO_CTRL | ACPI GPIO controllers |
| 0xFFFF | OTHER | everything else |
