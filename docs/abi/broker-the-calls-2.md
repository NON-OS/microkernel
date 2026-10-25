# Broker: The calls, continued

Part of [Broker](broker.md).

## The calls, continued

- `MkDeviceList` with class 0 lists every device; `list_by_class` filters on any
  other value (`src/hardware/broker/table/list.rs:29-34`). With `count` 0,
  `sys_device_list` returns how many devices match and writes nothing
  (`src/syscall/microkernel/device.rs:40-45`).
- `claim` asks `attach` to move the device into the claiming capsule's [IOMMU
  domain](../overview/glossary-hardware-broker.md#iommu-domain) before it powers the device, and
  refuses the claim with `Unconfined` when an IOMMU unit in service will not
  take it (`src/hardware/broker/claim/claim.rs:33-40`). `sys_device_claim`
  returns that refusal as `EPERM` (`src/syscall/microkernel/device.rs:67-81`).
  When no IOMMU unit in service covers the device, or its interrupts are not
  remapped, the claim is refused with `Unconfined` too; only a kernel built
  with a development opt-in lets it ahead, the device then reaching all of
  memory, and the serial log says so for that claim
  (`src/hardware/broker/confine/decide.rs`). A record with no PCI address,
  such as an ACPI or PS/2 one, is not attached at all, since `pci_address` finds
  nothing for it (`src/hardware/broker/confine/table.rs:39-43`).
- `sys_device_release` stops bus mastering first, then tears down every MMIO,
  IRQ, DMA and port grant on the device, then drops the claim
  (`src/syscall/microkernel/device.rs:84-102`). When a capsule exits, the exit
  path calls `release_all_for_pid` and its IRQ, DMA and port siblings, which
  release every claim and grant it still holds
  (`src/process/exit/teardown.rs:48-51`).
- `MkMmioMap` needs seven inputs and has six registers, so `mmio_map` takes the
  third as the BAR index in the high 32 bits and flags in the low 32 bits
  (`src/syscall/microkernel/dispatch/unpack.rs:19-40`). One flag is accepted,
  `MK_MMIO_MAP_WC` (bit 0): write-combining, only on a BAR with
  `BAR_FLAG_PREFETCH` and for pages wholly inside it (`EINVAL` otherwise), and
  uncached until every CPU's PAT has its WC entry
  (`src/hardware/broker/mmio/cache.rs`). Where the kernel maps the range
  itself (the boot frame buffer, write-combining) the capsule gets the kernel's
  memory type whatever it asked, since one page under two memory types is
  undefined; two kernel mappings of the range that disagree refuse the map
  (`EINVAL`). Any other bit is `ENOTSUP`
  (`src/hardware/broker/mmio/memory_type.rs`).
- `MkIrqBind` takes `flags` 0 for INTx, `BIND_MSIX` or `BIND_MSI`, never two
  (`src/hardware/broker/irq/types.rs:17-29`). For INTx `vector_count` is 0, and
  for MSI-X `irq_source` is 0 and `vector_count` is 1 or more, with vector i on
  grant `grant_id + i` (`src/hardware/broker/irq/types.rs:31-49`).
  `validate_msix_request` also refuses more vectors than the broker's pool or
  the device's MSI-X table holds
  (`src/hardware/broker/irq/validate/msix.rs:46-64`). `validate_msi_request`
  takes exactly one vector, with `irq_source` 0
  (`src/hardware/broker/irq/validate/msi.rs:26-46`). For INTx on x86_64,
  `validate_intx_request` wants the record's `irq_line` as `irq_source`
  (`src/hardware/broker/irq/validate/intx.rs:35-38`). On aarch64 and riscv64
  only INTx exists: `bind` refuses any flag, and `irq_source` must equal the
  record's `irq_source`, on aarch64 a GIC SPI from 32 to 1019
  (`src/hardware/broker/irq/aarch64/bind.rs:25-61`,
  `src/hardware/broker/irq/riscv64/bind.rs:33-67`). On x86_64 the broker's
  vectors are `BROKER_VEC_MIN` to `BROKER_VEC_MAX`, 64 of them
  (`src/arch/x86_64/interrupt/broker/vectors.rs:39-41`).
- `MkIrqWait` with `grant_id` 0 waits on every grant the caller holds. A timeout
  of 0 means `DEFAULT_WAIT_MS`, 100 ms
  (`src/syscall/microkernel/irq/wait.rs:17-30`). It can return early on any
  wake, so a driver calls it again with the `last_seq` it was given
  (`src/syscall/microkernel/irq/wait.rs:17-22`).
