# nonos_payment_proto

The meter's wire (service `payment`, plan/21 and plan/42), shared by the
meter and every caller: `no_std`, alloc only, no syscalls.

- Header: market_proto's 20 bytes under the magic "NPAY" (`frame`,
  `header`). A reply echoes op, flags and request id, and its payload starts
  with a status. `reply_body` believes a reply only for its own call.
- Bodies, every length explicit, all LE: `PayRequest` (PAY v2, lp is a u16
  length), `SetBudget`, `Account`, `VoucherRecord` and `Drain`; DRAIN's two forms
  (`DrainRequest`: LIST, and SIGN with `DrainSign`); OPEN's `OpenStart` and
  `OpenConfirm`; TOPUP's `TopUp` and CLOSE's `Close`.
- Ops 1 to 14, and every status the meter answers, its own below -1000:
  ENOPERSIST, EFROZEN and EGAS.

ORDER_* bodies land with B5.

```sh
cargo test
cargo build --release -Zbuild-std=core,alloc --target ../x86_64-nonos-user.json
```
