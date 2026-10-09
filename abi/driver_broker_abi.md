# Driver broker ABI

The compact reference for the broker syscalls a driver capsule calls: the
calls and their registers, the device record, the class IDs and the errors.
Every call here is routed to a handler by
`src/syscall/microkernel/dispatch/route.rs`; none returns `-ENOSYS` except the
port I/O calls on an architecture without port I/O. The full description
(what each call checks, the grant rules, the DMA flags, the PCI write
allowlist, debugging) is
[docs/abi/driver_broker_abi.md](../docs/abi/driver_broker_abi.md). The tags
and capability gates are published in [syscalls.toml](syscalls.toml).

## 1. Calls

Arguments in register order, `a0` upward. A call returns a non-negative value
on success and a negative errno on failure.

```
MkDeviceList(class: u32, buf: *mut DeviceRecord, count: u64) -> count written
MkDeviceClaim(device_id: u64) -> claim epoch
MkDeviceRelease(device_id: u64) -> 0

MkMmioMap(device_id: u64, claim_epoch: u64, bar_and_flags: u64,
          offset: u64, length: u64, out: *mut MmioMapOut) -> 0
MkMmioUnmap(grant_id: u64) -> 0

MkIrqBind(device_id: u64, claim_epoch: u64, irq_source: u32, flags: u32,
          vector_count: u32, out: *mut IrqBindOut) -> 0
MkIrqUnbind(grant_id: u64) -> 0
MkIrqAck(grant_id: u64) -> 0
MkIrqPoll(grant_id: u64, out: *mut IrqPollOut) -> 0
MkIrqWait(grant_id: u64, last_seq: u64, timeout_ms: u64, out_seq: *mut u64) -> 0 or 1

MkDmaMap(device_id: u64, claim_epoch: u64, length: u64, flags: u32,
         out: *mut DmaMapOut) -> 0
MkDmaUnmap(grant_id: u64) -> 0

MkPciConfigRead(device_id: u64, claim_epoch: u64, offset: u32, width: u32) -> value
MkPciConfigWrite(device_id: u64, claim_epoch: u64, offset: u32, value: u16) -> 0

MkPioGrant(device_id: u64, claim_epoch: u64, bar_index: u8, flags: u32,
           out: *mut PioGrantOut) -> 0
MkPioRead(grant_id: u64, port_offset: u16, width: u8, out: *mut u32) -> 0
MkPioWrite(grant_id: u64, port_offset: u16, width: u8, value: u32) -> 0
MkPioRelease(grant_id: u64) -> 0
```

`MkDeviceList` with `count == 0` returns the number of matching devices and
writes nothing; `class == 0` lists every device, and a class no device has
gives an empty list. `MkMmioMap` packs `bar_and_flags = (bar_index << 32) |
flags` (`src/syscall/microkernel/dispatch/unpack.rs`). `MkIrqWait` returns 1
when it slept its whole timeout with the sequence unmoved.

Output records, `repr(C)`, little-endian:

```
MmioMapOut  { u64 user_va; u64 length; u64 grant_id; }                    24 bytes
IrqBindOut  { u64 grant_id; u64 vector; }                                 16 bytes
IrqPollOut  { u64 seq; u64 overflow; }                                    16 bytes
DmaMapOut   { u64 user_va; u64 device_addr; u64 length; u64 grant_id; }   32 bytes
PioGrantOut { u16 port_base; u16 port_count; u32 _pad; u64 grant_id; }    16 bytes
```

## 2. Device record

`DeviceRecord` is 176 bytes and `Bar` 24; both sizes are asserted at compile
time in the kernel (`src/hardware/broker/device/record.rs`, `device/bar.rs`)
and in its libc mirror (`userland/libc/src/broker/types/`).

```
DeviceRecord {
    device_id:    u64       // broker-assigned
    bus_kind:     u8        // BUS_PCI=1, BUS_ACPI=2, BUS_VIRT=3
    pci_class:    u8        // PCI class code, as read
    pci_subclass: u8
    pci_progif:   u8
    class:        u32       // class id, see section 4
    vendor:       u16       // vendor id (PCI)
    device:       u16       // device id (PCI)
    flags:        u32       // CLAIMED=1, DISABLED=2 (defined; never set)
    bar_count:    u8        // valid BAR slot span, not compacted count
    irq_line:     u8        // PCI interrupt line (0xFF none)
    irq_pin:      u8        // PCI interrupt pin (0 none)
    _pad1:        u8
    irq_source:   u32       // PCI: the interrupt line
    bars:         [Bar; 6]  // hardware BAR indices preserved; holes are zeroed
}

Bar {
    base:   u64
    size:   u64
    aux:    u32      // LPSS I2C controller: source clock in Hz; else 0
    kind:   u8       // BAR_NONE=0, BAR_MMIO=1, BAR_PIO=2
    flags:  u8       // PREFETCH=1, MEM64=2
    _pad:   [u8; 2]
}
```

An ACPI-bus I2C-HID record gives `vendor`, `device`, `pci_progif`, `irq_pin`,
`irq_source` and bar 0 meanings of its own; they are listed in
`src/hardware/broker/acpi_i2c/hid_record.rs`.

## 3. Errors

| Code | Meaning |
|---|---|
| `-EPERM` | caller lacks the capability, or does not hold the claim or grant |
| `-ENOMEM` | no broker vector, user VA or DMA frames left |
| `-EFAULT` | user pointer null, not mapped or not writable |
| `-EBUSY` | device already claimed, or interrupt line already bound |
| `-ENODEV` | `device_id` not in the broker table, or released while not claimed |
| `-EINVAL` | argument out of range, misaligned or too wide; unknown grant id |
| `-ERANGE` | a `DMA32` map whose device address would not fit below 4 GiB |
| `-ENOSYS` | a port I/O call on an architecture without port I/O |
| `-ENOTSUP` | an unknown flag bit, or flags that exclude each other |
| `-ESTALE` | claim epoch advanced; the device was released and re-claimed |

## 4. Class IDs

```
CLASS_RNG            = 0x0001
CLASS_BLOCK          = 0x0010
CLASS_NETWORK        = 0x0020
CLASS_DISPLAY        = 0x0030
CLASS_INPUT          = 0x0040
CLASS_I2C_HID        = 0x0041    // ACPI-declared I2C-HID touchpad
CLASS_AUDIO          = 0x0050
CLASS_SERIAL         = 0x0060
CLASS_USB_HOST       = 0x0070
CLASS_USB_HOST_XHCI  = 0x0071    // USB host with xHCI prog-if 0x30
CLASS_GPIO_CTRL      = 0x0080    // ACPI GPIO community controller
CLASS_OTHER          = 0xFFFF
```

The broker classifies devices using PCI class codes on PCI buses
(`classify_pci`, `src/hardware/broker/class.rs`) and fixed entries for ACPI
devices. A device whose class is not recognised reports `CLASS_OTHER`.

## 5. Layout changes

The kernel and the libc define these records separately. The compile-time size
assertions on both sides fail the build when one of them changes size alone.
