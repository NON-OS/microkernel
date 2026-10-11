# nonos_settle_proto

The wire of `capsule_nox_settle` (service `nox_settle`, the plan), shared by
the settle capsule (B4) and its callers, the meter first. `no_std`, no
dependencies.

- Requests: `seq u32 | op u16 | reserved u16 | body`. Replies:
  `seq u32 | status i32 | body`. All little-endian.
- Ops: 1 HEALTHCHECK, 2 BROADCAST, 3 READ_NONCE, 4 CHANNEL_STATE,
  5 ETH_BALANCE, 6 OPENED. CHANNEL_STATE's request and reply are coded here;
  the other bodies land with the units that use them.

```sh
cargo test
```
