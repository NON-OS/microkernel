# Syscalls the kernel dispatches that no userland code references

Counted by `scripts/check_syscall_reach.py`. The number may only go down.

A syscall on this list is implemented, sits in the dispatch table, and is
called by nothing in the tree. That is the shape of the bug that keeps
recurring: MkCapsuleVerify, MkAttestDoc and MkFutexWait were all here until
something needed them and found them unwired.

Current set, and why each is still here:

- `SYS_SET_TLS` (MSTB): the std PAL's thread support is the intended
  consumer. Check the PAL before treating as dark. (`SYS_THREAD_SPAWN`,
  MTSP, left the list when libc's `thread` module wrapped it for no_std
  workers.)
- `SYS_STDOUT_WRITE` (MSOW): std uses `MkProcOutput`. Dark.
- `SYS_DEV_ROOT_REQUEST` (MDRQ): device root handoff. Dark.
- `SYS_USB_PORT_FACTS` (MUPF): a USB host's port facts (connectable,
  user-visible, internal or external), as capsule_acpi declares them with
  `MkAcpiUsbPorts`. Published by 9b90cdd771 (letter F-36) on the
  DeviceEnum gate. F-36 names no reader, and no USB driver or libc
  wrapper asks yet.
- `SYS_NUMA_NODE_OF` (MNNO): the proximity domain of a physical range.
  Added by 2dcb474b04 (F-40), published in `abi/syscalls.toml` by
  7b99e176f8 (F-41), on the DeviceEnum gate. F-40: the allocator does not
  consume it; nothing in userland asks yet.
- `SYS_NUMA_DISTANCE` (MNDS): SLIT's distance between two domains. Same
  commits, letters and gate as MNNO; nothing asks yet.
- `SYS_DEVICE_NODE` (MDNN): a device's full u32 domain, kept and
  published by 352896cd50 at the ruling (F-41). The device
  record's `numa_node` byte, which F-41 added to the kernel record and
  the libc mirror, carries the same domain up to 254; nothing asks for
  the full value yet.
- `SYS_NUMA_HMAT` (MNHM): HMAT's latency or bandwidth between an
  initiator and a target domain. Published by 9504452578 (F-46) on the
  DeviceEnum gate, as MNDS. F-46 keeps the attached initiator for the
  allocator, unused yet; nothing in userland asks yet.

To lower the baseline, wire a syscall and reduce the number in
`syscall-unreachable-count.txt` in the same commit. To add one, say why
here.
