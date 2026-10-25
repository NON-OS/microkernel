# nonos_p256

`nonos_p256` is the system's NIST P-256 for protocols run in userland:
WPA3-SAE in `nonos_wifi_core` (IEEE 802.11 group 19) and LE Secure
Connections in `nonos_bt_smp`. Both take the curve from here, so neither
carries curve code of its own. `no_std`.

## Public surface

- `LEN` (32), and `PRIME`, `ORDER`, `CURVE_B` and `TWO_256_MOD_P`, big
  endian.
- Field elements: `fe_const`, `curve_a`, `curve_b`, `curve_rhs`, and
  `fe_from_bytes_ct`, `fe_reduce32`, `fe_reduce_wide` and `fe_bytes`. The
  conversions from octets return a `Choice` or reduce with a masked select,
  so a secret value is never branched on.
- Scalars: `scalar_from_bytes` (refuses a value at or above the order),
  `scalar_reduce`, `scalar_bytes` and `scalar_is_trivial`.
- Points: `point_from_xy` refuses a coordinate at or above p and a point off
  the curve, before any arithmetic. `point_xy` gives x || y, or `None` for
  the identity.
- `Secret`: an ECDH secret in [1, r - 1], refused otherwise rather than
  reduced. `public_xy` is the secret times the generator; `shared_x` is the
  x coordinate of the secret times a checked peer point. The secret is
  overwritten with volatile writes when dropped.
- The types and traits of the arithmetic beneath (`FieldElement`, `Scalar`,
  `ProjectivePoint`, `AffinePoint`, `U256`, `Choice`, `ConditionallySelectable`
  and the rest), so a caller needs no other P-256 crate.

## The arithmetic

The arithmetic is RustCrypto's `p256` 0.13 on `primeorder` 0.13, the crate,
version and features the TLS client and `capsule_crypto` already build for
the capsule target. Scalar multiplication is constant time in the scalar
(`primeorder`'s `ProjectivePoint::mul`):
- a fixed 4-bit window over all 64 windows of the scalar, four doublings
  and one addition each, leading zero windows included;
- a table of the 16 multiples 0P to 15P, read in full at every window. Each
  entry is folded in with `conditional_assign` under a mask computed from
  the window, so the memory touched never depends on the scalar;
- the complete addition and doubling formulas of Renes, Costello and Batina
  (2015), which have no case for the identity or for equal inputs, so there
  is nothing to branch on.

Field inversion and square roots are fixed exponentiations.

## Tests

No tests in the crate. `nonos_p256_proofs` holds them.
