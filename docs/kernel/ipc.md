# IPC

How a [capsule](../overview/glossary.md#capsule) sends a message through the NONOS kernel: [endpoints](../overview/glossary.md#endpoint) and [inboxes](../overview/glossary.md#inbox), the size and queue limits, who may send to whom, and how a receiver blocks and times out.

## The model

Every capsule is a ring 3 process with an inbox of its own, named `proc.<pid>`. The spawn path registers it with `register_inbox`, together with a second inbox `stdin.<pid>` that the parent feeds (`src/kernel_core/process_spawn/capsule_spawn/runner/install/own_inboxes.rs:29-38`). The stdin inbox holds at most 64 messages, set by `STDIN_CAPACITY` (`src/kernel_core/process_spawn/capsule_spawn/runner/install/own_inboxes.rs:27`).

A service is an endpoint: a row in the kernel's service registry with a name, a port, the pid that serves it and the [capability](../overview/glossary.md#capability) bits a sender must hold, kept in `ServiceEndpoint` (`src/services/registry/endpoint.rs:22-27`). The registry holds at most `MAX_SERVICES` rows, 256 (`src/services/registry.rs:38`).

A message sent to any endpoint of a capsule lands in that capsule's `proc.<pid>` inbox: `kernel_route_ipc_corr` picks the destination that way unless the endpoint is owned by `KERNEL_OWNER` (`src/ipc/kernel_ipc.rs:67-73`). `KERNEL_OWNER` is pid 0 and marks a [reply inbox](../overview/glossary.md#reply-inbox) that the kernel itself drains (`src/ipc/nonos_inbox/registry.rs:44-47`).

The envelope is `IpcMessage`: sender name, destination name, payload, a millisecond timestamp, a correlation word and a 64 bit checksum (`src/ipc/nonos_channel/message.rs:24-32`). The kernel writes the sender as `proc.<caller pid>` when `kernel_route_ipc_corr` builds the envelope with `IpcMessage::new`, so a capsule cannot choose its own sender name (`src/ipc/kernel_ipc.rs:74-79`). The checksum is a keyed BLAKE3 hash over both names, the timestamp and the payload, cut to its last eight bytes in `compute_checksum` (`src/ipc/nonos_channel/hash.rs:63-80`). Its key comes from `init_ipc_secret`, which derives it with BLAKE3 from 32 random bytes at boot (`src/ipc/nonos_channel/hash.rs:25-34`); boot stops if that fails, in `init_core_services` (`src/kernel_core/init/entry/init_core_services.rs:37-38`).

## The IPC system calls

All eight calls need the `IPC` capability in the caller's token; the table entry for them is `can_ipc` (`src/syscall/contract/cap_table/mk.rs:109-116`). The numeric dispatcher passes the arguments as shown in `handle` (`src/syscall/microkernel/dispatch/ipc.rs:26-41`).

| Tag | Name | What it does |
|---|---|---|
| `MISD` | `MkIpcSend` | Send `buf[..len]` to an endpoint, by port or by `endpoint.<n>` name. |
| `MIRC` | `MkIpcRecv` | Take the next message from the caller's own inbox (endpoint 0) or from an endpoint it owns. |
| `MIRF` | `MkIpcRecvFrom` | As `MkIpcRecv`, and also write the sender's pid to a user pointer. |
| `MICL` | `MkIpcCall` | Send a request with a fresh [correlation token](../overview/glossary.md#correlation-token) and wait for the matching reply. |
| `MIRY` | `MkIpcReply` | Answer a caller that is waiting in `MkIpcCall`. |
| `MISP` | `MkIpcSendToPid` | Send straight into another process's `proc.<pid>` inbox. |
| `MSVL` | `MkServiceLookup` | Resolve a service name to its port and the pid that serves it. |
| `MSVR` | `MkServiceRegister` | Claim a service name and port at run time. |

The four letter tags are [syscall tags](../overview/glossary.md#syscall-tag); [System calls](syscalls.md) explains them.
