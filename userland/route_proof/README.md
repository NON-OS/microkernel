# nonos_route_proof

`nonos_route_proof` is what an anonymity transport reports about its route, who
may report it, and what an observer may conclude from it. `net.nym` and
`net.anon` build and post a report, the attest service (`capsule_attest`)
checks who sent it and keeps it on a board, and About reads the board and
shows a verdict. The crate is pure, `no_std` and has no dependencies, so every
rule is proven on the host.

## The report

`RouteReport` (`src/report.rs`) is a fixed 40 byte record, version 1: the
network (Nym or Anyone), the `Stage` (nothing fetched, directory in progress,
directory accepted, carrying traffic, stopped), the directory signatures
verified and required, the relays or mix nodes the directory names, how long
it stays valid, the open routes, and how many hops of the open route proved who
they are. It names no guard, gateway, relay, address or key: any local caller
can read the board, and a guard's identity is what a guard discovery attack is
after.

Each transport fills it from its own state through `anyone_report` or
`nym_report` (`src/facts.rs`), so the mapping is the one the verdict is proven
against.

## Who may report

`may_report` (`src/authorize.rs`) accepts a report about Nym only from the
process the kernel spawned as `net.nym`, and about Anyone only from `net.anon`,
each holding Network. The name and capabilities come from the kernel's process
table, not from the message. `Board::post` (`src/board.rs`) applies it, takes a
report only when it decodes exactly, and keeps the latest per network with the
board's own arrival time.

## The frames

`src/frame.rs` builds the attest service's frames: `post_frame` for
`OP_ROUTE_REPORT` (6) and `ask_frame` for `OP_PROOF_ROUTE` (7), to the service
named `attest`. `reply_body` reads a reply only when its op, request id and
lengths agree. The answer carries each network's latest report with its age on
the board's clock, so a reader judges freshness without trusting the sender's
clock.

## The verdict

`route_verdict` (`src/verdict.rs`) concludes `Anonymous` only from a report
younger than `STALE_AFTER_MS`, 30 s, by the transport for the route the system
is set to, with the route up, a directory signed by a quorum and still valid,
and every hop authenticated. Set to Direct, the verdict is `Exposed`. Set to an
anonymity network that is not carrying traffic, it is `NotEstablished` with the
reason (`Stale`), never `Exposed`, since the transports fail closed. A route
that cannot be read is `Unknown`.

## What it does not do

- It proves what the transport says it checked, not what happens on the wire.
  A report is the transport's own account.
- It covers Nym and Anyone. Direct has no transport to report.

## Tests

`userland/route_proof_proofs` runs the encoding, the authorization, the board,
the frames against the attest service's own decoder, and the verdict for every
combination of route and report. `attest_doc_proofs` uses the crate too.

See [the Nym mixnet capsule](../../docs/handbook/network/nym.md) and
[the Anyone onion routing capsule](../../docs/handbook/network/anyone.md).
