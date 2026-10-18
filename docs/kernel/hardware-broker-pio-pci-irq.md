# The hardware broker: port I/O, PCI configuration and interrupts

Part of [The hardware broker](hardware-broker.md).

## Port I/O

On x86_64 a driver holding `Pio` can ask for a grant on an I/O port BAR of a claimed device. It never runs `in` or `out` itself: `MkPioRead` and `MkPioWrite` are carried out by the kernel, at widths 1, 2 and 4 bytes, against the grant table (`src/hardware/broker/pio/types.rs:17-58`). Other architectures have no port I/O and the calls return `ENOSYS`; `pio_absent` stands in there (`src/hardware/broker/mod.rs:39-45`).

## PCI configuration

`MkPciConfigRead` needs `Driver` and the claim. `read` accepts widths 1, 2 and 4, aligned, inside the first 256 bytes (`src/hardware/broker/pci/read.rs:22-48`).

`MkPciConfigWrite` accepts few changes. In the Command register only Bus Master, Memory Space and Interrupt Disable may change, as `COMMAND_WRITABLE` says (`src/hardware/broker/pci/command.rs:22-33`). In MSI-X Message Control only Enable and Function Mask may change, and otherwise only a few vendor bits for HD Audio controllers and the PCI Express completion timeout of a network controller, listed in `writable` (`src/hardware/broker/pci/quirk_bits/writable.rs:25-41`). Any other offset, BARs, the interrupt line, the IDs and the capability pointers among them, `validate` refuses before the write reaches the bus (`src/hardware/broker/pci/allowlist.rs:37-60`).

## Interrupts

The routes are set up before any capsule starts: `init_broker_irq_routing` programs the IO-APIC from the ACPI tables on x86_64 and has nothing to do on aarch64, where the GIC is set up earlier (`src/arch/init_broker_irq_routing.rs:17-34`). On x86_64 the broker owns 64 vectors, `BROKER_VEC_MIN` `0x81` to `BROKER_VEC_MAX` `0xC0` (`src/arch/x86_64/interrupt/broker/vectors.rs:39-41`).

`MkIrqBind` needs `Irq` and the claim. The flags pick the path: 0 for a legacy INTx line, `BIND_MSI` 2 for MSI, `BIND_MSIX` 1 for MSI-X with a `vector_count` (`src/hardware/broker/irq/types.rs:24-44`). For MSI-X the kernel programs the table itself and returns an `IrqBindResult` with a base grant id and vector; the driver derives the others as `grant_id + i` (`src/hardware/broker/irq/types.rs:46-54`). A line the kernel routes to itself, such as the keyboard's, is refused, as `is_reserved` decides (`src/hardware/broker/irq/reserved.rs:50-59`).

When the vector fires, `on_vector` adds one to the grant's counter, masks an INTx line at the IO-APIC, wakes the driver if it is waiting and sends the end of interrupt; it takes no locks and touches no IPC (`src/hardware/broker/irq/dispatch.rs:17-73`).

`MkIrqWait(grant_id, last_seq, timeout_ms, out)` sleeps until the counter moves past `last_seq` and writes the new count to `out`. A `grant_id` of 0 waits on every grant the caller holds, and a `timeout_ms` of 0 means `DEFAULT_WAIT_MS`, 100 ms (`src/syscall/microkernel/irq/wait.rs:17-30`). It returns 0 when the count moved or another wake came first, and 1 when it slept out the whole timeout, as `verdict` decides (`src/syscall/microkernel/irq/timeout.rs:19-36`). Spurious returns are part of the contract; the driver checks the count and waits again. `MkIrqAck` unmasks an INTx line again and does nothing for MSI and MSI-X, in `ack_grant` (`src/hardware/broker/irq/release/ack.rs:25-42`).
