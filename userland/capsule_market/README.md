# capsule_market

## Role

`capsule_market` is the marketplace index and install-readiness service,
`market.index`. it holds the signed catalogue the image ships, checks it
against the committed operator key, and answers what is listed and whether a
release passes the install gates. it does not fetch, download or install
anything itself; the verdict it returns is advisory to the store and the
Linux installer. the handbook page is
[docs/handbook/apps/market-and-store.md](../../docs/handbook/apps/market-and-store.md).

```text
built-in baseline (target/market/index.bin), then /nonos/marketplace/index.bin
    |
    | load_verified: decode, serial newer, trusted operator, ed25519 signatures
    v
market.index -- list / get_app / get_release / install_ready --> store, installer
    |
    `-- MkIpcSend reply to the kernel reply endpoint
```

## Microkernel contract

- requests arrive with `MkIpcRecv` (`mk_ipc_recv`) on
  `service:4106:market.index`; replies go out with `MkIpcSend`
  (`mk_ipc_send`) to the kernel reply endpoint `0x1_0000_0007`, declared as
  `reply:4107:endpoint.4294967303`.
- `MkGetPid` (`mk_getpid`) names the caller the capsule passes to the vfs
  client when it reads `/nonos/marketplace/index.bin`.
- `MkDebug` (`mk_debug`) writes one `[MARKET]` line at boot when a catalogue
  is absent or refused. `MkExit` (`mk_exit`) ends the process if the heap
  cannot be set up.
- the feature is `nonos-capsule-market`, in `microkernel-desktop-base`, so the
  standard desktop images carry it. the kernel mirror is
  `src/security/market_capsule`.

the kernel does not parse marketplace indexes or make package policy.

## Interface contract

the op numbers come from `nonos_market_proto`, so the market, the store and
the Terminal cannot disagree on them.

| op | name | reply |
|---|---|---|
| 1 | `OP_LOAD_INDEX` | status; `E_INVAL`, `E_STALE` or `E_KEYREJECTED` on refusal |
| 2 | `OP_LIST_APPS` | every listing |
| 3 | `OP_GET_APP` | one listing's metadata |
| 4 | `OP_GET_RELEASE` | one release, or the first when the release id is empty |
| 5 | `OP_INSTALL_READY` | the verdict and six gates, seven bytes |
| 6 | `OP_HEALTHCHECK` | status |

a request whose declared `payload_len` runs past the received bytes is
refused with `E_MSGSIZE` before any handler runs; an unknown op gets
`E_INVAL`.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x59` is the whole authority: CoreExec (`0x01`),
IPC (`0x08`), Memory (`0x10`) and FileSystem (`0x40`). CoreExec is for
`MkGetPid`; FileSystem lets it read `/nonos/marketplace/index.bin` through
vfs, which serves only a holder of FileSystem. signatures are checked in
process by `nonos_ed25519` (`src/verify/crypto.rs`), not by a syscall, so it
holds no Crypto. it has no driver, MMIO, IRQ, DMA, PIO, network, admin, debug
or loader authority; the kernel installs the mask from the verified manifest
and the capsule cannot widen it.

## Privacy and persistence

the accepted index lives only in capsule memory. nothing is written back to
disk, and the capsule keeps no per-caller state. the committed operator public
key is public by design.

## Runtime lifecycle

1. `_start` sets up the heap, then `boot_index::load` runs before the first
   query arrives so a client never sees an empty catalogue on a machine that
   has one.
2. the built-in baseline (`include_bytes!` of `target/market/index.bin`,
   written by `tools/nonos-market-index`) is taken first; an empty baseline
   reads as "no catalogue", not a failure.
3. the operator file at `/nonos/marketplace/index.bin`, if present, is taken
   next through the vfs client, so a newer signed catalogue supersedes the
   built-in one.
4. `server::run` then loops on `MkIpcRecv` and answers each op.

## Failure model

- each catalogue goes through `load_verified` (`src/ingest`): `decode_index`,
  then the serial must be newer than the one held (any serial is accepted
  while `last_serial` is 0), then the operator key must be in
  `TRUSTED_OPERATORS`, then the operator's ed25519 signature over the signed
  bytes must verify. a bad publisher signature blocks that release only, not
  the index.
- at boot a stale serial is silent (it is the ordinary "no operator has
  published since this image" case); `Malformed`, `UntrustedOperator` and
  `SignatureRefused` each write a `[MARKET]` line saying why.
- on `OP_LOAD_INDEX` the same errors map to `E_INVAL` (`-22`), `E_STALE`
  (`-116`) and `E_KEYREJECTED` (`-129`, for both an untrusted operator and a
  refused signature). `OP_INSTALL_READY` with no accepted index, or a listing
  or release that is not found, returns `E_NODATA` (`-61`).
- a build with the `offline-verify` feature swaps `CryptoVerifier` for
  `RejectAll`, which refuses every signature, so no catalogue is ever
  accepted.

## Current implemented surface

the six ops above, the two-stage boot load (`src/boot_index.rs`), the trust
check (`src/bootstrap_trust`), the verifier (`src/verify`) and the readiness
evaluator (`src/install_ready`). nothing else: no fetcher, no installer, no
writeback path.

## Wire format

magic `0x4E4D_4B54`, version 1, a 20-byte little-endian header: magic `u32`,
version `u16`, op `u16`, flags `u16`, two pad bytes, `request_id` `u32`,
`payload_len` `u32`. a reply reuses the header, then a 4-byte `i32` status,
then the body. `OP_INSTALL_READY` returns a 7-byte body: `install_ready`, then
the index signature, package (a URL and both hashes), publisher signature,
validation, arch (and `kernel_abi_min` at most 1) and attestation gates, one
byte each. attestation passes on a `zk_trailer_hash`, or on a `linux.` listing
whose arch includes `x86_64-linux`, whose proof the machine mints after the
installer authenticates the bytes.

## State ownership

one `Store` in the capsule heap: the current accepted `MarketplaceIndex`, the
per-release publisher-verified flags, and `last_serial`. it is replaced whole
when a newer catalogue is accepted and is never persisted. no file handles, no
sessions, no shared memory.

## Operating rules

- one operator key. there is no rotation list beyond `TRUSTED_OPERATORS`
  (`src/bootstrap_trust/keys.rs`), which holds `NOX_OPERATOR_V1`, the 32-byte
  public key embedded at build from the committed
  `.keys/marketplace_operator_ed25519.pub`.
- any serial is accepted while nothing has been accepted yet; after that only
  a strictly newer serial is taken.
- a newer catalogue reaches a machine only by a rebuild or by a file at
  `/nonos/marketplace/index.bin`. there is no network fetcher.
- readiness is reported, not enforced: the store and the Linux installer act
  on the verdict.

## Release target

0.9.2.

## Release evidence

`userland/market_proofs` drives the index decoder and the request readers with
arbitrary bytes and checks the readiness gates and release selection. `make
nonos-mk-market-smoke` and `make nonos-mk-market-fixtures` build the host smoke
test and its fixtures.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x59`.
- [ ] `TRUSTED_OPERATORS` carries only the committed operator public key and
      the build embeds `.keys/marketplace_operator_ed25519.pub`.
- [ ] the six ops and the readiness gates pass `userland/market_proofs`.
- [ ] no network fetcher and no writeback path have been added.

## Explicit non-goals today

the capsule lists and verifies; it does not fetch, download or install, and it
holds no per-caller state. it runs no key-rotation list, speaks no network,
and its readiness verdict is advisory rather than an install authorization. it
does not re-check a package's bytes; that is the installer's job once it has
the file.

## Verification

`src/ingest/load/load_verified.rs` is the trust gate end to end: decode,
serial, trusted operator, operator signature, then per-release publisher
signatures. `src/install_ready/checks.rs` fixes the readiness flags and
`src/server/handlers/load_index.rs` fixes the errno mapping. every number,
op, endpoint and errno above is set in those files and in `nonos_market_proto`,
and `userland/market_proofs` exercises them on the host.
