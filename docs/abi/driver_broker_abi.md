# Driver Broker ABI

The driver broker ABI is the kernel boundary a driver capsule uses to find a
device, claim it, and take the narrow hardware grants it needs to drive it:
MMIO windows, interrupts, DMA buffers, PCI configuration access and, on
x86_64, port I/O. It is not a general physical-memory API and it is not a
kernel driver framework: the kernel validates and records every grant, and the
driver logic lives in the capsule.

Every call below is a microkernel syscall routed by
`src/syscall/microkernel/dispatch/route.rs` to the arm named in the call's
section. Arguments are given in register order, `a0` to `a5` (on x86_64
`rdi, rsi, rdx, r10, r8, r9`, as `abi/syscalls.toml [wire]` publishes them). A
call returns a non-negative value on success and a negative errno on failure.
The `nonos_libc` wrapper of each call is named beside it
(`userland/libc/src/broker/`).

## Authority model

Each call passes two checks. The syscall contract first asks the caller's
capability token for the call's bit (`src/syscall/contract/cap_table/mk.rs`);
a caller without it gets `EPERM` before any handler runs. The handler then
checks device ownership: a call on a device must come from the pid that holds
the device's claim and must name the claim's current epoch, and a call on a
grant must come from the pid that holds the grant.

| Capability | Gate | Calls |
|---|---|---|
| `DeviceEnum` | `can_device_enum` | `MkDeviceList` |
| `Driver` | `can_driver` | `MkDeviceClaim`, `MkDeviceRelease`, `MkPciConfigRead`, `MkPciConfigWrite` |
| `Mmio` | `can_mmio` | `MkMmioMap`, `MkMmioUnmap` |
| `Irq` | `can_irq` | `MkIrqBind`, `MkIrqUnbind`, `MkIrqAck`, `MkIrqPoll`, `MkIrqWait` |
| `Dma` | `can_dma` | `MkDmaMap`, `MkDmaUnmap` |
| `Pio` | `can_pio` | `MkPioGrant`, `MkPioRead`, `MkPioWrite`, `MkPioRelease` |

Each gate also admits `Admin`
(`src/capabilities/token/types/authority_broker.rs`, `authority_admin.rs`).
The two PCI configuration handlers ask for `Driver` itself as well, so an
`Admin` token without `Driver` is refused there with `EPERM`
(`src/syscall/microkernel/pci.rs`).

## Device records

```text
MkDeviceList(class: u32, buf: *mut DeviceRecord, count: u64) -> i64     MDLS 0x534C444D
```

`mk_device_list`. Returns the number of records written. `count == 0` returns
the number of matching devices and writes nothing; otherwise at most `count`
records are copied to `buf`, and a null `buf` is `EFAULT`. `class == 0` lists
every device; any other value lists the devices of that class, and a class no
device has gives an empty list (`src/syscall/microkernel/device.rs`,
`src/hardware/broker/table/list.rs`).

`DeviceRecord` is 176 bytes and `Bar` 24 bytes, both `repr(C)`
(`src/hardware/broker/device/record.rs`, `device/bar.rs`):

```text
DeviceRecord                       Bar
  0  u64 device_id                   0  u64 base
  8  u8  bus_kind                    8  u64 size
  9  u8  pci_class                  16  u32 aux
 10  u8  pci_subclass               20  u8  kind
 11  u8  pci_progif                 21  u8  flags
 12  u32 class                      22  u8  _pad[2]
 16  u16 vendor                     = 24 bytes
 18  u16 device
 20  u32 flags
 24  u8  bar_count
 25  u8  irq_line
 26  u8  irq_pin
 27  u8  _pad1
 28  u32 irq_source
 32  Bar bars[6]
 = 176 bytes
```

- `bus_kind`: 1 PCI, 2 ACPI, 3 virtual (`device/bus.rs`).
- `bar_count` is the span of BAR slots in use, not a compacted count; the
  hardware BAR index is kept and an empty slot is zeroed.
- `Bar.kind`: 0 none, 1 MMIO, 2 port I/O. `Bar.flags`: 1 prefetchable,
  2 64-bit. `Bar.aux` is zero except on an ACPI LPSS I2C controller, where it
  carries the DesignWare source clock in Hz.
- `flags`: `DEVICE_FLAG_CLAIMED` (1) and `DEVICE_FLAG_DISABLED` (2) are
  defined (`device/flags.rs`); no path sets either, so a listed record never
  reports a claim.
- A PCI record carries `irq_line` and `irq_pin` from configuration space and
  `irq_source = irq_line`. An ACPI-bus I2C-HID record gives several fields its
  own meaning (`src/hardware/broker/acpi_i2c/hid_record.rs`).

Class IDs (`src/hardware/broker/class.rs`):

| ID | Class | ID | Class |
|---|---|---|---|
| `0x0001` | RNG | `0x0050` | AUDIO |
| `0x0010` | BLOCK | `0x0060` | SERIAL |
| `0x0020` | NETWORK | `0x0070` | USB_HOST |
| `0x0030` | DISPLAY | `0x0071` | USB_HOST_XHCI (prog-if 0x30) |
| `0x0040` | INPUT | `0x0080` | GPIO_CTRL (ACPI GPIO community) |
| `0x0041` | I2C_HID (ACPI-declared touchpad) | `0xFFFF` | OTHER |

## Claim lifecycle

```text
MkDeviceClaim(device_id: u64) -> epoch                                  MDCL 0x4C43444D
MkDeviceRelease(device_id: u64) -> 0                                    MDRL 0x4C52444D
```

`mk_device_claim`, `mk_device_release`. A claim makes the caller the device's
only holder and returns the claim epoch, a positive counter drawn fresh for
every claim (`src/hardware/broker/claim/`). Every grant call on the device
names that epoch, so a grant cannot be replayed against a device that was
released and claimed again. Claiming also:

- puts the device in the caller's IOMMU domain when a remapping unit in
  service covers it (`src/hardware/broker/confine/attach.rs`). A device no unit
  covers stays on physical addresses and the broker logs it; a unit that will
  not take the device refuses the claim with `EPERM`;
- brings the device to power state D0 (`src/hardware/broker/power.rs`);
- makes sure none of its requests are no-snoop
  (`src/hardware/broker/claim/no_snoop.rs`).

`MkDeviceRelease` first turns bus mastering off, then drops the holder's MMIO,
IRQ, DMA and port I/O grants on the device, then removes the claim and leaves
the IOMMU domain (`src/syscall/microkernel/device.rs`). Process exit does the
same for every claim the capsule held: its MMIO grants and claims go first,
with bus mastering turned off, then its IRQ, DMA and port I/O grants
(`src/process/exit/finalize.rs`).

## MMIO grants

```text
MkMmioMap(device_id: u64, claim_epoch: u64, bar_and_flags: u64,
          offset: u64, length: u64, out: *mut MmioMapOut) -> 0          MMMP 0x504D4D4D
MkMmioUnmap(grant_id: u64) -> 0                                         MMUM 0x4D554D4D

MmioMapOut { u64 user_va; u64 length; u64 grant_id; }   // 24 bytes
```

`mk_mmio_map(device_id, claim_epoch, bar_index, flags, offset, length, out)`,
`mk_mmio_unmap`. The call has seven inputs and six registers, so `a2` packs
`(bar_index << 32) | flags` (`src/syscall/microkernel/dispatch/unpack.rs`).
`flags` must be 0. The BAR must be an MMIO BAR of the claimed device and the
request must lie inside it (`src/hardware/broker/mmio/map.rs`).

Offset and length need not be page aligned, and the BAR need not start on a
page: a 2 KiB AHCI ABAR at a base ending in 0x800 is mapped by its whole page.
`MmioMapOut.user_va` is the VA of the requested first byte (it carries the same
offset inside its page as the physical address), and `length` is the bytes
usable from there. No mapped page may hold any device's MSI-X table or PBA: the
mapping is cut short at the page below the first one it would reach, and
refused with `EPERM` when its first page holds one
(`src/hardware/broker/mmio/window.rs`). Pages are mapped user, read-write,
uncached and no-execute. If `out` cannot be written the grant is rolled back
and the call returns `EFAULT`.

## IRQ grants

```text
MkIrqBind(device_id: u64, claim_epoch: u64, irq_source: u32, flags: u32,
          vector_count: u32, out: *mut IrqBindOut) -> 0                 MIRB 0x4252494D
MkIrqPoll(grant_id: u64, out: *mut IrqPollOut) -> 0                     MIRP 0x5052494D
MkIrqAck(grant_id: u64) -> 0                                            MIRA 0x4152494D
MkIrqWait(grant_id: u64, last_seq: u64, timeout_ms: u64,
          out_seq: *mut u64) -> 0 | 1                                   MIRW 0x5752494D
MkIrqUnbind(grant_id: u64) -> 0                                         MIRU 0x5552494D

IrqBindOut { u64 grant_id; u64 vector; }     // 16 bytes
IrqPollOut { u64 seq;      u64 overflow; }   // 16 bytes
```

`mk_irq_bind`, `mk_irq_poll`, `mk_irq_ack`, `mk_irq_wait`, `mk_irq_unbind`
(`src/syscall/microkernel/irq/`). `flags` picks the delivery path, one at a
time (`src/hardware/broker/irq/types.rs`):

| Mode | `flags` | `irq_source` | `vector_count` |
|---|---|---|---|
| INTx | 0 | the record's `irq_line` | 0 |
| MSI | `BIND_MSI` (2) | 0 | 1 |
| MSI-X | `BIND_MSIX` (1) | 0 | 1 up to the broker's vector count, and no more than the device's MSI-X table |

An INTx bind resolves the line through the MADT interrupt overrides to a GSI
and routes it to a broker vector with the line masked. Only `MkIrqAck` unmasks
it (`src/hardware/broker/irq/release/ack.rs`), so a driver acknowledges once
after binding to open the line, and the line is masked again at each delivery
until the next acknowledgement. An MSI-X bind of N vectors allocates a
contiguous run: the grant ids are `grant_id + i` and the vectors `vector + i`
for `i` in `0..N`. MSI and MSI-X have no line mask, so `MkIrqAck` on them
changes nothing on the hardware.

`MkIrqPoll` copies the grant's delivery counter (`seq`) and overflow counter.
`MkIrqWait` blocks until the counter moves past `last_seq`, the timeout lapses
(0 means 100 ms), or another wake reaches the process, and writes the current
sequence to `out_seq` for the next call. `grant_id == 0` waits on every grant
the caller holds. It returns 0, or `IRQ_WAIT_TIMED_OUT` (1) when it slept the
whole timeout and the sequence did not move
(`src/syscall/microkernel/irq/wait.rs`, `irq/timeout.rs`). A return of 0 can be
a spurious wake; the caller compares the sequence and waits again.

## DMA grants

```text
MkDmaMap(device_id: u64, claim_epoch: u64, length: u64, flags: u32,
         out: *mut DmaMapOut) -> 0                                      MDMM 0x4D4D444D
MkDmaUnmap(grant_id: u64) -> 0                                          MDMU 0x554D444D

DmaMapOut { u64 user_va; u64 device_addr; u64 length; u64 grant_id; }   // 32 bytes
```

`mk_dma_map`, `mk_dma_unmap` (`src/syscall/microkernel/dma.rs`). The broker
does not pin a caller buffer: it allocates `length` bytes of physically
contiguous frames, zeroes them, maps them into the caller and returns where the
driver sees them (`user_va`) and where the device does (`device_addr`): an IOVA
in the capsule's domain when the device is confined, the physical address when
it is not (`src/hardware/broker/dma/map/`, `confine/map.rs`).

- `length` is a non-zero multiple of 4096 and at most the device class's
  ceiling (`src/hardware/broker/dma/limits.rs`): RNG, INPUT and SERIAL 1 page;
  AUDIO 16; NETWORK 64; USB_HOST and USB_HOST_XHCI 256; BLOCK 1024; DISPLAY
  8192; any other class 16.
- `flags` (`src/hardware/broker/dma/flags.rs`): `DMA_MAP_HIGH` (1) takes frames
  from the display pool or high memory; `DMA_MAP_DMA32` (2) keeps the frames
  below 4 GiB (`ENOMEM` when there are none) and the device address too
  (`ERANGE` when it would not fit); `DMA_MAP_COHERENT` (4) maps uncached;
  `DMA_MAP_WC` (8) maps write-combining. `DMA_MAP_HIGH` with `DMA_MAP_DMA32`, or
  `DMA_MAP_COHERENT` with `DMA_MAP_WC`, is `ENOTSUP`, as is any unknown bit.

`MkDmaUnmap` takes the grant from the device first, then scrubs the frames and
frees them; frames the device's domain will not give up are kept out of use
(`src/hardware/broker/dma/teardown.rs`).

## PCI configuration

```text
MkPciConfigRead(device_id: u64, claim_epoch: u64, offset: u32,
                width: u32) -> value                                    MPCR 0x5243504D
MkPciConfigWrite(device_id: u64, claim_epoch: u64, offset: u32,
                 value: u16) -> 0                                       MPCW 0x5743504D
```

`mk_pci_config_read`, `mk_pci_config_write`. A read is 1, 2 or 4 bytes, aligned
to its width, inside the first 256 bytes, and returns the value
(`src/hardware/broker/pci/read.rs`). A write is one 16-bit register and may
change only the Command register's Memory Space (bit 1), Bus Master (bit 2) and
Interrupt Disable (bit 10) bits, the MSI-X Message Control register's Function
Mask and Enable bits, and the vendor bits `pci/quirk_bits.rs` names for audio
and network functions (`src/hardware/broker/pci/allowlist.rs`). Anything else
is `EINVAL`. A value too wide to be a 16-bit register is refused by the
dispatch arm with `EINVAL` rather than written with its top half dropped.

## Port I/O grants

```text
MkPioGrant(device_id: u64, claim_epoch: u64, bar_index: u8, flags: u32,
           out: *mut PioGrantOut) -> 0                                  MPGT 0x5447504D
MkPioRead(grant_id: u64, port_offset: u16, width: u8,
          out: *mut u32) -> 0                                           MPRD 0x4452504D
MkPioWrite(grant_id: u64, port_offset: u16, width: u8, value: u32) -> 0 MPWR 0x5257504D
MkPioRelease(grant_id: u64) -> 0                                        MPRL 0x4C52504D

PioGrantOut { u16 port_base; u16 port_count; u32 _pad; u64 grant_id; }  // 16 bytes
```

`mk_pio_grant`, `mk_pio_read`, `mk_pio_write`, `mk_pio_release`
(`src/syscall/microkernel/pio/`). Port I/O exists only on x86_64; on other
architectures every one of these returns `ENOSYS`. A grant covers one port I/O
BAR of the claimed device, whole; `flags` must be 0, and a BAR that is not a
port I/O BAR is `ENOTSUP`. Reads and writes name an offset inside the grant and
a width of 1, 2 or 4 bytes, and the kernel runs the `in` or `out` instruction
itself. Each access checks again that the caller still holds the claim at the
grant's epoch (`src/hardware/broker/pio/`).

## Errors

| Errno | Value | Returned when |
|---|---|---|
| `EPERM` | -1 | the capability gate refused the call; the caller does not hold the claim or the grant; an MMIO request would map an MSI-X table or PBA; the IOMMU would not take the device at claim time |
| `ENOMEM` | -12 | no free broker vector, no user VA left, no frames for a DMA map, or the page tables could not be written |
| `EFAULT` | -14 | an output pointer is null or not writable, or the result could not be copied out (the grant is then rolled back) |
| `EBUSY` | -16 | `MkDeviceClaim` on a claimed device; `MkIrqBind` on a line already bound |
| `ENODEV` | -19 | an unknown device id; `MkDeviceRelease` of a device nobody holds; an interrupt or configuration access the platform could not carry out |
| `EINVAL` | -22 | a bad BAR index, range, length, alignment, width, offset or argument too wide for its field; an unknown grant id; a configuration write outside the allowlist |
| `ERANGE` | -34 | a `DMA_MAP_DMA32` map whose device address would not fit below 4 GiB |
| `ENOSYS` | -38 | a port I/O call on an architecture without port I/O |
| `ENOTSUP` | -95 | an unknown flag bit, or two flags that exclude each other; `MkPioGrant` on a BAR that is not a port I/O BAR |
| `ESTALE` | -116 | the epoch named is not the claim's current one |

The mappings are in `src/syscall/microkernel/{device.rs, mmio/errno_map.rs,
irq/errno_map.rs, dma.rs, pci.rs, pio/errno.rs}`.

## Security invariants

- No broker call grants access without both the capability and current device
  ownership.
- Grants are scoped to one owner process and one claim epoch, and are dropped
  when the claim is released or the process exits.
- MMIO grants stay inside the claimed device's BARs and never map an MSI-X
  table or PBA.
- IRQ grants are polled, acknowledged, waited on and released by grant id, by
  their holder only.
- DMA grants are zeroed kernel allocations, bounded by device class, and are
  taken from the device before their frames are freed.
- PCI configuration writes are limited to an allowlist of bits.
- Port I/O grants are x86_64-only and range checked, with the claim re-checked,
  on every access.

## Non-goals

The broker ABI does not parse device protocols, implement NIC, storage or GPU
drivers in the kernel, persist hardware state, or allow arbitrary physical
memory access. Protocol state belongs in driver capsules and higher service
capsules.

## Debugging a driver against the broker

The claim is where most bring-up problems surface, because every grant call
checks it. `EBUSY` on `MkDeviceClaim` means the device is already claimed:
either two drivers were spawned for the same hardware, or a previous instance
still holds it, which is a spawn-plan question rather than a hardware one.
`ENODEV` means the device id is not in the broker table at all, a discovery
problem one layer down; list with `MkDeviceList` and class 0 to see what the
broker enumerated. `EPERM` on a claim that is not a capability refusal means
the IOMMU would not take the device.

Once claimed, `EPERM` on a grant call has two causes that look identical from
the return value: the capsule's manifest did not declare the capability the
call needs (`Mmio` for a map, `Irq` for a bind or poll, `Dma` for a DMA map),
or the call named a claim or grant the caller does not hold. `ESTALE` means the
epoch is from an earlier claim of the device. Interrupts that bind cleanly but
never arrive, or arrive once and then stop, are usually an INTx line still
masked: the line is masked after the bind and after each delivery until
`MkIrqAck`, so a driver that polls or waits but never acknowledges sees nothing,
or sees the sequence advance once and then stall. A driver can block on
interrupts with `MkIrqWait` instead of polling; it should treat a 0 return
without a new sequence as a spurious wake and wait again.

## Source map

```text
src/syscall/microkernel/dispatch/{device,mmio,irq,dma,pio,debug,unpack}.rs
                                   the dispatch arms: register to argument
src/syscall/microkernel/device.rs  MkDeviceList, MkDeviceClaim, MkDeviceRelease
src/syscall/microkernel/mmio/      MkMmioMap, MkMmioUnmap, MmioMapOut
src/syscall/microkernel/irq/       MkIrqBind/Poll/Ack/Wait/Unbind, IrqBindOut, IrqPollOut
src/syscall/microkernel/dma.rs     MkDmaMap, MkDmaUnmap, DmaMapOut
src/syscall/microkernel/pci.rs     MkPciConfigRead, MkPciConfigWrite
src/syscall/microkernel/pio/       MkPioGrant/Read/Write/Release (x86_64), PioGrantOut
src/hardware/broker/claim/         the claim table, the epoch, release and exit cleanup
src/hardware/broker/confine/       the per-capsule IOMMU domain
src/hardware/broker/device/        DeviceRecord, Bar, bus kinds and flags
src/hardware/broker/class.rs       class IDs
src/hardware/broker/mmio/          BAR validation, the MSI-X/PBA exclusion, MMIO grants
src/hardware/broker/irq/           INTx, MSI and MSI-X binding, delivery counters, waits
src/hardware/broker/dma/           allocation, class ceilings, flags, teardown
src/hardware/broker/pci/           the configuration read rules and write allowlist
src/hardware/broker/pio/           port I/O grants and the checked accesses
userland/libc/src/broker/          the mk_* wrappers and the record types
```

## Related

- [abi/driver_broker_abi.md](../../abi/driver_broker_abi.md): the compact
  record, class and error reference beside `abi/syscalls.toml`.
- [Broker](broker.md): the fuller narrative of the same surface, with the
  constant tables.
- [Syscalls](syscalls.md) and [Capabilities](capabilities.md): every call's tag
  and gate.
- [The broker API for drivers](../drivers/broker-api.md) and
  [Writing a driver](../drivers/writing-a-driver.md).
- [IOMMU](../kernel/iommu.md): the domains a claim attaches to.
