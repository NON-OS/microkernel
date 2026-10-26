# IPC: blocking, timeouts and errors

Part of [IPC](ipc.md).

## Blocking, waking and timeouts

`MkIpcRecv` and `MkIpcRecvFrom` block until a message arrives. A `timeout_ms` of 0 waits forever; any other value ends the wait with `ETIMEDOUT`, -110, once that many milliseconds have passed, as `recv_from_inbox` does (`src/syscall/microkernel/ipc/recv_inbox.rs`, `fn recv_from_inbox`). The receiver reads its `wake_token` before it looks at the queue, so a message that lands between the empty check and the sleep is not lost (`src/syscall/microkernel/ipc/recv_inbox.rs`, `fn recv_from_inbox`).

A message longer than the receive buffer is cut to the buffer and the rest is dropped: the whole message is dequeued and `copy_to_user` copies only what fits (`src/syscall/microkernel/ipc/recv_take.rs`, `fn taken`). Size the buffer for the largest message the service sends.

`MkIpcCall` is a request and its reply in one call:

- `next_call_token` hands every call a nonzero correlation token (`src/syscall/microkernel/ipc/call/token.rs`, `fn next_call_token`).
- The call records a pending entry with the server through `pending_reply::push` and fails with `EBUSY` when the server's queue or the caller's share is full (`src/syscall/microkernel/ipc/call/sys_ipc_call.rs`, `fn sys_ipc_call`).
- A `timeout_ms` of 0 means 5000 ms here, not forever (`src/syscall/microkernel/ipc/call/sys_ipc_call.rs`, `fn sys_ipc_call`).
- `recv_reply_correlated` delivers only a message whose correlation equals the token and discards every other message in the reply inbox (`src/syscall/microkernel/ipc/recv_reply.rs`, `fn recv_reply_correlated`).
- A plain send carries correlation 0, which `sys_ipc_send` sets (`src/syscall/microkernel/ipc/send.rs`, `fn sys_ipc_send`), so it cannot pass as a reply.
- A call that times out leaves its pending entry in place, so a late reply still pairs with the right caller and is then discarded. Apart from the reply itself, only a failed send or `clear_pid`, when either side exits, removes an entry (`src/syscall/microkernel/ipc/call/settle.rs`, `fn settle`).

`MkIpcReply` takes its token from a pending entry of that destination through `pending_reply::take_for_reply`. Only an entry whose request the server has dequeued can be taken: each receive marks the entry of the request it takes (`src/syscall/microkernel/ipc/pending_reply/mark.rs`, `fn mark_served`, called from `recv_take.rs`, `fn taken`, and `recv_from_drain.rs`, `fn drain`). Of the dequeued entries, the one whose token matches the request the server received last wins when that request is the destination's; otherwise the destination's oldest dequeued entry answers, as a server that takes several requests and answers them in order needs. So a request the server never answers does not stamp the caller's later replies with an earlier call's token, and no reply answers a request still in the server's inbox (`src/syscall/microkernel/ipc/pending_reply/take_reply.rs`, `fn take_for_reply`; `reply_rule.rs`, `fn pick`). With no dequeued call from that destination it drops the reply and returns 0 (`src/syscall/microkernel/ipc/reply.rs`, `fn sys_ipc_reply`).

A second direct reply to one request finds no entry and is dropped while the caller's next call is still in the inbox. It is not refused when the same caller has another request the server has already dequeued and not answered: it takes that entry, so a pipelining server that answers one request twice hands the second answer to that caller's other call. A server answers each request once, and a server that takes several requests before it answers names each with `MkIpcRecvReq` and answers it with `MkIpcReplyTo`, which refuses a second answer with `ENOENT` (`src/syscall/microkernel/ipc/reply_to.rs`, `fn sys_ipc_reply_to`). A server can also answer with `MkIpcSend` to its own reply endpoint; `redirect_reply` then calls `pop`, which takes the pending entry whose token matches the request the server received last, not the oldest one, and the bytes go to that caller stamped with that token; a second such reply finds nothing (`src/syscall/microkernel/ipc/send_redirect.rs`, `fn redirect_reply`; `src/syscall/microkernel/ipc/pending_reply/pop.rs`, `fn pop`; `served_rule.rs`, `fn redirect_at`). PENDING is taken in one order only: `scripts/check_pending_order.py` holds that no holder calls out of the table or retakes it, that no marking receive holds a lock, and that every dequeue of a request marks.

## Errors a sender sees

| Errno | Value | When |
|---|---:|---|
| `EINVAL` | -22 | Length 0, length above 1 MiB, or a bad pid or name. |
| `EFAULT` | -14 | The buffer is not readable or writable user memory, or a reply sent with `MkIpcSend` could not be queued for its caller. |
| `EPERM` | -1 | The caller fails a capability gate, a held endpoint or the peer list. |
| `EACCES` | -13 | A receive names an endpoint the caller does not own, or the router's own capability check fails. |
| `ENOENT` | -2 | No endpoint or inbox of that name, or for `MkIpcSendToPid` and `MkIpcReply` a destination that has exited. |
| `ESRCH` | -3 | The inbox is gone or the process that drains it has exited, from the router. |
| `EAGAIN` | -11 | The destination inbox is full, from the router. |
| `EBUSY` | -16 | A full inbox for `MkIpcSendToPid` and `MkIpcReply`, a full pending queue for `MkIpcCall`, or a name or port already taken for `MkServiceRegister`. |
| `ENOMEM` | -12 | The registry is full for `MkServiceRegister`, or the kernel could not build the message. |
| `ETIMEDOUT` | -110 | A receive or a call waited out its timeout. |

The router's constants sit beside `EACCES` in `kernel_ipc.rs` (`src/ipc/kernel_ipc.rs`, `const EACCES`), and the syscall handlers use `ERRNO_PERM` and its neighbours (`src/syscall/microkernel/errnos.rs`, `const ERRNO_PERM`). The full table is on [Errors](../abi/errors.md). `ETIMEDOUT` is returned by these calls but is not listed in the `[errors]` table of [abi/syscalls.toml](../../abi/syscalls.toml).
