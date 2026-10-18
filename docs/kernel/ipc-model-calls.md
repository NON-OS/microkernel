# IPC: the model, the calls and limits

Part of [IPC](ipc.md).

## The model

Every capsule is a ring 3 process with an inbox of its own, named `proc.<pid>`. The spawn path registers it with `register_inbox`, together with a second inbox `stdin.<pid>` that the parent feeds (`src/kernel_core/process_spawn/capsule_spawn/runner/install/own_inboxes.rs`, `fn register`). The stdin inbox holds at most 64 messages, set by `STDIN_CAPACITY` (`src/kernel_core/process_spawn/capsule_spawn/runner/install/own_inboxes.rs`, `const STDIN_CAPACITY`).

A service is an endpoint: a row in the kernel's service registry with a name, a port, the pid that serves it and the [capability](../overview/glossary-boot-root-record.md#capability) bits a sender must hold, kept in `ServiceEndpoint` (`src/services/registry/endpoint.rs`, `struct ServiceEndpoint`). The registry holds at most `MAX_SERVICES` rows, 256 (`src/services/registry.rs`, `const MAX_SERVICES`).

A message sent to any endpoint of a capsule lands in that capsule's `proc.<pid>` inbox: `kernel_route_ipc_corr` picks the destination that way unless the endpoint is owned by `KERNEL_OWNER` (`src/ipc/kernel_ipc.rs`, `fn kernel_route_ipc_corr`). `KERNEL_OWNER` is pid 0 and marks a [reply inbox](../overview/glossary-peer-list.md#reply-inbox) that the kernel itself drains (`src/ipc/nonos_inbox/registry.rs`, `const KERNEL_OWNER`).

The envelope is `IpcMessage`: sender name, destination name, payload, a millisecond timestamp, a correlation word and a 64 bit checksum (`src/ipc/nonos_channel/message.rs`, `struct IpcMessage`). The kernel writes the sender as `proc.<caller pid>` when `kernel_route_ipc_corr` builds the envelope with `IpcMessage::new`, so a capsule cannot choose its own sender name (`src/ipc/kernel_ipc.rs`, `fn kernel_route_ipc_corr`). The checksum is a keyed BLAKE3 hash over both names, the timestamp and the payload, cut to its last eight bytes in `compute_checksum` (`src/ipc/nonos_channel/hash.rs`, `fn compute_checksum`). Its key comes from `init_ipc_secret`, which derives it with BLAKE3 from 32 bytes of `get_bytes_secure` at boot (`src/ipc/nonos_channel/hash.rs`, `fn init_ipc_secret`). The boot [stops](../overview/glossary-amnesic-boot.md#boot-stop) if `init_ipc_secret` reports a failure (`src/kernel_core/init/entry/init_core_services.rs`, `fn init_core_services`), but `get_bytes_secure` always reports success (`src/crypto/random_api/basic.rs`, `fn get_bytes_secure`). The key is only as good as the kernel's random source; [Randomness and cryptography](../security/randomness-and-cryptography.md#when-a-source-is-missing) says what that source is on a machine with no hardware generator.

## The IPC system calls

All eight calls need the `IPC` capability in the caller's token; the table entry for them is `can_ipc` (`src/syscall/contract/cap_table/mk_ipc.rs`, `fn check`). `handle` in the microkernel dispatcher unpacks each call's registers into the handler's arguments (`src/syscall/microkernel/dispatch/ipc.rs`, `fn handle`), and [IPC ABI](../abi/ipc.md) lists them per call.

| Tag | Name | What it does |
|---|---|---|
| `MISD` | `MkIpcSend` | Send `buf[..len]` to an endpoint, by port or by `endpoint.<n>` name. |
| `MIRC` | `MkIpcRecv` | Take the next message from the caller's own inbox (endpoint 0) or from an endpoint it owns. |
| `MIRF` | `MkIpcRecvFrom` | As `MkIpcRecv`, and also write the sender's pid to a user pointer. |
| `MICL` | `MkIpcCall` | Send a request with a fresh [correlation token](../overview/glossary-claim-epoch.md#correlation-token) and wait for the matching reply. |
| `MIRY` | `MkIpcReply` | Answer a caller that is waiting in `MkIpcCall`. |
| `MISP` | `MkIpcSendToPid` | Send straight into another process's `proc.<pid>` inbox. |
| `MSVL` | `MkServiceLookup` | Resolve a service name to its port and the pid that serves it. |
| `MSVR` | `MkServiceRegister` | Claim a service name and port at run time. |

The four letter tags are [syscall tags](../overview/glossary-stark-proof.md#syscall-tag); [System calls](syscalls.md) explains them.

## Limits

- A payload is at most `MAX_MESSAGE_SIZE`, 1 MiB (`src/ipc/nonos_channel/limits.rs`, `const MAX_MESSAGE_SIZE`).
- `send_with_correlation` refuses a length of 0 or above `MAX_MESSAGE_SIZE` with `EINVAL` before it allocates anything (`src/syscall/microkernel/ipc/send.rs`, `fn send_with_correlation`).
- `sys_ipc_send_to_pid` (`src/syscall/microkernel/ipc/send_to_pid.rs`, `fn sys_ipc_send_to_pid`) and `sys_ipc_reply` (`src/syscall/microkernel/ipc/reply.rs`, `fn sys_ipc_reply`) apply the same `MAX_MESSAGE_SIZE` bound. `sys_ipc_call` checks the response length `resp_len` against `MAX_MESSAGE_SIZE` (`src/syscall/microkernel/ipc/call/sys_ipc_call.rs`, `fn sys_ipc_call`), and its request goes through `send_with_correlation`.
- An inbox holds `DEFAULT_INBOX_CAPACITY` messages, 1024; a capacity set by hand must lie between `MIN_INBOX_CAPACITY` 16 and `MAX_INBOX_CAPACITY` 65536 (`src/ipc/nonos_inbox/registry.rs`, `const MAX_INBOX_CAPACITY`).
- Bytes are budgeted as well as messages: `TOTAL_BYTES_MAX` is 96 MiB for every inbox together, `INBOX_BYTES_MAX` is 16 MiB for one inbox, and `SHARE_BYTES_MAX` is half of that for one sender in one inbox (`src/ipc/nonos_inbox/budget.rs`, `const SHARE_BYTES_MAX`).
- Each queued message is charged its payload, both names and `MESSAGE_OVERHEAD` of 128 bytes (`src/ipc/nonos_inbox/budget.rs`, `const MESSAGE_OVERHEAD`).
- The kernel's own messages, sender 0, count against the inbox and the total but not against a sender share, as `admit` shows (`src/ipc/nonos_inbox/budget.rs`, `fn admit`).
- One service has at most `MAX_PER_SERVICE` 64 calls waiting for its replies, and one caller at most `MAX_PER_CALLER` 8 of them (`src/syscall/microkernel/ipc/pending_reply/share.rs`, `const MAX_PER_CALLER`).
- A service name passed to lookup or register is at most `NAME_MAX`, 64 bytes, in both `lookup.rs` (`src/syscall/microkernel/ipc/lookup.rs`, `const NAME_MAX`) and `register.rs` (`src/syscall/microkernel/ipc/register.rs`, `const NAME_MAX`).
- `register_endpoint` refuses a new endpoint once `MAX_SERVICES`, 256, are registered (`src/services/registry.rs`, `fn register_endpoint`), and `sys_service_register` reports that as `ERRNO_NOMEM`, -12 (`src/syscall/microkernel/ipc/register.rs`, `fn sys_service_register`).

A message that does not fit is refused, never cut on the way in: `try_enqueue` checks the count and the byte budget under one lock and hands the message back (`src/ipc/nonos_inbox/inbox.rs`, `fn try_enqueue`).
