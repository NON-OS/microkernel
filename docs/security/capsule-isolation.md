# Capsule isolation

How the NONOS kernel keeps one capsule from reaching another: separate address
spaces, a [capability](../overview/glossary.md#capability) check on every NONOS
system call, rules on who may send to whom, confined drivers and a sandbox for
Linux programs.

## Parts

- [Address spaces](capsule-isolation-address-spaces.md)
- [The capability check on every NONOS system
  call](capsule-isolation-the-capability-check.md)
- [The capability check on every NONOS system call,
  continued](capsule-isolation-the-capability-check-2.md)
- [IPC: who may send to whom](capsule-isolation-ipc-who-may.md)
- [Drivers: the broker and the IOMMU](capsule-isolation-drivers-the-broker.md)
- [Drivers: the broker and the IOMMU,
  continued](capsule-isolation-drivers-the-broker-2.md)
- [The Linux personality's sandbox; Host
  tests](capsule-isolation-the-linux-personalitys.md)
- [What isolation does not cover; See
  also](capsule-isolation-what-isolation-does.md)
