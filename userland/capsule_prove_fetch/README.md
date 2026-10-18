# capsule_prove_fetch

`capsule_prove_fetch` is `nonos.prove.fetch`. It carries kind-1
proof containers between `app.store` or `market.index` and the proof relay,
over Nym or Anyone and never directly. It never holds `DeviceSecret` and
never sees a witness: what it moves is a finished container, checked by its
anchor on the way out and on the way back. It is `no_std`.

## Role

```text
 app.store ──PUT/GET/STATUS──┐
 market.index ──GET/STATUS───┤  port 5202, one frame, one answer
                             v
                    nonos.prove.fetch ── the desk (src/desk/, pure)
                             │
              nonos_route_link: the route the person chose
                    (Nym or Anyone; Direct is refused)
                             │ TLS, the chain checked (nonos_tls)
                             v
                 the pinned relay: PUT or GET /v1/proof/<anchor hex>
```

- **Who may ask.** The caller is named by the kernel's service registry:
  the pid that owns `app.store` or `market.index` (`src/callers.rs`). A
  PUT is `app.store`'s alone; GET and STATUS are either's. Anyone else is
  `EPERM` before its body is read (`src/desk/answer.rs`).
- **One transfer at a time.** A transfer runs from its first PUT chunk or
  its first GET to Held or Failed; one under way makes every other `EBUSY`
  (`src/desk/transfer.rs`). A PUT left quiet for a minute is dropped
  (`src/serve.rs`).
- **Uploads.** Chunks come in order, from one caller, for one listing and
  total. The last one makes the anchor; bytes that are not a container are
  `NOT_CONTAINER` and nothing is sent (`src/desk/put.rs`).
- **Downloads.** A download is kept only when its bytes are a container
  whose anchor, under the listing asked for, is the anchor asked for; else
  it is `ANCHOR` and nothing of it is kept (`src/desk/finish.rs`). A held
  container is answered from memory, 64 KiB at a time (`src/desk/held.rs`).
- **The relay.** A content-addressed store keyed by anchor and trusted for
  nothing (`src/desk/http.rs`). Its answer is one `Content-Length`, at most
  1 MiB; chunked, unsized or longer is refused (`src/desk/reply.rs`,
  `src/desk/head.rs`). No relay is pinned yet (`RELAY` is `None` in
  `src/desk/relay.rs`), so every op answers `NO_RELAY` and nothing leaves
  the machine.
- **Budgets.** A stream opens within 45 s; a read waits 60 s on Nym and
  30 s on Anyone; one op takes at most 180 s over at most 6 connections
  (`src/desk/relay.rs`). Frames that come in meanwhile are answered between
  reads and writes, so STATUS is answered while a transfer runs
  (`src/io.rs`, `src/carry.rs`).

## IPC contract

The endpoint is `service:5202:nonos.prove.fetch`. Requests arrive with
`MkIpcRecvFrom`, which names the sending pid, and each is answered with
`MkIpcReply`. The frame is `nonos_proof_abi::wire::fetch` (`FETCH_MAGIC`):
PUT chunks, GET by anchor and STATUS, each answered with the transfer's
stage or a negative errno (`src/desk/errno.rs`).

## Capabilities

From `Capsule.mk`:

```make
CAPSULE_REQUIRED_CAPS := 0x3D
```

CoreExec, Network (the route to the relay), IPC (its endpoint, and
`net.socks5` or `net.anon`), Memory (a container in flight, at most 1 MiB)
and Crypto (`CryptoRandom` for TLS). The same mask is declared in the
binary's `.nonos.caps` section (`src/caps.rs`). Debug is optional and
granted only by a `capsule-serial-debug` build. No `DeviceSecret` and no
`FileSystem`.

## Files

- `src/main.rs`: the entry; the heap, then the endpoint.
- `src/serve.rs`: the receive loop and the quiet-PUT rule.
- `src/callers.rs`: the caller by its registered service.
- `src/carry.rs`, `src/exchange.rs`, `src/io.rs`: one transfer over the
  chosen route, one TLS connection, and the stream as TLS reads it.
- `src/desk/`: the pure half, which `userland/proof_abi_proofs` runs on
  the host as it ships.
