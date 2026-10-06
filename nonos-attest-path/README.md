# nonos-attest-path

The trailer format and the Merkle path check every attestation gate shares:
the kernel's spawn gate, the bootloader's kernel gate, the kernel's check of
the bootloader (through `nonos-boot-measure`), `nonos-secops`, and the enroll
tool that writes the trailers. One crate, so the check that admits an image is
the same code everywhere. The design is on
[the STARK layer page](../docs/handbook/trust/stark.md).

## What is in it

- `context`: the two context builders. `capsule_context` is 48 bytes: the
  capsule ELF's BLAKE3, the capability word and the policy epoch.
  `boot_context` is 40 bytes: a measurement and the boot epoch.
- `leaf`: `context_digest`, BLAKE3 under `NONOS-ATTEST-PATH-LEAF-v3`, and
  `leaf_of`, which puts the digest and the kind (kernel, capsule, padding,
  bootloader) into one width-8 Poseidon state over Goldilocks. `pad_leaf` makes
  padding leaves from a seed.
- `poseidon`, `params`, `field`: the permutation, `x^7` on every lane, 32 full
  rounds, a Cauchy MDS matrix, round constants from BLAKE3.
- `trailer`: the v3 path, `NZKPATH1`: depth, siblings as canonical
  little-endian words, direction bits. One encoding per path.
- `v4`: the v4 container, `NATTV4`: the kind, the v3 path and a STARK proof.
  `parse_v4` refuses another magic, the wrong kind, an empty or oversized proof
  and any byte past it. `words` builds the nine public words `nox_verify`
  checks.
- `verify`: folds a path from a context's leaf and compares it with the root.
  A zero root admits nothing.
- `tree` (feature `alloc`): `Tree::commit` and `Tree::trailer`, for the enroll
  tool. The gates build without `alloc`, so nothing on the verify path
  allocates.

The crate is `no_std` and does not verify STARK proofs itself; the gates pass
the proof to `nox_verify` from STARKs with the words built here.

## Checks

- 43 host tests: known answers, tamper and hostile cases, tree and root
  tests, and five seeded random-input tests that throw arbitrary bytes, every
  truncation and random damage at `parse_v4` and `verify`.
- Two Kani proofs: `parse_v4` and the v3 reader never panic and accept only
  their layouts, at 48 and 74 bytes (`src/kani_proofs.rs`).
   records both verified.
- Two cargo-fuzz targets, `v4_parse` and `v3_path`, run nightly by
  `.github/workflows/fuzz.yml`.

`nix flake check` runs the tests; `verify.yml`'s `proof-crates-kani` job runs
Kani.

## What it does not do

- The path check rests on Poseidon's collision resistance, which no proof in
  this tree covers.
- A v4 trailer carries its v3 path in the clear, so it reveals the slot's
  position and siblings.
