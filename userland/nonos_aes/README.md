# nonos_aes

`nonos_aes` is the system's AES: AES-128 and AES-256, counter modes,
AES-128-CMAC, and the AES-128 inverse cipher. `no_std`, no dependencies. Its
callers are the onion transport, the Wi-Fi core and the Bluetooth Security
Manager, which share it rather than each carrying a copy.

## Public surface

- `Aes128::new(key)` and `Aes128::encrypt_block(&mut block)`. Dropping an
  `Aes128` zeroes its round keys with volatile writes.
- `Ctr128Be::new(key)`: the counter is the whole 128-bit block, big endian,
  starting at zero. This is what a Tor relay runs.
- `Ctr64Be::new(key, iv)`: the counter is the low 64 bits of `iv`, big endian,
  wrapping within them. The high half stays fixed. This is the Sphinx layout.
- Both have `apply(&mut data)`, which XORs the keystream in and carries on
  from the exact byte where the previous call stopped, so data does not need
  to be block aligned.
- `BLOCK_BYTES` and `KEY_BYTES`, both 16.
- `Aes128::decrypt_block(&mut block)`: the inverse cipher, for AES Key
  Unwrap (RFC 3394). It always runs in software.
- `Aes256::new(key)` and `encrypt_block`, and `Ctr256Be`.
- `aes_cmac(key, parts)`: AES-128-CMAC (RFC 4493) of the parts taken as one
  message, without copying them together. `CMAC_BYTES` is 16.

The S-box is computed, not looked up: each byte is inverted in GF(2^8) as
x^254 with masked shift-and-add multiplies, then put through the FIPS 197
affine map (`src/sub_byte.rs`). The inverse S-box reuses it:
S^-1(y) = A^-1(S(A^-1(y))) for the affine map A (`src/inv_sub_byte.rs`), and
InvMixColumns is a pass of `xtime` before MixColumns (`src/inv_steps.rs`).
CMAC's subkey doubling masks its constant in. The goal is no table index or
branch on key or state bytes.

## What it does not do

No AES-192, no AES-256 inverse cipher, no authenticated encryption mode. It
does not track nonces: reusing a key and counter is the caller's mistake to
avoid.

## Users

- `capsule_net_anon` uses `Ctr128Be` for each hop's forward and backward
  keystream (`src/circuit/hop.rs`).
- `anon_ntor_proofs` re-exports it through its crypto shim.
- `nonos_wifi_core` uses `Aes128` for CCMP and key unwrap, `Aes256` for
  GCMP-256 and BIP-GMAC-256, and `aes_cmac` for the EAPOL-Key MIC and BIP.
- `nonos_bt_smp` uses `aes_cmac` for f4, f5, f6 and g2.
- `capsule_net_nym` does not use this crate. It has its own AES with a
  `Ctr64Be` in `src/crypto/aes/`, so `nonos_aes::Ctr64Be` has no caller outside
  `aes_proofs`.

## Tests

No tests in the crate. `aes_proofs` checks it against FIPS 197, its CTR
modes against SP 800-38A and its CMAC against RFC 4493, and compiles `sub_byte.rs` and `xtime.rs` by
`#[path]` to compare the S-box with a table. `nix flake check` runs that as
`proofs-aes_proofs`. There are no Kani proofs. The relay cell encryption that uses it is described
in [Anyone](../../docs/handbook/network/anyone.md). The kernel's own AES is a
separate implementation, in [Kernel crypto](../../docs/handbook/kernel/crypto.md).
