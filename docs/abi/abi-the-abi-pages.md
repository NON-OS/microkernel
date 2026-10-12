# The NONOS ABI: The ABI pages

Part of [The NONOS ABI](README.md).

## The ABI pages

- [Syscalls](syscalls.md): all 130 calls with number, name, capability and
  meaning, the capability check, and the graphics and input structures.
- [Errors](errors.md): the codes handlers return and every errno the kernel
  defines.
- [Capabilities](capabilities.md): the 36 capability bits, what each admits, and
  the groups `abi/caps.toml` publishes.
- [Broker](broker.md): the calls a driver makes to the device
  [broker](../overview/glossary.md#broker), the records they exchange, and the
  broker's constants.
- [IPC](ipc.md): the IPC calls, the message envelope, the limits, and the
  well-known service ports.

[Driver broker ABI](driver_broker_abi.md) in this directory is the broker call
reference built from the dispatch arms: every broker call's registers, the
output records, the device record, the class IDs and the errors.
`nonos-ci/run-static-checks.sh` greps it for the broker call names (`abi_doc`,
`nonos-ci/run-static-checks.sh:1850`), so it is kept in step with the handlers.
[Broker](broker.md) is the fuller narrative of the same surface.

## What the ABI is made of

The code is the authority. Three things in the kernel decide every call:

- `REGISTRY` lists every number the kernel accepts, one table per family
  (`src/syscall/abi/registry/mod.rs:27-28`). There are 130 calls in 0.9.2,
  listed in [Syscalls](syscalls.md).
- `is_allowed` decides which capability admits which call
  (`src/syscall/contract/cap_table/mod.rs:28-34`). The 36 capability bits are
  listed in [Capabilities](capabilities.md).
- `SyscallResult::error` turns a refusal into a negative errno
  (`src/syscall/types/result.rs:35-37`). The codes are listed in
  [Errors](errors.md).

Three files in `abi/` publish the same contract for toolchains and other
readers: `abi/syscalls.toml` (numbers, argument lists, errors, limits),
`abi/caps.toml` (capability bits and groups) and `abi/wire.toml` (structure
layouts). The kernel does not read them. Scripts under `scripts/` compare them
with the code, as described in [How the tables are
checked](abi-how-the-tables.md#how-the-tables-are-checked).

On the user side, `raw_syscall` in `userland/nonos_abi` is the trap trampoline
that `nonos_runtime`, `nonos_ipc` and the other `nonos_*` crates call through
(`userland/nonos_abi/src/raw.rs:17-39`), and `userland/libc` wraps the calls
under C names such as `mk_ipc_send` (`userland/libc/src/ipc/send.rs:20`).
