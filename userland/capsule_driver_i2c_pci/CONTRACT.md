# capsule_driver_i2c_pci: contract

What the capsule may call, who may call it, which devices that caller can
reach, and what the capsule keeps. The summary is in [README.md](README.md).

## Microkernel contract

The manifest grants `IPC`, `Memory`, `Driver`, `DeviceEnum` and `Mmio`:
`CAPSULE_REQUIRED_CAPS = 0x38018`. No `Irq`: the engine polls, and the kernel
lets an `Irq` holder post input events, which a bus driver has no business
doing. The capsule reaches hardware only through `MkDeviceList`,
`MkDeviceClaim`, `MkPciConfigWrite` (Memory Space and Interrupt Disable, never
Bus Master) and `MkMmioMap`. The kernel validates the signed manifest, brokers
grants, routes IPC, and revokes all grants on capsule exit.

## The client and its reach

Only `driver.i2c_hid0` may send to `driver.i2c_pci0`: the kernel holds the
endpoint to it (`src/services/registry/held.rs`), by name and by pid. Even that
client reaches only the devices setup put in reach: `OP_TRANSFER` and `OP_PROBE`
to any other address are refused with `E_INVAL` before the bus
(`src/transaction/types/reach.rs`). With nothing declared, nothing is in reach.

A device that returned its HID descriptor to setup's own probe is alone in
reach. A bind made before any did, while the pad may still be asleep, keeps in
reach each touchpad declared on the bound controller or on none, and the one
device the read-only sweep found for a declaration with no address; an address
declared twice carries the register of the declaration bound, the one
`OP_ACPI_HID` answers (`src/setup/sequence/settle.rs`). When no controller the
firmware named comes up and the first that does is taken, nothing is bound and
every declared address is in reach (`declared` in
`src/setup/sequence/order.rs`). The first time the client then writes exactly
the HID descriptor register declared for one of those addresses and reads a
valid HID descriptor back (wHIDDescLength 30, bcdVersion 1.00) through
`OP_TRANSFER`, the reach becomes that address alone for the rest of the session
(`src/transaction/types/narrow.rs`). Nothing else narrows it, and nothing widens
it again.

## Authority

The capsule may enumerate PCI devices, claim one Intel LPSS I2C function and map
BAR0. It has no DMA, interrupt, PIO, filesystem, network, display, credential,
input or input focus authority. Bus mastering stays off and the LPSS integrated
DMA engine stays in reset (`src/init/lpss_init.rs`).

## Privacy and persistence

The capsule stores no touch events, gestures, sensor readings, device names, HID
reports, or ACPI tables. Runtime state is limited to grant ids, PCI identity,
controller clock metadata, the bound touchpad's address and declaration facts,
the reach, and side-effect-free register snapshots.

## State ownership

`driver.i2c_pci0` owns only controller-facing I2C state: PCI identity, MMIO
grant, component id, controller enable/status values, FIFO levels, timing
registers, the reach, and the bus state (ready, faulting, reset, stuck).
HID-over-I2C owns descriptors and reports; the input router owns focus, routing,
and policy.

## Operating rules

- Do not parse HID reports in this capsule.
- Do not persist touch, gesture, or sensor data.
- Do not import kernel driver, memory, paging, or hardware internals.
- Do not add inline architecture assembly or raw PIO.
- Keep bus transfers bounded and fail closed on timeout or controller abort.
