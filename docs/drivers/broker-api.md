# Broker API

The calls a [driver capsule](../overview/glossary.md#driver-capsule) makes to find a device, claim it, map its registers, get DMA memory, take its interrupts and let it go, with what the kernel checks at each step.

## Where the calls live

A driver calls the [hardware broker](../overview/glossary.md#hardware-broker) through `nonos_libc`, the crate in `userland/libc` that every driver depends on (`userland/capsule_driver_virtio_rng/Cargo.toml:22`). The wrappers are in `userland/libc/src/broker/`, re-exported as `mk_device_list`, `mk_mmio_map` and the rest (`userland/libc/src/broker/mod.rs:29-51`). Each makes one system call whose number is four ASCII letters read as a little-endian integer by `tag4`, such as `N_MK_DEVICE_LIST` for `MDLS` (`userland/libc/src/syscall/numbers/broker.rs:18-33`, `src/syscall/abi/tag.rs:20-22`).

`nonos_abi`, the bottom of the native runtime, carries no broker numbers (`userland/nonos_abi/README.md`). The crates in `userland/sdk` are for applications and `userland/platform` is a host-side packaging tool, so a driver uses neither. `nonos_devmodel` runs only on the host, where it gives a proof crate a register window in memory (`userland/nonos_devmodel/README.md`); [writing-a-driver.md](writing-a-driver.md) shows it in use.

On the kernel side every call first passes the contract table, which checks the capability before any handler runs, for example `can_driver` for `MkDeviceClaim` (`src/syscall/contract/cap_table/mk.rs:123-136`). A handler in `src/syscall/microkernel/` then turns the arguments into a request to `src/hardware/broker`. The register-level tables of numbers, arguments and return values are in [../abi/broker.md](../abi/broker.md).

```mermaid
sequenceDiagram
    participant D as driver
    participant K as broker
    D->>K: MkDeviceList
    K-->>D: DeviceRecord array
    D->>K: MkDeviceClaim
    K-->>D: claim epoch
    D->>K: MkPciConfigWrite
    D->>K: MkMmioMap
    D->>K: MkDmaMap
    D->>K: MkIrqBind
    D->>K: MkIrqWait
    D->>K: MkDeviceRelease
```

A typical driver makes the calls in the order above: list, claim, turn on the PCI command bits it needs, map registers, take DMA memory, bind an interrupt, wait on it while serving, and release.

## Capabilities

Each call needs one bit of the driver's [capability word](../overview/glossary.md#capability-word), from `DeviceEnum` to `Pio` (`src/capabilities/types/defs.rs:39-50`). The contract table maps each call to its bit, starting with `MkDeviceList` (`src/syscall/contract/cap_table/mk.rs:123-136`):

| Bit | Name | Calls it admits |
|---|---|---|
| 15 | `DeviceEnum` | `MkDeviceList` |
| 16 | `Driver` | `MkDeviceClaim`, `MkDeviceRelease`, `MkPciConfigRead`, `MkPciConfigWrite` |
| 17 | `Mmio` | `MkMmioMap`, `MkMmioUnmap` |
| 18 | `Irq` | `MkIrqBind`, `MkIrqUnbind`, `MkIrqAck`, `MkIrqPoll`, `MkIrqWait` |
| 19 | `Dma` | `MkDmaMap`, `MkDmaUnmap` |
| 20 | `Pio` | `MkPioGrant`, `MkPioRead`, `MkPioWrite`, `MkPioRelease` |

Every one of these checks also passes for a holder of `Admin`, as `can_driver` and its siblings show (`src/capabilities/token/types/authority_broker.rs:24-55`). A driver also holds `IPC` (bit 3) to serve and `Memory` (bit 4) to allocate (`src/capabilities/types/defs.rs:26-27`). Holding `Irq` lets a capsule post input events too, because `can_input_source` accepts it (`src/capabilities/token/types/authority_broker.rs:59-63`).

## The device record

`MkDeviceList` copies out `DeviceRecord` entries of 176 bytes, a size `DeviceRecord` asserts at compile time (`src/hardware/broker/device/record.rs:21-61`):

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

A `Bar` has kind 1 for memory and 2 for port I/O, and `aux` carries the DesignWare source clock of an ACPI I2C controller, zero otherwise (`src/hardware/broker/device/bar.rs:17-49`). `DEVICE_FLAG_CLAIMED` and `DEVICE_FLAG_DISABLED` are defined but nothing sets them, so the list never shows a device as claimed (`src/hardware/broker/device/flags.rs:19-20`).

`classify_pci` turns the PCI class into the broker class (`src/hardware/broker/class.rs:57-84`):

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
