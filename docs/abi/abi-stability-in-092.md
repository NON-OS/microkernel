# The NONOS ABI: Stability in 0.9.2

Part of [The NONOS ABI](README.md).

## Stability in 0.9.2

NONOS 0.9.2 makes no written stability promise for argument lists, record
layouts or behaviour. What the code does enforce is narrower:
`check_abi_stable.py` reads the three tables named in `TABLES` and fails when a
published syscall number, errno or capability bit is renumbered or withdrawn,
when two names share a value, or when an entry is published without being added
to the baseline (`scripts/check_abi_stable.py:18-29`). Its baseline,
`scripts/baselines/abi-published.txt`, holds 192 entries: 130 syscall numbers,
25 errnos and 37 capability bits. The `static-abi` entry of `staticChecks` runs
it together with `check_syscall_abi.py`, `check_syscall_args.py`,
`check_caps_abi.py` and other scripts, in order, and stops at the first that
fails (`tools/nix/checks.nix:197-234`). In the flake check run on this commit,
`static-abi` failed on `check_prebuilt.py`, which reports two unclassified
binaries and is not an ABI check. `check_syscall_abi.py` and
`check_abi_stable.py` ran before it and passed; `check_syscall_args.py` and
`check_caps_abi.py` come after it and did not run there. All four pass when run
by hand, as [below](abi-how-the-tables.md#how-the-tables-are-checked).

The version fields are labels, not checks: `abi/syscalls.toml` sets `version` to
3 and `abi` to `nonos-sys-v1` (`abi/syscalls.toml:6-7`), and nothing in the
kernel reads them.

The published files still disagree with the code in places. Where they differ,
the code decides, and these pages follow it:

- The argument lists in `abi/syscalls.toml` are checked for their length only,
  and only where the `ARM` pattern of `check_syscall_args.py` finds the call's
  dispatch arm under `src/syscall/microkernel/dispatch/`
  (`scripts/check_syscall_args.py:17-40`): 105 calls at this commit, with the
  other 25 unchecked. Several lists are wrong. For `MICL`, the file puts an op
  code second, while `sys_ipc_call` takes no op code and ends with a timeout
  (`src/syscall/microkernel/ipc/call/sys_ipc_call.rs:48-55`). For `MPGT`,
  `sys_pio_grant` takes a BAR index and flags where the file lists a port range
  (`src/syscall/microkernel/dispatch/pio.rs:27-30`). For `MIRA` and `MIRP`,
  `sys_irq_ack` and `sys_irq_poll` take a grant id where the file lists a slot
  (`src/syscall/microkernel/irq/ack.rs:34`,
  `src/syscall/microkernel/irq/poll.rs:39`). For `GDIM`,
  `handle_display_dimensions` takes four arguments where the file lists two
  (`src/syscall/dispatch/router/graphics_backend.rs:47`). For `MPCW`, the file
  gives the value as a `u32`, and `u16_arg` refuses anything wider than 16 bits
  (`src/syscall/microkernel/dispatch/debug.rs:31-36`).
- The `[limits]` section of `abi/syscalls.toml` publishes `max_mmap_bytes` as
  268435456, 256 MiB (`abi/syscalls.toml:48-52`). `sys_mmap` refuses only a
  length above `MAX_MMAP_SIZE`, 1 GiB
  (`src/syscall/microkernel/memory/consts.rs:19`). No script compares that
  section with the code.
- The `[token]` section of `abi/caps.toml` publishes `id_bytes` and `mac_bytes`
  of 32 (`abi/caps.toml:58-65`). `CapabilityToken` holds a `u64` `token_id` and
  a 64-byte `signature` (`src/capabilities/token/types/defs.rs:22-32`).
  [Capabilities](capabilities-changing-capabilities-at.md#limits) has the MAC as
  well.
- `abi/driver_broker_abi.md` describes an older device record and lists claim
  and map as reserved. [Broker](broker.md) has the current calls and layouts.
