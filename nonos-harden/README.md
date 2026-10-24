# nonos-harden

Use this host command to get one GO or NO-GO before an image touches a
machine. It runs the tree's verifiers over a sealed release and prints one
versioned verdict. You can sign that verdict with your own Ed25519 key.

```
cargo build --release --manifest-path nonos-harden/Cargo.toml
target/release/nonos-harden check --release <sealed-dir> [--tree <checkout>] [--format json] [--out FILE [--sign-key KEY]]
```

**GO** means every one of the 13 checks ran and passed. That covers four
things:

- **Sealed bytes:** the release's sealed bytes match its record.
- **Kernel signature:** the kernel's Ed25519 and ML-DSA-65 signatures verify
  under the tree's kernel keys.
- **Source:** the build receipt agrees with the release, and the tree is the
  receipt's source.
- **Tree verifiers:** the tree's own checks pass. These are `nonos-verify`
  hygiene, trust-chain and supply-chain, `inputs.py --check`, and the verify
  matrix's static-checks and proof-crates rows.

A development image that passes reads `GO (development image, not a release)`.
When checks carry limits, the line counts them (`GO, 2 declared limits`), and
each limit is listed under it. A verifier's gap is a limit only if the tree
declares it in `scripts/baselines/declared-limits.txt`; any other gap is
NO-GO.

Any check that fails or could not run makes the verdict **NO-GO**. The
verdict names that check and gives its reason.

GO does **not** mean the image was booted. No QEMU run or device is
involved, and every verdict says so in its `scope` field.

The full page is [docs/release/nonos-harden.md](../docs/release/nonos-harden.md).
It covers:

- each check
- the verdict format, `nonos-harden-verdict/1`
- exit codes
- signing
- how it is proved

## Proofs

- `cargo test` runs the proofs: fixtures sealed by the seal's own code,
  proptest fuzzing of every untrusted parser, and a self-check of the mutant
  list.
- `python3 mutants.py` runs each mutant in `mutants.json` against a copy of
  the crate. It confirms that the proof each mutant names fails.
