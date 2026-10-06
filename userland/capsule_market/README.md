# capsule_market

## Role

`capsule_market` is the marketplace index and install-readiness service,
`market.index`. It holds the signed catalogue the image ships, checks it
against the operator key, and answers what is listed and whether a release
passes the install gates. It does not fetch or install anything itself. The
handbook page is
[docs/handbook/apps/market-and-store.md](../../docs/handbook/apps/market-and-store.md).

```text
BASELINE (embedded) then /nonos/marketplace/index.bin
    |
    | load_verified: decode, newer serial, trusted operator, signatures
    v
market -- list / get / install-ready --> app_store, init (Linux installs)
```

## Microkernel contract

- Requests arrive with `MkIpcRecv` on `service:4106:market.index`; replies go
  out on `reply:4107:endpoint.4294967303`.
- `CAPSULE_REQUIRED_CAPS = 0x59`: CoreExec, IPC, Memory and FileSystem.
  CoreExec is for `MkGetPid`; FileSystem lets it read
  `/nonos/marketplace/index.bin` through vfs (`src/boot_index.rs`), since vfs
  serves only a holder of FileSystem. Signatures are checked in process by
  `nonos_ed25519`, so it holds no Crypto. It has no driver, MMIO, IRQ, DMA,
  PIO, network, admin, debug or loader authority.
- The kernel mirror is `src/security/market_capsule`. The capsule is turned on
  by `nonos-capsule-market` in `microkernel-desktop-base`, so the standard
  desktop images carry it and the offline desktop does not.

The kernel does not parse marketplace indexes or make package policy.

## Loading the catalogue

At start the capsule takes the catalogue embedded at build time
(`target/market/index.bin`, from `mk/21-market.mk`), then the file at
`/nonos/marketplace/index.bin` if one is there, before it serves anything.
Each goes through `load_verified`: it must decode, its serial must be newer
than the one held, its operator key must be in `TRUSTED_OPERATORS`, and the
operator's signature over the index must verify. Each release's publisher
signature is checked and kept per release; a bad one blocks that release, not
the index. A build with the `offline-verify` feature refuses every signature.

## Interface contract

Magic `0x4E4D4B54`, version 1, a 20-byte header.

| Op | Name | Reply |
|---|---|---|
| 1 | `OP_LOAD_INDEX` | status; `E_INVAL`, `E_STALE` or `E_KEYREJECTED` on refusal |
| 2 | `OP_LIST_APPS` | every listing |
| 3 | `OP_GET_APP` | one listing's metadata |
| 4 | `OP_GET_RELEASE` | one release |
| 5 | `OP_INSTALL_READY` | the verdict and six gates, seven bytes |
| 6 | `OP_HEALTHCHECK` | status |

The six gates are the index signature, the operator's validation, the package
(a URL and both hashes), the publisher signature, the arch (and
`kernel_abi_min` at most 1), and attestation (a trailer hash, or a `linux.`
listing whose proof the machine mints). An empty release id asks for the
first release.

## Privacy and persistence

The accepted index lives only in capsule memory. Nothing is written back.

## Limits

- One operator key; no rotation list beyond `TRUSTED_OPERATORS`.
- Any serial is accepted while nothing has been accepted yet.
- A newer catalogue reaches a machine only by a rebuild or a file at
  `/nonos/marketplace/index.bin`; there is no network fetcher.

## Verification

`userland/market_proofs` drives the index decoder and the request readers with
arbitrary bytes and checks the readiness gates and release selection. `make
nonos-mk-market-smoke` and `make nonos-mk-market-fixtures` build the host smoke
test and its fixtures.
