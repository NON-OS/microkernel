# capsule_keyring

## Role

`capsule_keyring` is the in-memory key store. It keeps key records for the
capsules that own them, holds the wallet's Ethereum and NOX keys, signs
with them, and seals the wallet's account into a vault record that survives
a reboot. The handbook pages are
[System apps and services](../../docs/handbook/apps/system-apps.md) and
[Wallet](../../docs/handbook/apps/wallet.md).

```text
client capsule (wallet, login, ...)
    |
    | keyring IPC, service:4098:keyring
    v
keyring -- in-memory store, 128 keys, 16 per owner
    |
    `-- reply / error
```

## Microkernel contract

- `MkIpcRecv` receives requests on `service:4098:keyring`; replies go to
  `reply:4099:endpoint.4294967298`.
- `CryptoRandom` draws generated keys and nonces.
- `CryptoMachineKey` gives the machine root the vault derives each record's
  key from.
- `MkServiceLookup` names the wallet's pids for the vault gate.
- `MkPidAlive` finds the keys of owners that ended.
- `MkExit` terminates on fatal setup failure.
- The kernel mirror is `src/security/keyring_capsule`.

secp256k1 signing and public keys come from the `nonos_secp256k1` library in
this process (`src/server/secp.rs`), not from a kernel call. The kernel holds
no user keys and no keyring policy.

## Interface contract

24 ops, all listed in `ALL` in `src/protocol/ops.rs`, where a const assertion
fails the build if two share a code:

| Ops | Purpose |
|---|---|
| `OP_STORE`, `OP_RETRIEVE`, `OP_DELETE`, `OP_METADATA`, `OP_COUNT` | caller-owned key records |
| `OP_LOCK`, `OP_UNLOCK` | lock or release one entry the caller owns |
| `OP_WALLET_IMPORT`, `OP_WALLET_GENERATE`, `OP_WALLET_GENERATE_HD`, `OP_WALLET_RECOVER` | create a wallet key: from a 32-byte secret, at random, or from a BIP39 mnemonic (m/44'/60'/0'/0/0) |
| `OP_WALLET_ADDRESS`, `OP_WALLET_EXPORT` | the Ethereum address; the raw key, to its owner only |
| `OP_SIGN_NOX_RECEIPT` | a recoverable EIP-712 signature |
| `OP_SIGN_NOX_APPROVE`, `OP_SIGN_NOX_TRANSFER`, `OP_SIGN_NOX_STAKE_APPROVE`, `OP_SIGN_NOX_STAKE`, `OP_SIGN_NOX_UNSTAKE`, `OP_SIGN_NOX_STAKE_LOCKED`, `OP_SIGN_ETH_TRANSFER` | raw signed EIP-1559 transactions |
| `OP_LIST_WALLET_RAILS` | the wallet rails and their status |
| `OP_VAULT_SEAL`, `OP_VAULT_OPEN` | seal and open the wallet's account record; the wallet only |

Every op that names an owner goes through `resolve_caller`: the pid in the
request is accepted only when it equals the pid the kernel stamped on the
message, and sender 0 is refused. A frame too short to carry a sequence
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

Enabled rails are ETH and NOX. PR is config-required. SAL is a separate
Salvium wallet track and is reported reserved until the native Salvium wallet
core capsule is ported and tested.

## Authority

The manifest grants `IPC`, `Memory` and `Crypto`
(`CAPSULE_REQUIRED_CAPS = 0x38`). Crypto is for `CryptoRandom` and
`CryptoMachineKey`. It has no device, MMIO, IRQ, DMA, PIO, filesystem,
network, admin, or debug authority. The keyring is the authority for the
Keyring bit, so it does not hold it; callers do.

## Privacy and persistence

Key records live only in capsule memory and are gone at reboot. The one
exception is the wallet's account: `OP_VAULT_SEAL` returns a blob sealed by
`nonos_vault` under a key derived per record from the machine root, which the
wallet keeps itself. Only the wallet may seal or open it: `may_use_vault`
accepts a sender only when it owns one of `app.nonos_wallet`,
`app.nonos_wallet.1` or `app.nonos_wallet.2` (`src/server/vault_gate/rule.rs`).
After each request the receive buffer is wiped.

## Operating rules

- Hold one owner to `MAX_KEYS_PER_OWNER` keys of `MAX_KEYS`, so no program can refuse every
  other program a key.
- Drop the keys of an owner that ended, each wiped as its own delete would have: a key answers
  only the pid that stored it, so nobody can use it once that pid is gone. Looked for every two
  seconds while requests arrive, and before any request when the keyring is full
  (`src/server/reap.rs`, `src/store/ended.rs`).
- `unlock` and its locking twin act only on an entry the caller owns.
- Never mirror keys into kernel service state.

## Explicit non-goals today

No hardware secure element, remote sync or password UI. The vault blob is
stored by the wallet, not here.

## Verification

- Build: `make -B nonos-mk-keyring`
- Host proofs: `userland/wallet_proofs` (`keyring_owner_tests.rs` runs the real store: the
  per-owner share and the dropping of ended owners' keys; the vault gate rule is held there
  too).
