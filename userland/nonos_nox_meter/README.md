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
  past expiry, and on overflow. `synced` takes the chain's `settled` as a
  floor under `cumulative`, so a rolled-back state file cannot sign below
  what was paid. `voucher` builds the newest voucher with the channel's own
  expiry, which the contract requires; the caller never picks one.
  default allows nothing; `fresh` is a new allowance from SET_BUDGET.
- `state`: the two together. Every change returns a new value, which the
  capsule writes to disk before anything is signed from it. A full channel
  table forgets only a channel past expiry and fully settled.
- `persist`: the state file (`NXMT` v1), always `FILE_LEN` bytes because the
  vfs replaces a record only at its own length, and tagged under a key only
  the meter can derive, since any FileSystem holder can unlink and recreate
  the name.
- `caller`: which registry names may call which op.
- `pay`: PAY v2's body, and `priced`, which refuses unless the release is
  trusted and every word of the request equals its signed commerce block.
- `state::ChainView`: settle's CHANNEL_STATE; a closed channel takes no
  further charge.
- `wire`: the bodies of SET_BUDGET, ACCOUNT and DRAIN.

Hashes are injected through `nonos_marketplace_abi::Keccak`.

## What the tag does not stop

spent. Channel deposits bound it, and the chain's settled amounts raise each
channel's floor again. Closing it needs a monotonic counter, such as a TPM NV
index, which is later work.

## Tests

37 host tests in `tests/`, with `sha3` as a dev-dependency, so every hash is
checked against an independent implementation.

```sh
cd userland/nonos_nox_meter
cargo test
cargo build --release -Zbuild-std=core,alloc --target ../x86_64-nonos-user.json
```
