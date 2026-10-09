# capsule_keyring

## Role

`capsule_keyring` is the in-memory key store: it keeps key records for the
capsules that own them, holds the wallet's Ethereum and NOX keys, signs with
them, and seals the wallet's account into a vault record that survives a reboot.
It ships and is spawned at init; it is not parked. The capsule catalog is
[docs/handbook/apps/capsule-catalog.md](../../docs/handbook/apps/capsule-catalog.md),
and the wallet page is
[docs/handbook/apps/wallet.md](../../docs/handbook/apps/wallet.md).

```text
client capsule (wallet, login, ...)
    |
    | keyring IPC, service:4098:keyring
    v
keyring -- resolve_caller --> in-memory store, 128 keys, 16 per owner
    |                             |
    |                             `-- CryptoMachineKey -> nonos_vault seal
    `-- reply on reply:4099:endpoint.4294967298
```

## Microkernel contract

- `CAPSULE_REQUIRED_CAPS = 0x38`: IPC (`0x8`), Memory (`0x10`) and Crypto
  (`0x20`). The kernel mirror (`src/security/keyring_capsule`) requests the same
  three bits. There is no optional mask. The capsule is the authority for the
  Keyring bit, so it does not hold `Capability::Keyring`; callers do.
- Service `service:4098:keyring`, reply `reply:4099:endpoint.4294967298`.
- The feature `nonos-capsule-keyring` is in `microkernel-keyring`,
  `microkernel-terminal-only` and `microkernel-desktop-offline` (and the desktop
  sets built on it). Init spawns it first in `spawn_after_ramfs`, ahead of
  entropy and crypto (`src/userspace/init/spawn_plan/core.rs`).
- Crypto (`0x20`) is for the two kernel crypto syscalls the capsule drives:
  `crypto_random` (`CryptoRandom`) for generated keys and nonces, and
  `machine_key` (`CryptoMachineKey`) for the machine root the vault derives each
  record's key from (`src/vault/root.rs`). secp256k1 signing and public keys are
  computed in process by `nonos_secp256k1` (`src/server/secp.rs`), not by a
  kernel call. The kernel holds no user keys and no keyring policy.

## Interface contract

29 ops, all listed in `ALL` in `src/protocol/ops.rs`, where a const assertion
(`all_distinct`) fails the build if two share a code. They are served by
`src/server/dispatch.rs`:

| Ops | Purpose |
|---|---|
| `OP_STORE`, `OP_RETRIEVE`, `OP_DELETE`, `OP_METADATA`, `OP_COUNT` | caller-owned key records |
| `OP_LOCK`, `OP_UNLOCK` | lock or release one entry the caller owns |
| `OP_WALLET_IMPORT`, `OP_WALLET_GENERATE`, `OP_WALLET_GENERATE_HD`, `OP_WALLET_RECOVER`, `OP_WALLET_DERIVE` | create or derive a wallet key: from a 32-byte secret, at random, or from a BIP39 mnemonic (m/44'/60'/0'/0/0) |
| `OP_WALLET_ADDRESS`, `OP_WALLET_EXPORT` | the Ethereum address; the raw key, to its owner only |
| `OP_SIGN_NOX_RECEIPT` | a recoverable EIP-712 signature |
| `OP_SIGN_NOX_APPROVE`, `OP_SIGN_NOX_TRANSFER`, `OP_SIGN_NOX_STAKE_APPROVE`, `OP_SIGN_NOX_STAKE`, `OP_SIGN_NOX_UNSTAKE`, `OP_SIGN_NOX_STAKE_LOCKED`, `OP_SIGN_ETH_TRANSFER`, `OP_SIGN_TX` | raw signed EIP-1559 transactions |
| `OP_LIST_WALLET_RAILS` | the wallet rails and their status |
| `OP_VAULT_SEAL`, `OP_VAULT_OPEN`, `OP_SHIELD_MATERIAL`, `OP_SHIELD_SEAL`, `OP_SHIELD_OPEN` | seal, open and shield the wallet's account record; the wallet only |

Every op that names an owner goes through `resolve_caller` (`src/server/caller.rs`):
the pid in the request is accepted only when it equals the pid the kernel stamped
on the message, and sender 0 is refused. A frame too short to carry a sequence
number is answered with `EINVAL` under sequence 0.

`OP_LIST_WALLET_RAILS` returns:

```text
u32 count
repeat count:
  u8  symbol_len
  u8  family
  u16 status
  u32 flags
  u64 chain_id
  u8  contract_address[20]
  u8  symbol[symbol_len]
```

Enabled rails are ETH and NOX. PR is config-required. SAL is a separate Salvium
track and is reported reserved until the native Salvium wallet core capsule is
ported and tested.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x38` is the whole authority the capsule asks for: IPC
for `mk_ipc_recv_from`/`mk_ipc_send`, Memory for the heap, and Crypto for
`CryptoRandom` and `CryptoMachineKey`. It holds no device, MMIO, IRQ, DMA, PIO,
filesystem, network, admin or debug capability, and no Keyring bit. The kernel
installs the mask from the verified manifest at spawn; the capsule cannot widen
it.

## Privacy and persistence

Key records live only in capsule memory and are gone at reboot. The one
exception is the wallet's account: `OP_VAULT_SEAL` returns a blob sealed by
`nonos_vault` under a key derived per record from the machine root, which the
wallet keeps itself. Only the wallet may seal or open it: `may_use_vault`
accepts a sender only when it owns one of `app.nonos_wallet`,
`app.nonos_wallet.1` or `app.nonos_wallet.2` (`src/server/vault_gate/rule.rs`).
After each request the receive buffer is wiped (`src/server/wipe.rs`,
`src/server/zeroize.rs`).

## Runtime lifecycle

Spawned once at init through the verified path, first of the after-ramfs
capsules. It registers `service:4098:keyring`, looks up the wallet pids with
`MkServiceLookup` for the vault gate, then loops: `MkIpcRecvFrom` for a request
with its sender pid, dispatch, `MkIpcSend` for the reply, `MkYield` between
turns. A reaper (`src/server/reap.rs`) runs every two seconds while requests
arrive, and before any request when the store is full, using `MkPidAlive` to
drop the keys of owners that ended. On a fatal setup failure it calls `MkExit`.

## Failure model

Oversized or malformed frames, an unknown op, an unauthorized caller or a
signing or vault backend failure return explicit protocol errors
(`src/protocol/errno.rs`). A key answers only the pid that stored it; a request
from any other pid is refused rather than served. A dead or restarted capsule is
reported to clients with an errno of its own by the kernel path.

## Current implemented surface

The 29 ops above, served by the handlers under `src/server/handlers/`, over the
store in `src/store/` (128 keys, 16 per owner, `MAX_KEY_SIZE` 256 bytes). EIP-712
and EIP-1559 encoding live under `src/server/eip712/` and `src/server/eip1559/`;
RLP under `src/server/rlp/`; the vault under `src/vault/`. Nothing else.

## Wire format

Every request carries an 8-byte header: a `u32` sequence number and a `u16` op
(`HDR_LEN = 8`, `src/protocol/types.rs`), then the op's payload. The reply echoes
the sequence number and carries a status with the result bytes. A frame shorter
than the header is answered `EINVAL` under sequence 0. The kernel reply endpoint
is `0x1_0000_0002`, distinct from ramfs, entropy and crypto so concurrent
in-flight requests cannot cross-route.

## State ownership

The capsule owns the whole key store in its own memory: up to `MAX_KEYS` (128)
records, at most `MAX_KEYS_PER_OWNER` (16) per owner. It owns no persistent
state. The sealed vault blob is owned and stored by the wallet, not here. No key
is ever mirrored into kernel service state.

## Operating rules

- Hold one owner to `MAX_KEYS_PER_OWNER` of `MAX_KEYS`, so no program can starve
  every other program of a slot.
- Drop the keys of an owner that ended, each wiped as its own delete would have;
  checked every two seconds while requests arrive, and before any request when
  the store is full (`src/server/reap.rs`, `src/store/ended.rs`).
- `unlock` and its locking twin act only on an entry the caller owns.
- Only the wallet pids may seal or open the vault.
- Never mirror keys into kernel service state; wipe the receive buffer after
  each request.

## Release target

0.9.2.

## Release evidence

`userland/wallet_proofs` (`keyring_owner_tests.rs`) runs the real store on the
host: the per-owner share and the dropping of ended owners' keys, with the vault
gate rule exercised there too. The static gate `nonos-ci/run-static-checks.sh`
runs `scripts/check_mirror_caps.py` over the spawn mirror's capability word.

## Release checklist

- [ ] `CAPSULE_REQUIRED_CAPS` unchanged at `0x38`, Keyring bit absent.
- [ ] `ALL` lists every op and `all_distinct` holds (no shared code).
- [ ] `keyring_owner_tests.rs` passes: per-owner cap, ended-owner reaping and
      the vault gate rule.

## Explicit non-goals today

No hardware secure element, remote sync or password UI. The vault blob is stored
by the wallet, not here. The capsule serves no on-disk key file.

## Verification

- Build: `make -B nonos-mk-keyring`; sign through the shared capsule rules.
- Host proofs: `userland/wallet_proofs`.
- Static gate: `bash nonos-ci/run-static-checks.sh`, including the mirror
  capability check.
