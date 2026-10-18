# Capsule isolation: The Linux personality's sandbox

Part of [Capsule isolation](capsule-isolation.md).

## The Linux personality's sandbox

The [Linux personality](../overview/glossary-linux-personality.md#linux-personality),
`capsule_linux`, runs unmodified x86_64 Linux programs as guests and answers
their system calls. [Linux personality](../userland/linux-personality.md) lists
which calls are served and which are refused.

A guest holds nothing. `empty_guest` creates the process, installs an empty
capability set with `install_spawn`, and logs `[FOREIGN] guest pid=<pid>
caps=<mask>` with the mask read back (`src/process/foreign/spawn.rs:53-79`).
With no bits, every NONOS call a guest could make fails the cap table, except
`MkTtyQuery`, which only describes the caller's own streams
(`src/syscall/contract/cap_table/mk_surface.rs:45-51`).

A NONOS system call number is four ASCII bytes packed by `tag4`
(`src/syscall/abi/tag.rs:20-22`), so a Linux number never matches one.
`syscall_handler` finds no `SyscallNumber` for it and passes it to `redirect`
(`src/arch/x86_64/syscall/manager/entry.rs:38-53`). `redirect` parks the guest
thread and wakes its supervisor, the capsule that created it, which answers; a
process with no supervisor gets `ENOSYS` (`src/process/foreign/trap.rs:28-57`).

The supervisor needs `ForeignExec` for every call that touches a guest
(`can_foreign_exec` in `src/syscall/contract/cap_table/mk_device.rs:55-69`).
Among the `Capsule.mk` files in this tree, only `capsule_linux` requests it, in
`CAPSULE_REQUIRED_CAPS` (`userland/capsule_linux/Capsule.mk:45`). A supervisor
reaches only its own guests: the map, copy, protect and unmap peer calls start
with `supervised_asid`, which refuses a pid the caller does not supervise
(`src/process/foreign/peer_guard.rs:65-73`), and `sys_peer_tls` makes the same
check (`src/process/foreign/peer_tls.rs:24-34`). A peer call touches only the
guest's user half and moves at most `MAX_SPAN`, 1 MiB, at a time
(`src/process/foreign/peer_guard.rs:28-39`). One supervisor holds at most
`MAX_GUESTS`, 1024 guests and guest threads; past that a spawn is `EAGAIN`
(`src/process/foreign/room.rs:33-38`).

Inside `capsule_linux`:

- Only the `INSTALL` role asks for `Network`, because a package mirror is
  reached through `net.sockets`, which serves only holders of `Network`
  (`src/userspace/capsule_linux/roles.rs:35-44`). The `RUN` and `TERMINAL` roles
  ask for nothing extra (`src/userspace/capsule_linux/roles.rs:46-67`).
- `under_root` places every guest path under the family's root or its private
  directories, and `writable` keeps the shared tree read-only to guests; only an
  install writes it (`userland/capsule_linux/src/linux/file/root.rs:34-65`).
- `not_loopback` refuses a bind or listen outside 127.0.0.0/8 with `EACCES`
  (`userland/capsule_linux/src/linux/net/policy.rs:33-41`), and `refuse_out`
  answers a datagram to anywhere outside the family with `ENETUNREACH`
  (`userland/capsule_linux/src/linux/net/policy.rs:43-53`).
- The personality holds `Debug` on every build, hardened and air-gapped
  included, because `LINUX_CAPS` names it
  (`src/userspace/capsule_linux/spawn.rs:39-43`). `sys_mk_debug` writes its
  lines to the serial console, except during a Terminal's private run, which
  `is_private_run` recognises and whose lines go only to the Terminal
  (`src/syscall/microkernel/debug.rs:61-65`).

## Host tests

Some of these rules are compiled into host test crates by path, so the tests run
the kernel's own code. The broker's `unconfined_allowed` is mounted as `posture`
in `kernel_proofs` (`userland/kernel_proofs/src/confine_posture/mod.rs:22-23`),
and `held.rs`, with `HELD` and `inbox_admits`, as `held`
(`userland/kernel_proofs/src/ipc_held_tests/mod.rs:23-24`). At this commit
`kernel_proofs` runs 388 tests and all pass.
