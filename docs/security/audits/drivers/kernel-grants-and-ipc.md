# Kernel grants and IPC

The device-grant broker, the PCI stack and its boot pass, sender admission to
driver endpoints, the input ring's capabilities and kernel entropy. Index and
notation: README.md. Paths under src/ unless they start with userland/.

## Grant broker: near

The MkDevice*, MkMmio*, MkIrq*, MkDma*, MkPio* and MkPciConfig* syscalls over crate::hardware::broker (claims, grant tables, per-capsule IOMMU domains).

- low: `syscall/microkernel/mmio/map.rs:66` unconditional serial traces and pid 7 hooks on the grant path; `hardware/broker/grant.rs:17` eleven TCB files over 75 lines.

## In-kernel PCI stack and security helpers: partial

Enumeration, capability walks, BAR decode, MSI programming, VMD, src/drivers/pci/security and src/drivers/security.

- **medium** `drivers/pci/security/policy.rs:25`: the PCI and driver security helpers are permissive by default and have no caller on the live path: validation code in the TCB that enforces nothing. -> none
- low: `drivers/security/examples.rs:97` demo driver code built into the kernel; `drivers/pci/error.rs:1` seventeen files over 75 lines.

## Held endpoints and the reply route


## Input capabilities


## Entropy

The in-kernel virtio_rng driver and the capsule are in media.md.

- low: `drivers/virtio_rng/device/random.rs:12` completion wait is a spin count.
