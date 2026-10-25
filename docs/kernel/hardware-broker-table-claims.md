# The hardware broker: the device table and claims

Part of [The hardware broker](hardware-broker.md).

## The device table

At boot `seed_hardware_broker` fills the table from the PCI scan, then adds the legacy platform devices and, on x86_64, the I2C and GPIO controllers the ACPI tables describe (`src/kernel_core/init/platform/hardware_broker.rs:19-40`). [PCI and ACPI](pci-and-acpi.md) describes the scan.

Each entry is a `DeviceRecord` of 176 bytes: the device id, the bus kind, the PCI class, subclass and programming interface, the broker class, vendor and device ids, flags, the interrupt line, pin and source, a BAR count and up to six BARs (`src/hardware/broker/device/record.rs:19-61`). A PCI device's id is its index in the scan, as `init_from_pci` numbers it (`src/hardware/broker/table/init.rs:28-43`); a platform device is numbered after the last one by `register_platform_device` (`src/hardware/broker/table/init.rs:45-51`).

| Class | Id | PCI class:subclass matched, or platform source |
|---|---|---|
| `RNG` | `0x0001` | none; no entry gets this class at this commit |
| `BLOCK` | `0x0010` | 01 any, and 08:05 SD host |
| `NETWORK` | `0x0020` | 02 any |
| `DISPLAY` | `0x0030` | 03 any |
| `INPUT` | `0x0040` | 09 any, and the PS/2 keyboard and aux port records |
| `I2C_HID` | `0x0041` | none, from ACPI |
| `AUDIO` | `0x0050` | 04:01 and 04:03 |
| `SERIAL` | `0x0060` | 07:00, and the I2C controllers from ACPI |
| `USB_HOST` | `0x0070` | 0C:03 other than xHCI |
| `USB_HOST_XHCI` | `0x0071` | 0C:03 with programming interface 0x30 |
| `GPIO_CTRL` | `0x0080` | none, from ACPI |
| `OTHER` | `0xFFFF` | everything else |

The ids are in `ids` (`src/hardware/broker/class.rs:21-44`) and the PCI mapping in `classify_pci` (`src/hardware/broker/class.rs:57-84`). The PS/2 records come from `register_legacy`, which adds them on every machine and leaves the driver to find out whether an i8042 answers (`src/hardware/broker/platform.rs:31-40`).

`MkDeviceList(class, buf, count)` needs `DeviceEnum`. With a count of 0 `sys_device_list` returns how many devices of that class exist; otherwise it copies up to `count` records (`src/syscall/microkernel/device_list.rs`). A window's base is a physical address, so a record the caller does not hold reads zero in the `base` of every bar that is not kind None; size, kind, aux and flags stay (`table/list.rs` `list_shown`, rule in `broker/shown.rs`). A kind-None bar carries a name, never an address: a GPIO controller's `_HID` bytes, or an I2C-HID record's host controller as its device_id (u64::MAX when unresolved, `acpi_i2c/host_id.rs`). Tested in kernel_proofs::device_shown.

## Claims

`MkDeviceClaim(device_id)` needs `Driver`. A device has one holder at a time; `claim` refuses a second with `EBUSY` and returns a fresh [claim epoch](../overview/glossary-claim-epoch.md#claim-epoch) to the first (`src/hardware/broker/claim/claim.rs:23-45`). In the same call the broker moves the device into the capsule's [IOMMU domain](../overview/glossary-hardware-broker.md#iommu-domain) when a remapping unit in service covers it, powers it to D0 with `power_on_device`, turns off no-snoop requests, and turns Bus Master off if firmware left it on (`master_off.rs`; VMMDev, which reads guest memory past any IOMMU, excepted), so the device masters nothing until its driver turns Bus Master on through `MkPciConfigWrite` once its own rings exist (`src/hardware/broker/claim/claim.rs`). The boot PCI pass turns on only the decoders and leaves Bus Master as firmware left it (`src/bus/pci/assign/command_rule.rs`). When an IOMMU is in service and will not take the device, the claim fails with `ClaimError::Unconfined`, which the caller sees as `EPERM` (`src/syscall/microkernel/device.rs:75-81`); [IOMMU](iommu.md) explains when that happens.

Epochs come from one counter that starts at 1, in `next_epoch` (`src/hardware/broker/claim/state.rs:24-29`). Every MMIO, DMA, IRQ and PIO request carries the epoch, and one that does not match the live claim is refused as `StaleEpoch`, which the caller sees as `ESTALE`, -116, as `validate` does for DMA (`src/hardware/broker/dma/map/validate.rs:46-52`). A grant from an earlier claim cannot be reused after a release and a new claim.

