# hardware, bus and drivers

These three modules are the kernel's device layer, and the division between them is the point. `src/hardware/` is the **broker**: the security mediator through which a userland driver capsule claims a device and receives confined MMIO/DMA/IRQ/PIO grants — no driver touches hardware without going through it. `src/bus/` is the low-level PCI bus underneath (raw config access, enumeration, BAR assignment). `src/drivers/` holds only the handful of drivers the kernel keeps for its own boot; every other driver is a userland capsule.

## bus

`src/bus/mod.rs` is thin — `pub mod pci; pub use pci::*;` — so `bus` today *is* the PCI subsystem: raw config-space I/O, enumeration, BAR/command enabling, resource-window assignment, and (x86) AER error reporting. It is the hardware-access primitive the broker and the boot path build on.

| Item | Where | What it does |
|---|---|---|
| `init` | `src/bus/pci/init.rs:22` | Enumerate the PCI bus. |
| `pci_read32` / `pci_write32` | `src/bus/pci/config.rs:23` / `:37` | Raw config-space access (8/16/32-bit). |
| `enable_bus_master` | `src/bus/pci/enable.rs:19` | Enable bus mastering / memory / I/O space. |
| `find_device_by_id` / `enumerate_devices` | `src/bus/pci/find.rs:27` / `:84` | Find a device, or list them all. |
| `struct PciDevice` | `src/bus/pci/types.rs:30` | A discovered PCI device. |

## drivers

`src/drivers/mod.rs` declares only `pci`, `security` and `virtio_rng`, and re-exports `init_pci` and `init_virtio_rng`. These are the **only** drivers in the kernel — the two the microkernel boot path needs (PCI enumeration and a virtio-rng entropy probe). Every other driver is a userland capsule under `src/hardware/*_capsule/`. The `security` submodule is driver-facing validators (DMA/MMIO/LBA/PCI bounds and rate limiting) used across the trusted paths.

| Item | Where | What it does |
|---|---|---|
| `init_pci` | `src/drivers/pci/manager/global.rs:30` | Initialize the kernel PCI manager. |
| `init` (virtio-rng) | `src/drivers/virtio_rng/init.rs:24` | Bring up the virtio-rng entropy source. |
| `get_random_bytes` | `src/drivers/virtio_rng/api.rs:25` | The entropy-consumer API. |
| `validate_dma_buffer` | `src/drivers/security/dma.rs` (via `mod.rs:36`) | Bounds-check a DMA buffer. |
| `struct RateLimiter` | `src/drivers/security/rate_limiter.rs:58` | Per-driver operation rate limiting. |

## Parts

- [The broker is the gate; hardware](hardware-broker-gate.md)
- [Wiring; See also](hardware-wiring.md)
