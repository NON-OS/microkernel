# capsule_shield

`nonos.shield`, the NOX Shield wallet the phone apps run, as the service the
NONOS wallet window calls. It holds one `Wallet` of the phones' core
([`userland/shield_core`](../shield_core/README.md)) and answers the window
over IPC: open the shield from the account's recovery words, read the pool,
review and send deposits, prove private payments and withdrawals on this
machine, follow them to the chain and settle one from the public account
when no lander does. The window's side is
[`capsule_wallet_nonos`](../capsule_wallet_nonos/README.md); the wire between
them is [`shield_wire`](../shield_wire).

## Microkernel contract

- Service `service:5012:nonos.shield`, reply
  `reply:5013:endpoint.nonos.shield.reply` (`Capsule.mk`). The kernel mirror
  `src/userspace/capsule_shield` embeds the signed bytes and init spawns it
  after the network, with exactly the bits below.
- `CAPSULE_REQUIRED_CAPS = 0x7d`: CoreExec, Network, IPC, Memory, Crypto and
  FileSystem.

  | Bit | Capability | Why |
  |---|---|---|
  | 0x01 | CoreExec | run at all, threads, `MkExit` |
  | 0x04 | Network | every read and send, through `net.socks5` (Nym) or `net.anon` (Anyone) |
  | 0x08 | IPC | the wallet window's calls, and the two network services |
  | 0x10 | Memory | the prover's heap, about a gigabyte while it proves |
  | 0x20 | Crypto | `CryptoRandom` for blindings and proof entropy, `MachineKey` for the store's file key |
  | 0x40 | FileSystem | the sealed note store under `/data/shield` |

- It is a std capsule (`CAPSULE_BUILD_STD = std,panic_abort`), so the prover
  runs its parallel phases on a rayon pool with one worker per online core
  (`src/pool.rs`).
- The feature `nonos-capsule-shield` is in `microkernel-desktop-base`, the set
  every online desktop carries. The airgapped profile takes it out with the
  other network programs (`tools/nix/config.nix`).

## Who may call it

Only the wallet window. Every request's sender is matched against the pids
registered as `app.nonos_wallet`, `app.nonos_wallet.1` and
`app.nonos_wallet.2`; anything else is answered `STATUS_DENIED` and nothing
runs (`src/service.rs`).

## Wire format

A request is a sequence number, an op and a body of newline-separated fields;
a reply is the sequence number, a status and a body of `key=value` lines
(`shield_wire`). Answers are immediate. Long work (open, sync, review,
confirm, quote, send, withdraw, follow, self-settle, take back) starts as one
background job and answers `STATUS_STARTED`; the window polls `OP_RESULT`
each tick and may `OP_CANCEL` a proof. A second job while one runs is
answered `STATUS_BUSY` with the running job's name (`src/jobs.rs`).

| Status | Meaning |
|---|---|
| `0` OK | the answer is in the body |
| `1` STARTED | the job runs; ask `OP_RESULT` |
| `2` BUSY | another job runs |
| `3` REFUSED | `why=` says why, in a sentence the window shows |
| `-22` MALFORMED | the request did not parse, or the op is unknown |
| `-13` DENIED | the sender is not the wallet window |

## Keys and secrets

- The recovery words arrive once, from the keyring through the window, and
  are held in a wrapper that zeroes them when the job ends, whether it ran or
  was refused. The request buffer is zeroed after every reply.
- The store's file key is sealed under a key derived from the machine root
  (`nonos_vault`, `src/guard.rs`), which the TPM gives only to this machine in
  the boot state that sealed it. A store opens only when its derived account
  is the window's.
- Randomness is the kernel's CSPRNG through getrandom's custom backend; a
  short draw is an error, never zeros (`src/random.rs`).

## Network

Every byte goes over the anonymity network the machine has chosen, Nym or
Anyone, through `nonos_route_link` with TLS 1.3 from `nonos_tls`. Direct is
refused, never fallen back to. Landers are reached as their Anyone onion
services (`<56 letters>.anyone`, `shield_core/src/net/pools.rs`); when none
lands a spend, the window offers Settle it myself, with the sentence on what
that links, after a refusal or 30 minutes.

## The prover

`nox_prover` from STARKs, unchanged, at the commit the flake pins. The
capsule ships the periodic cache the prover proves from, made and checked by
the Nix build (`src/periodic.rs`); without it a proof costs about a quarter
more time and half a gigabyte more memory.

## Verification

- `shield_core`'s own tests and pinned production vectors
  (`cargo test --release` in `userland/shield_core`, and its README for the
  vectors).
- `shield_wire`'s tests, and `wallet_proofs` for the window's side.
- `nix build .#capsule-shield` builds it hermetically.
- On a boot: `[SHIELD] capsule spawned` from init, and the window's Shield
  screen opening (docs/TESTING-LOCALLY.md).
