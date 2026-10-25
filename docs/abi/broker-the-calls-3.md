# Broker: The calls, continued

Part of [Broker](broker.md).

## The calls, continued

Most failures map the same way across the calls: no claim is `EPERM`, a stale
epoch `ESTALE`, an unknown device `ENODEV`, a bad index, length or range
`EINVAL`, an unknown flag `EOPNOTSUPP`, no memory or address space `ENOMEM`.
`errno_for` in `src/syscall/microkernel/mmio/errno_map.rs` is one example, and
it also maps a window that would expose the device's MSI-X table or pending-bit
array to `EPERM` (`src/syscall/microkernel/mmio/errno_map.rs:22-36`). See
[Errors](errors.md).


## Records

Every record is `repr(C)`, little-endian, and the same in the kernel and in the
libc copy under `userland/libc/src/broker/types/`. `DeviceRecord` asserts its
size at compile time (`src/hardware/broker/device/record.rs:61`).

An `I2C_HID` record reuses fields: `hid_record` puts the 7-bit I2C address in
`vendor`, the HID descriptor register in `device`, the host controller slot in
`pci_progif`, the GPIO community in `irq_pin`, the pin or GSI in `irq_source`,
and the controller and its `HID_INFO_*` bits in bar 0, whose kind is none and
which `bar_count` does not count
(`src/hardware/broker/acpi_i2c/hid_record.rs:31-47`).

### `Bar`, 24 bytes

| Field | Offset | Type | Meaning |
|---|---|---|---|
| `base` | 0 | `u64` | Start of the BAR |
| `size` | 8 | `u64` | Length of the BAR |
| `aux` | 16 | `u32` | For an ACPI LPSS I2C controller, the DesignWare source clock in Hz; in an `I2C_HID` record, the connection speed in Hz; 0 otherwise |
| `kind` | 20 | `u8` | `BAR_KIND_*` |
| `flags` | 21 | `u8` | `BAR_FLAG_*`; in an `I2C_HID` record, the `HID_INFO_*` bits |
| `_pad` | 22 | `[u8; 2]` | Padding |

### `DeviceRecord`, 176 bytes

| Field | Offset | Type | Meaning |
|---|---|---|---|
| `device_id` | 0 | `u64` | The broker's id for the device |
| `bus_kind` | 8 | `u8` | `BUS_KIND_*` |
| `pci_class` | 9 | `u8` | PCI class code |
| `pci_subclass` | 10 | `u8` | PCI subclass |
| `pci_progif` | 11 | `u8` | PCI programming interface |
| `class` | 12 | `u32` | Broker class, below |
| `vendor` | 16 | `u16` | Vendor id |
| `device` | 18 | `u16` | Device id |
| `flags` | 20 | `u32` | Always 0 in 0.9.2 |
| `bar_count` | 24 | `u8` | One past the highest BAR slot in use; a BAR keeps its hardware index and empty slots are zero |
| `irq_line` | 25 | `u8` | Legacy interrupt line, 0xFF for none; the INTx source on x86_64 |
| `irq_pin` | 26 | `u8` | Interrupt pin |
| `_pad1` | 27 | `[u8; 1]` | Padding |
| `irq_source` | 28 | `u32` | Interrupt source; what it names depends on the bus |
| `bars` | 32 | `[Bar; 6]` | Six `Bar` records |

### `MmioMapOut`, 24 bytes

| Field | Offset | Type | Meaning |
|---|---|---|---|
| `user_va` | 0 | `u64` | Where the window is mapped in the caller |
| `length` | 8 | `u64` | Bytes mapped |
| `grant_id` | 16 | `u64` | Pass to `MkMmioUnmap` |

### `IrqBindOut`, 16 bytes

| Field | Offset | Type | Meaning |
|---|---|---|---|
| `grant_id` | 0 | `u64` | First grant; MSI-X vector i is `grant_id + i` |
| `vector` | 8 | `u64` | First vector |
