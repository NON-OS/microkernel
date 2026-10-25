# Syscalls: Device broker

Part of [Syscalls](syscalls.md).

### Device broker

These are the calls of the device broker. `MkDeviceClaim` returns a [claim
epoch](../overview/glossary-claim-epoch.md#claim-epoch) that the later calls on the device
pass back. See [Broker](broker.md) for the arguments and the records.

| Tag | Number | Name | Capability | Gate | Meaning |
|---|---|---|---|---|---|
| `MDLS` | `0x534C444D` | `MkDeviceList` | Admin or DeviceEnum | `can_device_enum` `src/syscall/contract/cap_table/mk_device.rs:24` | List broker devices, all or one class. |
| `MDCL` | `0x4C43444D` | `MkDeviceClaim` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_device.rs:25` | Claim a device; returns the claim epoch. |
| `MDRL` | `0x4C52444D` | `MkDeviceRelease` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_device.rs:25` | Release a claimed device and every grant on it. |
| `MBFB` | `0x4246424D` | `MkBootFramebuffer` | Mmio | `can_mmio` `src/syscall/contract/cap_table/mk_platform.rs:70` | The boot framebuffer's geometry, for the holder of its claim. |
| `MMMP` | `0x504D4D4D` | `MkMmioMap` | Mmio | `can_mmio` `src/syscall/contract/cap_table/mk_device.rs:26` | Map part of a claimed device's MMIO BAR; `flags` bit 0 (`MK_MMIO_MAP_WC`) asks write-combining, given only on a prefetchable BAR (`src/hardware/broker/mmio/cache.rs`), and the kernel's own type where it maps the range (`src/hardware/broker/mmio/memory_type.rs`). |
| `MMUM` | `0x4D554D4D` | `MkMmioUnmap` | Mmio | `can_mmio` `src/syscall/contract/cap_table/mk_device.rs:26` | Unmap an MMIO grant. |
| `MIRB` | `0x4252494D` | `MkIrqBind` | Irq | `can_irq` `src/syscall/contract/cap_table/mk_device.rs:30` | Bind a claimed device's interrupt: INTx, MSI or MSI-X. |
| `MIRU` | `0x5552494D` | `MkIrqUnbind` | Irq | `can_irq` `src/syscall/contract/cap_table/mk_device.rs:30` | Unbind an interrupt grant. |
| `MIRA` | `0x4152494D` | `MkIrqAck` | Irq | `can_irq` `src/syscall/contract/cap_table/mk_device.rs:30` | Acknowledge an interrupt on a grant. |
| `MIRP` | `0x5052494D` | `MkIrqPoll` | Irq | `can_irq` `src/syscall/contract/cap_table/mk_device.rs:30` | Read a grant's interrupt sequence without waiting. |
| `MIRW` | `0x5752494D` | `MkIrqWait` | Irq | `can_irq` `src/syscall/contract/cap_table/mk_device.rs:31` | Wait for an interrupt on one grant or on all of the caller's. |
| `MDMM` | `0x4D4D444D` | `MkDmaMap` | Dma | `can_dma` `src/syscall/contract/cap_table/mk_device.rs:32` | Get a DMA buffer for a claimed device. |
| `MDMU` | `0x554D444D` | `MkDmaUnmap` | Dma | `can_dma` `src/syscall/contract/cap_table/mk_device.rs:32` | Free a DMA grant. |
| `MPCR` | `0x5243504D` | `MkPciConfigRead` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_device.rs:33` | Read an allowed PCI configuration register of a claimed device. |
| `MPCW` | `0x5743504D` | `MkPciConfigWrite` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_device.rs:33` | Write an allowed 16-bit PCI configuration register. |
| `MPGT` | `0x5447504D` | `MkPioGrant` | Pio | `can_pio` `src/syscall/contract/cap_table/mk_device.rs:38` | Get a grant over a claimed device's port BAR; x86_64 only. |
| `MPRD` | `0x4452504D` | `MkPioRead` | Pio | `can_pio` `src/syscall/contract/cap_table/mk_device.rs:38` | Read 1, 2 or 4 bytes from a granted port. |
| `MPWR` | `0x5257504D` | `MkPioWrite` | Pio | `can_pio` `src/syscall/contract/cap_table/mk_device.rs:38` | Write 1, 2 or 4 bytes to a granted port. |
| `MPRL` | `0x4C52504D` | `MkPioRelease` | Pio | `can_pio` `src/syscall/contract/cap_table/mk_device.rs:38` | Drop a port grant. |
| `MHVB` | `0x4256484D` | `MkHvBackdoor` | Pio | `can_pio` `src/syscall/contract/cap_table/mk_device.rs:38` | One VMware backdoor call from the absolute pointer's set, where the host answers it; -19 elsewhere. |
| `MHVP` | `0x5056484D` | `MkHvPost` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_platform.rs:68` | Post a VMBus message: the bus's on the root, a channel's own on its record. |
| `MHVR` | `0x5256484D` | `MkHvRecv` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_platform.rs:68` | Take the oldest VMBus message: the core's on the root, a channel's answers on its record. |
| `MHVE` | `0x4556484D` | `MkHvEvents` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_platform.rs:68` | Take and clear VMBus's channel event flags: the core's on the root, a channel's on its record. |
| `MHVS` | `0x5356484D` | `MkHvSignal` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_platform.rs:68` | Signal the host on a channel's connection: any on the root, the channel's own on its record. |
| `MHVC` | `0x4356484D` | `MkHvChannel` | Driver | `can_driver` `src/syscall/contract/cap_table/mk_platform.rs:68` | Give a channel the host offered a record of its own, for its driver; the VMBus root's holder only, never a host service. |

### Graphics and input

| Tag | Number | Name | Capability | Gate | Meaning |
|---|---|---|---|---|---|
| `MSRG` | `0x4752534D` | `MkSurfaceRegister` | GraphicsSurfaceCreate | `can_surface_create` `src/syscall/contract/cap_table/mk_trust.rs:41` | Register the caller's own memory as a surface; returns its id. A range in a broker window is refused unless it lies wholly inside one DMA grant the caller's process holds (a GPU driver's scanout); such a surface's frames stay the grant's, and when the grant goes they wait for the last process mapping them before the broker frees them (`src/kernel_core/surface_registry/pin/grant_run.rs`). |
| `MSSH` | `0x4853534D` | `MkSurfaceShare` | GraphicsSurfaceCreate | `can_surface_create` `src/syscall/contract/cap_table/mk_trust.rs:41` | Share a surface the caller owns with `receiver_pid`, a live process other than the caller's, and return its handle; `flags` bit 0 maps that receiver's view writable, any other bit is `EINVAL`, a fifth receiver of one surface `ENOMEM`. Naming the caller's own process names nobody and only returns the handle (`do_share` in `src/syscall/dispatch/router/surface_handlers/share.rs`, `receiver` in `src/syscall/dispatch/router/surface_handlers/receiver.rs`). |
| `MSPR` | `0x5250534D` | `MkSurfacePresent` | GraphicsPresent | `can_present` `src/syscall/contract/cap_table/mk_trust.rs:43` | Present a surface, whole or one damage rectangle. |
| `MDVW` | `0x5756444D` | `MkDisplayVsyncWait` | GraphicsDisplayQuery | `can_display_query` `src/syscall/contract/cap_table/mk_trust.rs:44` | Sleep to the next tick of the kernel's 60 Hz frame clock; not the display's own sync. |
| `MIEP` | `0x5045494D` | `MkInputEventPost` | InputSource | `can_input_source` `src/syscall/contract/cap_table/mk_surface.rs:36` | Post one input event to the kernel's input ring. |
| `MIED` | `0x4445494D` | `MkInputEventDrain` | InputConsumer | `can_input_consumer` `src/syscall/contract/cap_table/mk_surface.rs:37` | Drain input events from the ring. |
| `MIEW` | `0x5745494D` | `MkInputEventWait` | InputConsumer | `can_input_consumer` `src/syscall/contract/cap_table/mk_surface.rs:38` | Wait for new input events. |
| `GDIM` | `0x4D494447` | `GraphicsDisplayDimensions` | GraphicsDisplayQuery | `Capability::GraphicsDisplayQuery` `src/syscall/contract/cap_table/graphics.rs:29` | The display's width and height, and its physical size when the firmware gives it. |

Both display calls know display 0 only. `wait_for_vsync` sleeps to the next
multiple of a period taken from `TARGET_HZ`, 60 by default, and reads no
hardware sync (`src/kernel_core/surface_registry/vsync.rs:19-58`).
`handle_display_dimensions` refuses any other display with `EINVAL`
(`src/syscall/dispatch/router/graphics_backend.rs:47-50`).
