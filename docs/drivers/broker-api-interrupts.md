# Broker API: Interrupts

Part of [Broker API](broker-api.md).

## Interrupts

`mk_irq_bind(device_id, epoch, irq_source, flags, vector_count, out)` binds one
of three kinds, and `mk_irq_bind` documents them
(`userland/libc/src/broker/irq.rs:17-51`):

- `flags` 0 is legacy INTx: `irq_source` is the record's line and `vector_count`
  is 0.
- `MK_IRQ_BIND_MSIX` takes a run of 1 to 64 vectors, no more than the device's
  MSI-X table holds, programmed from entry 0; `irq_source` is 0. Grant ids and
  vectors of the run are the base plus `i`.
- `MK_IRQ_BIND_MSI` takes one MSI vector, for a function without MSI-X.

A device holds one MSI or MSI-X bind at a time: the MSI-X and MSI validators
answer a second one with `AlreadyBound`, which is -16
(`src/hardware/broker/irq/validate/msix.rs:68-70`,
`src/hardware/broker/irq/validate/msi.rs:43-44`).

The broker pool holds 64 vectors, `BROKER_VEC_COUNT`
(`src/arch/x86_64/interrupt/broker/vectors.rs:41`). `bind` checks the claim and
epoch and picks the path from the flags
(`src/hardware/broker/irq/bind/bind.rs:23-39`). A bind fails with -1 on a line
the kernel keeps for itself (`ReservedGsi`), -16 on a line another grant holds
and -12 when no vector is free
(`src/syscall/microkernel/irq/errno_map.rs:33-38`).

When the interrupt fires, `on_vector` masks an INTx line at the IO-APIC, bumps
the grant's sequence counter, wakes a waiting driver and sends EOI
(`src/hardware/broker/irq/dispatch.rs:32-73`). The driver then uses one of
these:

- `mk_irq_wait(grant_id, last_seq, timeout_ms, out_seq)` sleeps until the
  counter moves past `last_seq`. Grant 0 waits on every grant the caller holds.
  Timeout 0 means `DEFAULT_WAIT_MS`, 100 ms
  (`src/syscall/microkernel/irq/wait.rs:30-58`). It returns
  `MK_IRQ_WAIT_TIMED_OUT` (1) when it slept out the whole timeout with the
  counter unmoved, 0 otherwise, and callers loop because early wakes are part of
  the contract (`userland/libc/src/broker/irq_wait.rs:22-44`).
- `mk_irq_poll(grant_id, out)` reads the counter and an overflow count without
  sleeping, through the broker's `poll`
  (`src/hardware/broker/irq/poll.rs:27-35`).
- `mk_irq_ack(grant_id)` unmasks an INTx line once the driver has cleared the
  device's status; for MSI and MSI-X `ack_grant` changes nothing
  (`src/hardware/broker/irq/release/ack.rs:25-42`).

`mk_irq_unbind(grant_id)` gives the grant back.

## Release

`mk_device_release(device_id)` ends the claim. `sys_device_release` first stops
the device mastering the bus with `quiesce_held_device`, then tears down the
MMIO, IRQ, DMA and PIO grants on it, and drops the claim last
(`src/syscall/microkernel/device.rs:84-113`). The broker's `release` turns bus
mastering off again, logs whether the bit stuck, and detaches the device from
the capsule's domain (`src/hardware/broker/claim/release.rs:24-36`).

A driver that exits or crashes loses the same things. The exit path calls
`release_all_for_pid` and the IRQ, DMA and PIO equivalents for the dying process
(`src/process/exit/teardown.rs:48-51`).
