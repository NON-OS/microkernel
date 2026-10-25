# nonos_nox_meter

The meter's accounting, without the capsule around it: `no_std` and pure,
built for `x86_64-nonos-user` and tested on the host. `capsule_payment` (the
meter, the plan) links it.

- `voucher`: the Voucher struct of the plan as the nine canonical words the
  keyring signs, its struct hash, and `ChannelKey::id`, keccak of
  `abi.encode(CHANNEL_TAG, chainId, settlement, payer, payee, listingId,
  openNonce)`.
- `typed`: the `OP_SIGN_TYPED_DATA` (op 30) request for table entry 1 and its
  181-byte reply. The caller supplies no type string or domain.
- `usage` and `merkle`: one charge as an OZ standard-tree leaf, and the
  sorted-pair root over a voucher's charges.
- `channel`: a prefunded channel. `charged` refuses past the deposit, at or
  past expiry, on overflow, and on a frozen channel. `synced` raises the
  settled floor only up to the largest cumulative this meter signed; a
  report above that (a lying RPC, or a file rolled back past a voucher the
  chain has seen) freezes the channel instead (v1.3, E13). `voucher`
  builds the newest voucher with the channel's own expiry, which the
  contract requires; the caller never picks one.
- `budget`: the owner's caps, global and per listing, with an expiry. The
  default allows nothing; `fresh` is a new allowance from SET_BUDGET.
- `state`: the two together. Every change returns a new value, which the
  capsule writes to disk before anything is signed from it. A full channel
  table forgets only a channel past expiry and fully settled.
- `persist`: the state file (`NXMT` v4: the last settle, each channel's
  `carried`, and the last daily retry), always `FILE_LEN` bytes because the
  vfs replaces a record only at its own length, and tagged under a key only
  the meter can derive, since any FileSystem holder can unlink and recreate
  the name.
- `open`: OPEN's two calls. `open_plan` is START's approve and open at the
  meter's next two nonces, refused for a zero amount, a tip above the fee
  cap, or ETH short of gas for both; their calldata is encoded here, with
  pinned selectors, and `sign_tx_request` is the keyring's op 25 body.
  `confirmed` is CONFIRM's check of the ChannelOpened event.
- `settle`: DRAIN SIGN's `settle_plan`, the settle() calldata, and the
  TOPUP and CLOSE plans. Another settle() is signed only for a newer voucher
  than the last one carried, at that one's own nonce, or, once the account
  nonce has passed it, for the same vouchers at most once a day; every plan stays
  under the fee caps, and a settle() under `SETTLE_GAS_MAX`.
- `channel`: no charge within `CHARGE_MARGIN` (a day) of expiry.
- `root`: the label the meter's root is derived under, 0x01 (C2c's bound
  prefix) then "nonos.meter.root.v1".
- `caller`: which registry names may call which op.
- `pay`: `priced`, which refuses a PAY v2 request (`nonos_payment_proto`)
  unless the release is trusted and every word equals its signed commerce
  block.
  `landing`, PAY's charge decision: channel by payee, budget, unit.
  `funded`, FUNDED's bit for app.store: `priced` then `landing` on one unit.
- `state::ChainView`: settle's CHANNEL_STATE; a closed channel takes no
  further charge.
- `wire`: the meter's state as payment_proto's ACCOUNT and DRAIN replies,
  and SET_BUDGET's body as a fresh allowance. The byte layouts are
  payment_proto's.

Hashes are injected through `nonos_marketplace_abi::Keccak`.

## What the tag does not stop

A genuine older state file, replayed, restores budget the owner has since
spent. Channel deposits bound it, and a channel the chain has settled past
the file's record freezes. Closing it needs a monotonic counter, such as a
TPM NV index, which is later work.

## Tests

42 host tests in `tests/`, with `sha3` as a dev-dependency, so every hash is
checked against an independent implementation.

```sh
cd userland/nonos_nox_meter
cargo test
cargo build --release -Zbuild-std=core,alloc --target ../x86_64-nonos-user.json
```
