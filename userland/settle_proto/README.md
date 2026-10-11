# nonos_settle_proto

The wire of `capsule_nox_settle` (service `nox_settle`, plan/26 and plan/42),
shared by settle (B4) and its callers, the meter first. `no_std`, alloc only.

- Header: market_proto's 20 bytes under the magic "NSTL". A reply's payload
  starts with a status, and `reply_body` believes it only for its own call.
- Ops: 1 HEALTHCHECK, 2 BROADCAST (chain_id | raw signed tx -> hash),
  3 READ_NONCE, 4 CHANNEL_STATE, 5 GAS (balance, next nonce, fee
  suggestions), 6 OPENED (a transaction's ChannelOpened event; EAGAIN while
  unmined, ENOENT without one).
- Bodies: `ChannelState`, `Gas`, `Opened`, and `chain_hash`, the chain id and
  32-byte id most requests carry.

READ, INTENT and FETCH land with settle itself (B4).

```sh
cargo test
```
