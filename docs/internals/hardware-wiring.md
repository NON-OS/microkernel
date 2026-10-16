# hardware, bus and drivers: Wiring; See also

Part of [hardware, bus and drivers](hardware.md).

## Wiring

- **broker** is called by the [syscall](syscall.md) `microkernel/` handlers (dma, device, pci, pio, mmap/munmap), and calls [memory](memory.md) (`IommuDomain` for confinement), [bus](hardware.md#bus) (consumes `PciDevice` for its table), [capabilities](capabilities.md), [kernel_core](kernel-core.md) spawn, [security](security.md) (the capsule spawners verify id-certs) and the [drivers](hardware.md#drivers) helpers.
- **bus** is called by the boot/platform init, the broker, the virtio-rng driver and the arch IOMMU/watchdog code; it calls [arch](arch.md) for the port/MMIO config primitives.
- **drivers** is called heavily by [crypto](crypto.md) for entropy (`virtio_rng`) and by the broker; it calls [bus](hardware.md#bus) to find its device and [arch](arch.md)/[memory](memory.md) for MMIO.

Three PCI paths exist and are distinct: `bus::pci` (the raw bus), `drivers::pci` (a higher-level manager with allow/block lists, MSI, stats) and `hardware::broker::pci` (the brokered config-access gate capsules go through).

## See also

- [Hardware broker](../kernel/hardware-broker.md) and [Broker API](../drivers/broker-api.md): the behavior, kernel and driver sides.
- [Drivers](../drivers/README.md) and [Writing a driver](../drivers/writing-a-driver.md): the capsule driver model.
- [memory](memory.md) and [IOMMU](../kernel/iommu.md): the confinement the broker sets up.
- [PCI and ACPI](../kernel/pci-and-acpi.md): enumeration from the behavior side.
