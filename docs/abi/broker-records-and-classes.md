# Broker: records, continued, and device classes

Part of [Broker](broker.md).

### `IrqPollOut`, 16 bytes

| Field | Offset | Type | Meaning |
|---|---|---|---|
| `seq` | 0 | `u64` | The grant's event counter |
| `overflow` | 8 | `u64` | The slot's overflow counter |

### `DmaMapOut`, 32 bytes

| Field | Offset | Type | Meaning |
|---|---|---|---|
| `user_va` | 0 | `u64` | Where the buffer is mapped in the caller |
| `device_addr` | 8 | `u64` | The address the device uses |
| `length` | 16 | `u64` | Bytes mapped |
| `grant_id` | 24 | `u64` | Pass to `MkDmaUnmap` |

### `PioGrantOut`, 16 bytes

| Field | Offset | Type | Meaning |
|---|---|---|---|
| `port_base` | 0 | `u16` | First port |
| `port_count` | 2 | `u16` | Number of ports |
| `_pad` | 4 | `u32` | Padding |
| `grant_id` | 8 | `u64` | Pass to the port calls |

## Device classes

`classify_pci` sorts a PCI device into a class by its class and subclass codes,
and anything it does not know lands in `OTHER`
(`src/hardware/broker/class.rs:57-84`). Records the kernel builds from ACPI and
the legacy platform take their class directly: the PS/2 keyboard and aux ports
are `INPUT` (`src/hardware/broker/platform.rs:50-65`), and an LPSS I2C
controller is `SERIAL` (`src/hardware/broker/acpi_i2c/record.rs:42-43`).
`list_by_class` matches the class exactly, so asking for `INPUT` does not return
`I2C_HID`, and asking for `USB_HOST` does not return `USB_HOST_XHCI`
(`src/hardware/broker/table/list.rs:29-34`). The DMA page ceiling is the most
4096-byte pages one `MkDmaMap` may ask for on a device of that class, from
`dma_page_limit_for_class` (`src/hardware/broker/dma/limits.rs:31-46`).

| Constant | Value | DMA page ceiling | Devices |
|---|---|---|---|
| `RNG` | `0x0001` | 1 | Defined, but no record carries it in 0.9.2 |
| `BLOCK` | `0x0010` | 1024 | Mass storage, NVMe, SATA and SD host controllers |
| `NETWORK` | `0x0020` | 64 | Network controllers |
| `DISPLAY` | `0x0030` | 8192 | Display controllers |
| `INPUT` | `0x0040` | 1 | PCI input controllers, and the PS/2 keyboard and aux ports |
| `I2C_HID` | `0x0041` | 16 | An ACPI-declared I2C-HID touchpad |
| `AUDIO` | `0x0050` | 16 | Audio, PCI subclass 0x01 or 0x03 |
| `SERIAL` | `0x0060` | 1 | PCI class 0x07 subclass 0x00, and ACPI LPSS I2C controllers |
| `USB_HOST` | `0x0070` | 256 | USB host controllers that are not xHCI |
| `USB_HOST_XHCI` | `0x0071` | 256 | xHCI controllers, prog-if 0x30 |
| `GPIO_CTRL` | `0x0080` | 16 | A platform GPIO community from ACPI |
| `OTHER` | `0xFFFF` | 16 | Anything not classified |
