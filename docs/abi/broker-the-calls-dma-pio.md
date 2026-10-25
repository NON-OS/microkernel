# Broker: The calls, continued

Part of [Broker](broker.md).

## The calls, continued

- `MkDmaMap` takes a length in bytes, a non-zero multiple of 4096, and
  `validate` refuses `DMA_MAP_HIGH` with `DMA_MAP_DMA32`, `DMA_MAP_COHERENT`
  with `DMA_MAP_WC`, and `DMA_MAP_TO_DEVICE` with `DMA_MAP_FROM_DEVICE`
  (`src/hardware/broker/dma/map/validate.rs:39-52`). A map longer than its
  class's ceiling, below, is refused as `BadLengthForClass`
  (`src/hardware/broker/dma/map/validate.rs:63-67`). A `DMA_MAP_DMA32` map that
  would end above 4 GiB fails with `Above4G`, returned as `ERANGE`
  (`src/syscall/microkernel/dma.rs:109-111`).
- The PCI calls check a live claim and an allowlist of registers in the broker,
  and both `sys_pci_config_read` and `sys_pci_config_write` ask for `Driver`
  themselves, so `Admin` alone is refused
  (`src/syscall/microkernel/pci/config.rs:27-34`,
  `src/syscall/microkernel/pci/config.rs:42-49`). A write is one 16-bit register, and
  `u16_arg` refuses a wider value with `EINVAL`
  (`src/syscall/microkernel/dispatch/debug.rs:31-36`). A Command value of
  writable bits alone is ORed on; bits 16 to 31 of `offset` name the bits to
  change instead, so a driver can clear Bus Master or Interrupt Disable
  (`mk_pci_config_update`, `src/hardware/broker/pci/change.rs`).
- Port I/O exists on x86_64 only. On aarch64 and riscv64 every port call returns
  `ENOSYS` (`src/syscall/microkernel/pio/mod.rs:17-49`). `from_arg` reads `w`,
  the access width, as 1, 2 or 4 bytes and refuses anything else
  (`src/syscall/microkernel/pio/width.rs:23-30`).
- `MkHvBackdoor` makes one VMware backdoor call, on a host that answers it:
  VMware, or QEMU's vmport under KVM or TCG; elsewhere it returns `ENODEV`
  (`src/arch/x86_64/hypervisor/backdoor_rule.rs`). Only the absolute pointer's
  calls are allowed: GETVERSION (10), ABSPOINTER_DATA (39) for 1 to 4 words,
  ABSPOINTER_STATUS (40), and ABSPOINTER_COMMAND (41) with enable, disable,
  relative or absolute. Any other command or argument returns `EINVAL`, so the
  backdoor's RPC channel, which carries data between guest and host, stays
  closed.
- The `MkHv*` VMBus calls need `Driver` and a claim on the VMBus root, a `VIRT`
  record (vendor 0x1414, device 0x5642) the kernel registers once the Hyper-V
  SynIC is up (`src/hardware/broker/vmbus.rs`). The SynIC runs on VP 0, with
  VMBus on SINT 2 at vector 0xC1 (`src/arch/x86_64/hypervisor/hv/regs.rs`).
  `MkHvPost` posts only the channel messages a guest sends (type 1, on
  connection 1 or 4), aimed at VP 0 and SINT 2. Every guest page a message names
  must lie in a DMA grant the caller holds on the VMBus root: the contact's
  monitor pages, its interrupt page before VMBus 5.0, and a GPADL's frames
  (`src/arch/x86_64/hypervisor/hv/vmbus_out.rs`). Any other message returns
  `EINVAL`, a page not held `EPERM`, and a host with no room `EAGAIN`.
  `MkHvRecv` returns `EAGAIN` when no message waits.
