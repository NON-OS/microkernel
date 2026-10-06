# capsule_attest

## Role

`capsule_attest` is the userland attestation service, `systems.nonos.attest`.
It answers local queries about the running system: what is running and with
which capability mask, each NONOS invariant with a verdict from checking it
against the live process table, and the latest route report from each
anonymity transport. It is a report built from what the kernel answers, not a
cryptographic proof: nothing in a reply is signed.

```text
any local caller net.nym / net.anon
 | |
 | OP_PROOF_* queries | OP_ROUTE_REPORT
 v v
capsule_attest -- MkProcStat (AttestRead) --> kernel process table
 |
 +--> masks, invariant verdicts, route board
```

The kernel's STARK and signature checks are described in
[the STARK layer page](../../docs/handbook/trust/stark.md); this capsule is
not part of them.

## Microkernel contract

- `MkIpcRecvFrom` on port `4444` reads queries with the sender's pid.
- `MkIpcReply` answers the sender.
- `MkProcStat` reads the process table: every pid's spawn name, state and,
 because this capsule holds AttestRead, its capability mask.
- `MkTimeMillis` reads the monotonic clock for `OP_PROOF_BOOT` and the route
 reports' ages.
- `MkYield` when nothing arrived.

## Interface contract

| Op | Value | Purpose |
|---|---|---|
| `OP_HEALTHCHECK` | 0x0001 | liveness ping |
| `OP_PROOF_SUMMARY` | 0x0002 | product name, tagline and version |
| `OP_PROOF_INVARIANTS` | 0x0003 | each invariant's name, claim and mechanism, then a verdict (holds, broken, unchecked) and the counts behind it |
| `OP_PROOF_BOOT` | 0x0004 | monotonic boot ms and a fixed bootloader label |
| `OP_PROOF_CAPSULE_LIST` | 0x0005 | every running process's name and capability mask, from one read of the process table |
| `OP_ROUTE_REPORT` | 0x0006 | a transport posts its route report |
| `OP_PROOF_ROUTE` | 0x0007 | the latest report from each transport, with its age; any caller may ask |

`OP_ROUTE_REPORT` takes who sent it from the kernel, not from the message: the
sender's pid from the IPC layer, then that pid's spawn name and mask from the
process table. The board accepts a report about Nym only from `net.nym` and
about Anyone only from `net.anon`, each holding Network
(`userland/route_proof/src/authorize.rs`). Reports name no relay, gateway or
key.

`OP_PROOF_BOOT` returns the string "NONOS bootloader (hybrid Ed25519 +
ML-DSA-65)" as written in the source; it does not read the loader's verdict.
The kernel's boot attestation record, `MkBootAttest`, is read by About and the
installer, not by this capsule.

## Authority

`Capsule.mk` declares `CAPSULE_REQUIRED_CAPS := 0x80000019`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x01 | CoreExec | `MkExit`, the only way it ends |
| 0x08 | IPC | receive and reply on port 4444 |
| 0x10 | Memory | reply buffers |
| 0x80000000 | AttestRead | see every process's mask through `MkProcStat`, which shows other processes' masks only to a holder of AttestRead or ProcessControl |

`Debug` is absent on purpose: the capsule asserts the NO LOGS invariant and has
no log surface. It holds no Network.

## Runtime lifecycle

1. `_start` initializes the heap.
2. The server enters `run()` on port `4444` with an empty route board.
3. Each loop iteration:
 - `mk_ipc_recv_from`; yield and retry when nothing arrived or the sender
 is pid 0.
 - Route the request with `handlers::route`, passing the sender's pid and
 the board.
 - Reply to the sender.

## Failure model

- Unknown op: `E_BAD_OP` (-38).
- Bad magic, version or length: `E_BAD_MAGIC`, `E_BAD_VERSION`, `E_BAD_LEN`.
- A reply that would not fit: `E_INVAL`, no partial reply.
- The kernel refusing the process table: `E_INVAL` for the capsule list,
 `V_UNCHECKED` verdicts for the invariants. Never an empty list or a pass.
- A route report from anyone but the right transport: `E_PERM`.
- Heap init failure at boot: exit `1`.

## Files

| Concern | File |
|---|---|
| Entry and heap init | `main.rs` |
| Wire protocol | `protocol/*.rs` |
| Invariant table and probes | `state/invariants/` |
| Product identity | `state/product.rs` |
| Process table read and sender identity | `state/live/` |
| Server loop | `server/runner.rs` |
| Reply builder | `server/respond.rs` |
| Router | `server/handlers/router.rs` |
| Per-op handlers | `server/handlers/{health,proof_summary,proof_invariants,proof_verdict,proof_boot,proof_capsule_list,route_report,proof_route}.rs` |

## Wire format

A 20-byte header (magic `0x41545354`, `'ATST'`, version 1), then the typed
payload. Replies carry a status word after the header. Per-op payload layouts
are in the handler files.

## State

The invariant table and the product identity are compile-time tables. The
capsule list and the verdicts come from a fresh process table read per query.
The route board holds the latest report from each transport, in memory only.
Nothing is written to a file.

## Build and image

`nonos-capsule-attest` is in `microkernel-desktop-offline`, so every desktop
image carries it, and in the two input end-to-end profiles. Init spawns it
with the desktop services, and the supervisor restarts it if it ends. The
kernel mirror is `src/userspace/capsule_attest/`. The certificate, manifest
and v4 trailer are committed under `nonos-data/trust/capsules/attest.*`.

## Checks

`userland/route_proof_proofs` runs the route report, board and authorization
code, and `userland/service_header_proofs` the request header decode. No
booted run of this capsule is recorded yet.

## What it does not do

- No signature on any reply. A remote party cannot tell a reply from this
 capsule from a forged one.
- No remote transport: it answers local callers only.
- No TPM binding: the replies are not quoted.
- Several invariants are not runtime properties; their verdict is always
 `V_UNCHECKED`.
