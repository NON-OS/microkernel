# capsule_payment

## Role

`capsule_payment` is the meter. It signs cumulative NOX
wallet. Each voucher is signed with the meter's own secp256k1 key, through
the keyring's `OP_SIGN_TYPED_DATA` entry 1. The accounting is
`nonos_nox_meter`'s; this capsule is the I/O around it.

```text
installer / app.store / wallet
        |  OP_PAY v2                     wallet: OP_ACCOUNT, OP_SET_BUDGET
        v
capsule_payment --> market.index   OP_GET_COMMERCE: is the release trusted, what does it sell
                --> nox_settle     OP_CHANNEL_STATE: deposited, settled, expiry, closed
                --> keyring        op 30 (voucher), ops 8/9/10/15 (its own key)
                --> vfs            /data/meter.state, /data/meter.key
nox_settle --> OP_DRAIN: the newest signed voucher of each channel
```

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x79
```

CoreExec, IPC, Memory, Crypto and FileSystem. There is no Network: settle
talks to the chain. Service `service:4114:payment`; replies go to the kernel
reply endpoint. Requests arrive through `MkIpcRecvFrom`, so each op's caller
is checked by which registry names the kernel-stamped pid owns
(`nonos_nox_meter::caller`).

`nonos-capsule-payment` is in `microkernel-desktop-base`. The kernel mirror is
`src/userspace/capsule_payment`. Init spawns it with the desktop services
(`spawn_plan/payment.rs`), after the keyring and the vfs. A change to the caps

## Keys

The meter holds one key, its own secp256k1 key. It is generated in the
keyring once, exported, sealed with nonos_vault under the root
machine_key(BOUND_PREFIX | "nonos.meter.root.v1") to `/data/meter.key`, and
imported again at every boot. Only the meter's manifest namespace can derive
that root (C2c). The state file's tag key comes from the same root. A sealed
key that does not open is refused, never replaced: channels are bound to the
meter's address, and each names a reclaimTo for a key that is lost anyway.
There is no Ed25519 key and no NXL1 in v1.

Until C2c names the bound prefix byte, `keys::bound::BOUND_PREFIX` is `None`
and every op that needs the key answers EAGAIN. On a boot that keeps nothing
the meter holds no key: PAY, SET_BUDGET and DRAIN answer ENOPERSIST, while
LIST_TOKENS and ACCOUNT still answer. So the budget stays zero.

## Interface contract

The wire is `nonos_payment_proto`: market_proto's 20-byte header under the
magic "NPAY", a reply's payload starting with its status, every body and
status defined there.

| Op | Caller | Body | Reply |
|---|---|---|---|
| 1 HEALTHCHECK | anyone | none | none |
| 2 PAY | payers (installer, app.store, the wallet) | `PayRequest` | one `VoucherRecord` |
| 3 DRAIN | nox_settle | first index u8 | `Drain` |
| 4 LIST_TOKENS | anyone | none | one NOX row per chain where it is deployed (`nonos_nox_config`) |
| 5 ACCOUNT | wallet | first index u8 | `Account` |
| 6 SET_BUDGET | wallet | `SetBudget` | none |
| 7 | - | - | EINVAL: NXL1 is not in v1 |
| 8-12 | - | - | EINVAL until ORDER_* (B5) and OPEN (B3c) land |

Every refusal is a status named in `nonos_payment_proto::status`.
