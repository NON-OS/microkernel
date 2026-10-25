# Broker API: Errors

Part of [Broker API](broker-api.md).

## Errors

Every call returns a negative errno on failure, from `ERRNO_PERM` (-1) to
`ERRNO_STALE` (-116) (`src/syscall/microkernel/errnos.rs:22-54`). The ones a
driver meets most:

| Value | Name | Usual cause |
|---|---|---|
| -1 | EPERM | not the holder, an MSI-X page in the request, or a device a remapping unit would not take |
| -12 | ENOMEM | no address space or no vector left |
| -16 | EBUSY | device or interrupt line already held, or a second MSI or MSI-X bind |
| -19 | ENODEV | no such device |
| -22 | EINVAL | bad BAR, range, length or alignment |
| -34 | ERANGE | a DMA32 buffer above 4 GiB |
| -95 | EOPNOTSUPP | an unknown flag bit |
| -116 | ESTALE | an old claim epoch |

[Errors](../abi/errors.md) has the full list.

## Known gaps

- `DEVICE_FLAG_CLAIMED` is never set, so a driver cannot tell from the list that
  a device is taken; the claim answers -16 instead.
- `abi/syscalls.toml` describes `MkPioGrant` with `port_base` and `port_count`
  arguments, while `sys_pio_grant` takes a BAR index
  (`abi/syscalls.toml:393-396`, `src/syscall/microkernel/pio/grant.rs:40-46`).
  The code is what runs.
- Without a remapping unit in service a claimed device can reach all of memory.
  The broker says so in the log on each claim.

## See also

- [Drivers](README.md)
- [Writing a driver](writing-a-driver.md)
- [Platform](platform.md)
- [Broker ABI](../abi/broker.md)
- [Errors](../abi/errors.md)
- [The hardware broker](../kernel/hardware-broker.md)
- [IOMMU](../kernel/iommu.md)
- [Capabilities](../kernel/capabilities.md)
