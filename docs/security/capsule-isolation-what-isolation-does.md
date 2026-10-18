# Capsule isolation: What isolation does not cover

Part of [Capsule isolation](capsule-isolation.md).

## What isolation does not cover

A capsule can do everything its bits allow, and nothing here judges intent. DMA
on a machine without a remapping unit in service is not confined. Side channels
between capsules are not closed, whether they take turns on one CPU or run at
once on cores that share a cache. [Protections and
limits](protections-and-limits.md) lists these and the other gaps with their
code.

## See also

- [Capabilities in the kernel](../kernel/capabilities.md)
- [System calls](../kernel/syscalls.md)
- [IPC](../kernel/ipc.md)
- [Hardware broker](../kernel/hardware-broker.md)
- [IOMMU](../kernel/iommu.md)
- [Broker API for drivers](../drivers/broker-api.md)
- [Manifests and capabilities](../userland/manifests-and-capabilities.md)
- [Protections and limits](protections-and-limits.md)
- [Checking the security claims yourself](checking-the-claims.md)
