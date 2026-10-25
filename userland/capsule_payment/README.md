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
                --> nox_settle     CHANNEL_STATE, GAS, BROADCAST, OPENED (settle_proto)
                --> keyring        op 30 (voucher), op 25 (approve, open), ops 8/9/10/15 (its key)
                --> vfs            /data/meter.state, /data/meter.key
nox_settle --> OP_DRAIN: LIST the newest signed vouchers, SIGN a settle() of them
```

## Microkernel contract

```text
CAPSULE_REQUIRED_CAPS = 0x10000079
```

CoreExec, IPC, Memory, Crypto, FileSystem and Keyring. There is no Network: settle
talks to the chain. Service `service:4114:payment`; replies go to the kernel
reply endpoint. Requests arrive through `MkIpcRecvFrom`, so each op's caller
is checked by which registry names the kernel-stamped pid owns
(`nonos_nox_meter::caller`).

`nonos-capsule-payment` turns the meter on. No profile array names it: the
owner adds it to one after re-signing (plan/42). The kernel mirror is
`src/userspace/capsule_payment`. Init spawns it with the desktop services
(`spawn_plan/payment.rs`), after the keyring and the vfs. A change to the caps
or the ELF needs the owner to re-sign the manifest and id-cert and re-enroll.

## Keys

The meter holds one key, its own secp256k1 key. It is generated in the
keyring once, exported, sealed with nonos_vault under the root
machine_key(BOUND_PREFIX | "nonos.meter.root.v1") to `/data/meter.key`, and
imported again at every boot. Only the meter's manifest namespace can derive
that root (C2c). The state file's tag key comes from the same root. A sealed
key that does not open is refused, never replaced: channels are bound to the
meter's address, and each names a reclaimTo for a key that is lost anyway.
There is no Ed25519 key and no NXL1 in v1 (plan/20, v1.3).

The label is `nonos_nox_meter::root::root_label()`: C2c's bound prefix 0x01,
then "nonos.meter.root.v1", 20 bytes, pinned by a test. The root is private
only on a kernel that carries C2c. A kernel without it derives a 0x01 label
like any other, for any Crypto holder, so the meter must ship only with C2c.
On a boot that keeps nothing the meter holds no key: PAY, SET_BUDGET and
DRAIN answer ENOPERSIST, while LIST_TOKENS and ACCOUNT still answer. So the budget stays zero.

## Interface contract

The wire is `nonos_payment_proto`: market_proto's 20-byte header under the
magic "NPAY", a reply's payload starting with its status, every body and
status defined there.

| Op | Caller | Body | Reply |
|---|---|---|---|
| 1 HEALTHCHECK | anyone | none | none |
| 2 PAY | payers (installer, app.store, the wallet) | `PayRequest` | one `VoucherRecord` |
| 3 DRAIN | nox_settle | `DrainRequest`: LIST (first index u8) or SIGN | LIST: `Drain`. SIGN: the raw settle() of the named channels' newest vouchers, signed by the meter's key; ENOENT when none is above its floor, or none is newer than the last settle carried and the nonce is not that settle's own (a fee bump) |
| 4 LIST_TOKENS | anyone | none | one NOX row per chain where it is deployed (`nonos_nox_config`) |
| 5 ACCOUNT | wallet | first index u8 | `Account` |
| 6 SET_BUDGET | wallet | `SetBudget` | none |
| 7 | - | - | EINVAL: NXL1 is not in v1 |
| 8-11 | - | - | EINVAL until ORDER_* land with escrow (B5) |
| 12 OPEN | wallet | `OpenStart` or `OpenConfirm` | START: a job id (u32), or EBUSY; the job's result is approveTx[32] and openTx[32], both signed by the meter's key and broadcast by settle, never above the config's fee caps (EGAS); CONFIRM: the channel id, once a ChannelOpened matching the request is mined (EAGAIN before) |
| 13 TOPUP | wallet | `TopUp` | a job id, or EBUSY; its result is approveTx[32] and topUpTx[32], signed by the meter's key and broadcast by settle |
| 14 CLOSE | wallet | `Close` | a job id, or EBUSY; its result is closeTx[32], or ETIME before expiry plus the contract's GRACE. Anyone may close: the meter's key is a convenience |
| 15 JOB | wallet | id u32 | EAGAIN while the job runs, then its result once; ENOENT for any other id. Each step is an ASYNC settle request answered into this capsule's inbox; a step with no answer in 150 s fails the job (ETIME) |
| 16 CLAIM | wallet | chain_id u64 | a job id, or EBUSY; the job reads claimable(NOX, meter) (READ 60), ENOENT when nothing is credited, then signs ShieldedPool.claim(1, meter) itself (CLAIM_GAS 150000, the fee caps) and has settle broadcast it; its result is claimTx (v1.4.17) |
| 21 FUNDED | app.store | `FundedAsk`: chain_id u64, lp listing | one byte: 1 when PAY of one unit, to the record's payee in its unit at its unitPrice (the listing's id from OP_GET_COMMERCE), would charge, by PAY's own `priced` and `landing`: channel, margin, deposit, unit and budget; 0 for every other case, a failed market lookup or no key included. EINVAL only for a body that does not parse |

Every refusal is a status named in `nonos_payment_proto::status`.

PAY refuses on a channel within a day of its expiry (ETIME), so the last
drain lands before anyone may close it. The state file (NXMT v4) keeps the
chain and nonce of the last settle() the meter signed, per channel the
cumulative it carried, and when it last signed the same vouchers again: the
bound on SIGN. Once its account nonce has passed the last settle's (that
nonce went to another transaction), the meter signs the same vouchers again
at most once a day (v1.4.12).
