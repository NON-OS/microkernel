# IPC: exit, unused code and tests

Part of [IPC](ipc.md).

## When a capsule exits

A send to a capsule that has exited fails at once, even while its process row still exists: `owner_lives` treats a `Zombie` or `Terminated` process as gone (`src/ipc/nonos_inbox/registry.rs`, `fn owner_lives`).

At teardown `release_pending_replies_for_pid` drops every pending call the process made or was owed (`src/process/exit/teardown.rs`, `fn teardown`). When the process is finalized later, `unregister_for_pid` removes `proc.<pid>`, `stdin.<pid>` and `stdout.<pid>` and zeroes every payload still queued in them (`src/ipc/nonos_inbox/drop_pid.rs`, `fn unregister_for_pid`). The output inbox of a finished child, `stdout.<pid>`, is kept for its parent to drain while `is_retained` says so, and its request and stdin inboxes go (`src/process/exit/finalize.rs`, `fn finalize_teardown`; `drop_pid.rs`, `fn unregister_all_but_stdout`); at most `RETAINED_CAP`, 64, such inboxes are kept (`src/process/exit/postmortem.rs`, `const RETAINED_CAP`). [Processes and capsule spawn](processes-and-spawn.md) covers the rest of exit.

## Code that is present but not used

- `src/ipc/nonos_channel` also holds `bus.rs`, `channel.rs` and `stats.rs`, but its module file declares only `error`, `hash`, `limits` and `message`, so the channel bus is not compiled (`src/ipc/nonos_channel/mod.rs`, its `mod` lines).
- `kernel_route_ipc` and `kernel_check_ipc_permission` have no caller at this commit; the send path calls `kernel_route_ipc_corr` (`src/ipc/kernel_ipc.rs`, `fn kernel_route_ipc_corr`).
- `src/ipc/pipe` is a byte FIFO with a `PIPE_BUF_SIZE` of 65536 bytes and at most `MAX_PIPES`, 1024, pipes (`src/ipc/pipe/types.rs`, `const MAX_PIPES`). Nothing calls `create_pipe` at this commit, so no pipe is ever made (`src/ipc/pipe/mod.rs`, `pub use api::create_pipe`); the only use from outside is `get_fd`, which asks `is_pipe` (`src/process/fd_table.rs`, `fn get_fd`).

## Tests and proofs

The [proof crate](../overview/glossary-peer-list.md#proof-crate) `userland/kernel_proofs` compiles the byte budget (`userland/kernel_proofs/src/inbox_budget/mod.rs`), the pending call shares (`userland/kernel_proofs/src/reply_share/mod.rs`), the held endpoint rule (`userland/kernel_proofs/src/ipc_held_tests/mod.rs`) and the peer list (`userland/kernel_proofs/src/ipc_peers_tests.rs`) straight from the kernel sources. It passed, 388 tests, in the flake check run on this commit. `userland/mechanism_proofs` includes `MAX_MESSAGE_SIZE` from the kernel file (`userland/mechanism_proofs/src/constants/mod.rs`); it passed, 56 tests, in the same run.
