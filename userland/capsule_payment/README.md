# capsule_payment

## Role

`capsule_payment` is the meter (plan/20 and plan/21). It signs cumulative NOX
vouchers on prefunded channels, within a budget the owner sets from the
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
or the ELF needs the owner to re-sign the manifest and id-cert and re-enroll.

## Keys

The meter's root is the machine key under the caller-bound label
`nonos.meter.root.v1`. No other capsule can derive it (C2c). From the root:
- the state file's tag key;
- the Ed25519 key for NXL1;
- the sealing key for the secp256k1 secret. That secret is generated in the
  keyring once, exported, sealed to `/data/meter.key`, and imported again at
  every boot. A sealed key that does not open is refused, never replaced:
  channels are bound to the meter's address.

Until C2c names the bound prefix byte, `keys::bound::BOUND_PREFIX` is `None`.
Every op that needs a key answers EAGAIN, and no state is loaded or saved. On
a boot that keeps nothing, the meter changes no state (EACCES), so its budget
stays zero.

## Interface contract

Header `seq u32 | op u16 | reserved u16`; reply `seq u32 | status i32 | body`.

| Op | Caller | Body | Reply |
|---|---|---|---|
| 1 HEALTHCHECK | anyone | none | none |
| 2 PAY | payers | `nonos_nox_meter::pay` (v2); the old 124-byte body is EINVAL | the signed voucher, 265 bytes |
| 3 DRAIN | nox_settle | first index u8 | `wire::drain_reply` |
| 4 LIST_TOKENS | anyone | none | one NOX row per chain where it is deployed (`nonos_nox_config`) |
| 5 ACCOUNT | wallet | first index u8 | `wire::account_reply` |
| 6 SET_BUDGET | wallet | `wire::parse_budget` | none |

Refusals:
- ENOENT: no channel, or no NOX on the chain;
- EIO: a bad file or reply;
- ENFILE: the table is full;
- ENOSPC: past the deposit;
- ETIME: expired;
- EOVERFLOW;
- EDQUOT: over budget;
- EPROTO: a request word off the signed block;
- EOPNOTSUPP: not a voucher listing;
- EDOM: below the minimum charge;
- EKEYREJECTED: an untrusted release;
- EPERM: the caller does not hold the op's role.
