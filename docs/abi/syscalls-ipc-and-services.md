# Syscalls: IPC and services

Part of [Syscalls](syscalls.md).

### IPC and services

A message goes to an [endpoint](../overview/glossary-dma-pool.md#endpoint), a port a
service registered, and lands in the [inbox](../overview/glossary-hardware-broker.md#inbox) of
the process that serves it. See [IPC](ipc.md) for the arguments, the limits and
the ports.

| Tag | Number | Name | Capability | Gate | Meaning |
|---|---|---|---|---|---|
| `MISD` | `0x4453494D` | `MkIpcSend` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_ipc.rs:46` | Send a message to the endpoint a port number names. |
| `MIRC` | `0x4352494D` | `MkIpcRecv` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_ipc.rs:46` | Receive from the caller's own inbox (endpoint 0) or an endpoint it owns; a timeout of 0 waits for ever. |
| `MICL` | `0x4C43494D` | `MkIpcCall` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_ipc.rs:46` | Send a request and wait for the reply that carries its correlation token. |
| `MIRF` | `0x4652494D` | `MkIpcRecvFrom` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_ipc.rs:46` | Receive as `MIRC` does, and also write the sender's pid. |
| `MIRY` | `0x5952494D` | `MkIpcReply` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_ipc.rs:46` | Answer a caller whose `MICL` this server took. |
| `MISP` | `0x5053494D` | `MkIpcSendToPid` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_ipc.rs:46` | Send straight to a process's own `proc.<pid>` inbox. |
| `MSVL` | `0x4C56534D` | `MkServiceLookup` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_ipc.rs:46` | Resolve a service name to its port and owning pid. |
| `MSVR` | `0x5256534D` | `MkServiceRegister` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_ipc.rs:46` | Register a service name on a port; few names may be claimed at run time. |

### Processes, memory and time

| Tag | Number | Name | Capability | Gate | Meaning |
|---|---|---|---|---|---|
| `MMAP` | `0x50414D4D` | `MkMmap` | Memory | `can_allocate_memory` `src/syscall/contract/cap_table/mk_proc.rs:38` | Map up to 1 GiB of fresh zeroed pages, at a page-aligned address the caller names or one the kernel picks; returns the address. |
| `MUMP` | `0x504D554D` | `MkMunmap` | Memory | `can_deallocate_memory` `src/syscall/contract/cap_table/mk_proc.rs:39` | Unmap pages the caller mapped. |
| `MCLD` | `0x444C434D` | `MkCapsuleLoad` | CoreExec and IPC and Memory | `grants_all` `src/syscall/contract/cap_table/mk_proc.rs:41-44` | Verify a [capsule](../overview/glossary-boot-root-record.md#capsule) from the ELF, certificate, manifest and trailer the caller passes, and start it; returns the pid. |
| `MCVF` | `0x4656434D` | `MkCapsuleVerify` | CoreExec and IPC and Memory | `grants_all` `src/syscall/contract/cap_table/mk_proc.rs:46-49` | Run the same verification without starting anything, and write a summary. |
| `MEXT` | `0x5458454D` | `MkExit` | valid token | `is_valid` `src/syscall/contract/cap_table/mk.rs:39` | End the calling process with an exit code. |
| `MPAL` | `0x4C41504D` | `MkPidAlive` | valid token | `is_valid` `src/syscall/contract/cap_table/mk.rs:39` | 1 when a pid names a live process, else 0. |
| `MWAT` | `0x5441574D` | `MkWait` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_ipc.rs:35` | Wait, up to a timeout, for a child to exit, and return its exit code. |
| `MKIL` | `0x4C494B4D` | `MkKill` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_ipc.rs:36` | Send SIGINT, SIGTERM or SIGKILL to a child, a supervised guest, or, with ProcessControl or Admin, any process. |
| `MGPD` | `0x4450474D` | `MkGetPid` | CoreExec | `can_getpid` `src/syscall/contract/cap_table/mk_ipc.rs:24` | The caller's pid. |
| `MKAR` | `0x52414B4D` | `MkArgs` | CoreExec | `can_getpid` `src/syscall/contract/cap_table/mk_ipc.rs:25` | Copy the caller's arguments, NUL separated. |
| `MTSP` | `0x5053544D` | `MkThreadSpawn` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_ipc.rs:26` | Start a thread in the caller's address space; returns its id. |
| `MSTB` | `0x4254534D` | `MkSetTls` | valid token | `is_valid` `src/syscall/contract/cap_table/mk_ipc.rs:31` | Set the caller's thread pointer base. |
| `MYLD` | `0x444C594D` | `MkYield` | valid token | `is_valid` `src/syscall/contract/cap_table/mk.rs:39` | Give up the CPU. |
| `MFTW` | `0x5754464D` | `MkFutexWait` | valid token | `is_valid` `src/syscall/contract/cap_table/mk.rs:39` | Sleep while a 32-bit word holds an expected value. |
| `MFTK` | `0x4B54464D` | `MkFutexWake` | valid token | `is_valid` `src/syscall/contract/cap_table/mk.rs:39` | Wake waiters on a word. |
| `MSPI` | `0x4950534D` | `MkSpawnInstance` | Admin or SpawnWindow | `can_spawn_window` `src/syscall/contract/cap_table/mk_surface.rs:37` | Ask for another window of an embedded, attested app. |
| `MTRN` | `0x4E52544D` | `MkToolRun` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_surface.rs:51` | Run a baked, attested tool by name as the caller's child, holding no more than the caller unless the caller holds `SpawnWindow` or `Admin` (`src/userspace/tool_capsules/bound.rs`). |
| `MTMS` | `0x534D544D` | `MkTimeMillis` | valid token | `is_valid` `src/syscall/contract/cap_table/mk.rs:39` | Wall-clock milliseconds; a time correction can move it. |
| `MMON` | `0x4E4F4D4D` | `MkTimeMonotonic` | valid token | `is_valid` `src/syscall/contract/cap_table/mk.rs:39` | Milliseconds since boot; never moves backwards. |
| `MTRT` | `0x5452544D` | `MkTimeRtc` | valid token | `is_valid` `src/syscall/contract/cap_table/mk.rs:39` | Read the real-time clock as a date and time record. |
| `MTAD` | `0x4441544D` | `MkTimeAdjust` | TimeSet | `can_set_time` `src/syscall/contract/cap_table/mk_proc.rs:36` | Correct the wall clock to a Unix time in milliseconds between 2025-01-01 and 2100-01-01 UTC. |
| `MBAT` | `0x5441424D` | `MkBatteryStatus` | valid token | `is_valid` `src/syscall/contract/cap_table/mk.rs:39` | Battery charge; in 0.9.2 always `ENODEV` or `EOPNOTSUPP`, since the kernel runs no AML. |

### Output, input and terminals

| Tag | Number | Name | Capability | Gate | Meaning |
|---|---|---|---|---|---|
| `MPST` | `0x5453504D` | `MkProcStat` | valid token | `is_valid` `src/syscall/contract/cap_table/mk.rs:39` | Read the process table; redacted unless the caller holds AttestRead or ProcessControl. |
| `MOUT` | `0x54554F4D` | `MkProcOutput` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_ipc.rs:32` | Drain one line a child wrote to its `proc.<pid>` inbox. |
| `MPIN` | `0x4E49504D` | `MkProcInput` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_ipc.rs:33` | Feed one message to a child's `stdin.<pid>` inbox. |
| `MSRD` | `0x4452534D` | `MkStdinRead` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_ipc.rs:34` | Read the caller's own stdin. |
| `MSOW` | `0x574F534D` | `MkStdoutWrite` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_device.rs:41` | Program stdout, to the caller's own `proc.<pid>` inbox. |
| `MPVW` | `0x5756504D` | `MkPrivateWrite` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_device.rs:41` | Output only the caller's launcher reads, never the serial line. |
| `MTTY` | `0x5954544D` | `MkTtySet` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_surface.rs:50` | Say which of a child's streams reach a terminal, and its size. |
| `MTTQ` | `0x5154544D` | `MkTtyQuery` | none | `true` `src/syscall/contract/cap_table/mk_surface.rs:51` | Ask whether stdin, stdout or stderr is on a terminal, and its size; else `ENOTTY`. |
| `MDBG` | `0x4742444D` | `MkDebug` | Debug | `can_debug` `src/syscall/contract/cap_table/mk_device.rs:40` | Write one diagnostic line of up to 256 bytes to the boot serial. |
