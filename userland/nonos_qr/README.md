# nonos_qr

`nonos_qr` is a QR code encoder (ISO/IEC 18004) for the wallet: byte mode,
versions 1 to 10, all four error-correction levels. `no_std` with `alloc`, no
dependencies. It is written in this tree.

## Public surface

```rust
pub enum Ecc { Low, Medium, Quartile, High }

pub struct QrCode { pub size: usize, pub modules: Vec<bool> }
impl QrCode { pub fn get(&self, x: usize, y: usize) -> bool }

pub fn encode(data: &[u8], ecc: Ecc) -> Option<QrCode>
```

`encode` picks the smallest version from 1 to 10 whose byte-mode capacity holds
`data` at that level, builds the codewords with Reed-Solomon over GF(256) and
the block interleaving from the standard, places them, then tries all eight
masks and keeps the one with the lowest penalty score. It returns `None` when
the data does not fit in version 10. `modules` is row-major, `true` for a dark
module, `size` by `size`. The quiet zone is not included.

## What it does not do

No decoding, no numeric, alphanumeric or kanji modes, no versions above 10, and
no rendering to pixels. The `std` feature in `Cargo.toml` is declared but
nothing in `src/` uses it.

## Users

`capsule_wallet_nonos` encodes receive URIs and values at `Ecc::Medium`
(`src/wallet/paint/paint_receive.rs`, `src/wallet/etna/parts/qr.rs`).
`capsule_qrgen` does not use this crate; it uses the crates.io `qrcode` crate.

## Tests

None. The crate has no unit tests, no integration tests and no Kani proofs,
and no proof crate depends on it, so nothing in `nix flake check` exercises it.
The wallet is described in [Wallet](../../docs/handbook/apps/wallet.md).
