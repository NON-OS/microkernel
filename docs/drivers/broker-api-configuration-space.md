# Broker API: Configuration space

Part of [Broker API](broker-api.md).

## Configuration space

`mk_pci_config_read(device_id, epoch, offset, width)` returns the value read.
The broker's `read` accepts widths 1, 2 and 4, aligned, inside the first
`CONFIG_LIMIT` (256) bytes (`src/hardware/broker/pci/read.rs:22-48`).

`mk_pci_config_write(device_id, epoch, offset, value)` writes 16 bits, and only
to a few registers. `validate` accepts the Command register, the MSI-X Message
Control register and a short list of vendor bits, and refuses every other offset
(`src/hardware/broker/pci/allowlist.rs:37-60`):

- In Command, `COMMAND_WRITABLE` is Bus Master, Memory Space and Interrupt
  Disable (`src/hardware/broker/pci/command.rs:33`). `validate_command` ORs a
  value made only of those bits into the register, and otherwise requires the
  new value to match the register outside them
  (`src/hardware/broker/pci/command.rs:35-41`).
- In MSI-X Message Control, `MSIX_CONTROL_WRITABLE` is Enable and Function Mask
  (`src/hardware/broker/pci/allowlist.rs:35`).
- `writable` adds a few vendor bits: on Intel HD Audio, TCSEL at 0x44, the
  clock-gating bit at 0x48 and the no-snoop bit at 0x78; on AMD and ATI HD
  Audio, the snoop bits at 0x42; on any network function, the PCIe completion
  timeout bits (`src/hardware/broker/pci/quirk_bits.rs:49-61`).

The constants a driver passes are `MK_PCI_CFG_COMMAND` (0x04),
`MK_PCI_CMD_MEMORY_SPACE` (bit 1), `MK_PCI_CMD_BUS_MASTER` (bit 2),
`MK_PCI_CMD_INTX_DISABLE` (bit 10), `MK_PCI_MSIX_CTRL_FUNCTION_MASK` (bit 14)
and `MK_PCI_MSIX_CTRL_ENABLE` (bit 15)
(`userland/libc/src/broker/pci.rs:27-36`).

## Registers

The call is `mk_mmio_map(device_id, epoch, bar_index, flags, offset, length,
out)`, and `mk_mmio_map` packs the BAR index into the top half of one argument
(`userland/libc/src/broker/mmio.rs:24-36`). `flags` must be 0, since
`FLAGS_KNOWN` is empty (`src/hardware/broker/mmio/map.rs:47-53`).
`map_for_caller` then checks the claim and epoch, that the BAR is a memory BAR
within `bar_count`, and that the request fits the BAR. It maps the pages into
the capsule as user, read-write, uncached and no-execute, and `map_for_caller`
records the [grant](../overview/glossary.md#grant) last
(`src/hardware/broker/mmio/map.rs:49-112`). The offset and length need not be
page aligned: the returned `user_va` already carries the offset into its first
page (`src/hardware/broker/mmio/map.rs:30-35`).

No mapping ever covers an MSI-X table or pending-bit array, of any device. The
mapping is cut short at the page below the first `protected` page, and a request
that starts in one is refused with -1, `WouldExposeMsixTable`
(`src/hardware/broker/mmio/window.rs:32-37`,
`src/syscall/microkernel/mmio/errno_map.rs:33`). `mk_mmio_unmap(grant_id)` gives
a grant back.

Grants land in two windows of every capsule's address space: `USER_MMIO_BASE` to
`USER_MMIO_END` for registers and `USER_DMA_BASE` to `USER_DMA_END` for DMA
buffers (`src/hardware/broker/windows.rs:31-34`). `MkMunmap` and a fixed-address
`MkMmap` refuse any range that `touches_device_window`, so a generic unmap
cannot hand a frame the device still reaches back to the allocator
(`src/syscall/microkernel/memory/munmap.rs:43`,
`src/syscall/microkernel/memory/mmap.rs:35`).

Port I/O exists only on x86_64; other targets build `pio_absent` and the calls
fail with ENOSYS (`src/hardware/broker/mod.rs:39-45`). `mk_pio_grant(device_id,
epoch, bar_index, flags, out)` grants a whole port BAR, and `grant_for_caller`
refuses a BAR that is not a port BAR, has size zero or runs past port 0xFFFF
(`src/hardware/broker/pio/map.rs:32-78`). `mk_pio_read` and `mk_pio_write` name
the grant, an offset and a width of 1, 2 or 4 bytes, `PioWidth`
(`src/hardware/broker/pio/types.rs:39-53`); the kernel runs the `in` or `out`.
