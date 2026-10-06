# capsule_socks5_proofs

Host test crate for the SOCKS5 to mixnet proxy in `userland/capsule_socks5`. It compiles the
capsule's pure modules through `#[path]` and drives them with hand-built bytes. The Cargo
package name is `socks5_proofs` (`Cargo.toml`); it depends on `spin` and `nonos_policy_proto`.

## What is under test

`src/lib.rs` mounts these capsule files:

- `../../capsule_socks5/src/wire/mod.rs` (`wire`): RFC 1928 greeting, request and reply codec.
- `../../capsule_socks5/src/conn/mod.rs` (`conn`): the per-client handshake state machine.
- `../../capsule_socks5/src/server/request.rs`, `server/reply.rs`, `server/inbox.rs`,
  `server/kept.rs`: request markers, reply markers, in-order delivery of mixnet chunks, and
  the store that repeats a numbered answer.
- `../../capsule_socks5/src/manager/mod.rs` (`manager`): the connection id table.
- `../../capsule_socks5/src/tunnel/mod.rs` (`tunnel`): Nym Socks5Request framing.
- `../../capsule_socks5/src/nym/watch.rs` (`watch`): the exit silence and rotation record.
- `../../capsule_socks5/src/nym/batch.rs` (`batch`): reading net.nym's batched answer back
  into whole messages.
- `../../capsule_socks5/src/server/gather.rs` (`gather`): filling one answer with everything
  that has come back for a stream.
- `../../nonos_route_link/src/frame.rs`, `answer.rs` and `pick.rs` (`route_frame`,
  `route_answer`, `route_pick`): the terminal's side of the conversation with this proxy, which
  is the shared route client, and the rule for which network it leaves by.
  `nonos_policy_proto` is a dependency for the route constants.

`src/kept_harness.rs` is a test helper that drives `request` and `kept` the way
`server/run.rs` does.

## What the tests check

- `wire_tests.rs`: no-auth detection, IPv4, IPv6 and domain CONNECT parsing with
  network-order ports, truncated input reported as incomplete
  (`a_truncated_request_is_incomplete_not_an_error`).
- `conn_tests.rs`: greeting, CONNECT, failed open and unsupported command paths, and split or
  pipelined input (`connect_opens_a_tunnel_then_replies_and_relays`).
- `inbox_tests.rs`: out-of-order chunks are held until the gap fills, connections are kept
  apart, duplicates are dropped (`a_chunk_that_arrives_early_waits_for_the_gap`), the inbox is
  bounded per stream and in all, and a gap that outlasts a resend is reported
  (`bytes_stuck_behind_a_gap_are_reported_once_the_gap_outlasts_a_resend`).
- `batch_tests.rs`: several messages per answer, a message split across answers, and pieces
  that cannot be placed dropped rather than read as messages.
- `terminal_tests.rs`: the terminal's frames are read as numbered stream bytes, a lost answer is
  given again without the bytes being carried twice, and the terminal leaves only by the chosen
  network: Direct only when chosen, Anyone through net.anon, and an unreadable policy store as
  the mixnet, never direct (`an_unreadable_store_is_the_mixnet_and_never_direct`).
- `gather_tests.rs`: a 100 KB flight reordered by the mixnet comes back whole in a handful of
  polls (`a_hundred_kilobyte_flight_comes_back_whole_in_a_handful_of_polls`), an answer waits
  only when it has nothing, and another stream's traffic cannot hold a poll back.
- `reply_tests.rs`, `kept_tests.rs`: an empty answer is still a one-byte message, and a lost
  numbered answer is given again (`a_lost_answer_is_given_again_and_the_next_number_reads_on`).
- `manager_tests.rs`: nonzero, non-repeating ids, slot reuse, a full table refusing a client.
- `tunnel_tests.rs`: host:port rendering for each address type inside the provider envelope.
- `watch_tests.rs`: an exit that has delivered nothing rotates after its silence budget; one
  that has delivered, or was configured, never does (`a_configured_exit_is_never_rotated_away`).

## Running

```sh
cd userland/capsule_socks5_proofs
cargo test --release
```

At the time of writing this runs 107 tests. CI runs it: the "Mixnet proxy proofs" and "Mixnet
proxy clippy" steps in `.github/workflows/verify.yml` run `cargo test --release` and
`cargo clippy --release -- -D warnings` in this directory.

## Not covered

The IPC calls to `net.nym` (`src/ipc/`), session open and send/receive (`src/nym/session.rs`,
`send.rs`, `recv.rs`), exit discovery and bootstrap, `server/run.rs`, `server/relay.rs` and
the setup wait in `src/main.rs` make syscalls and are not mounted here. The loop relay.rs runs
for each answer is `server/gather.rs`, which is.
