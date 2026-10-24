# nonos-harden

A host command that gives one GO or NO-GO before an image touches a machine.
It checks a sealed release against the tree that built it, runs the tree's
own verifiers, and prints one versioned verdict. You can sign that verdict
with your own Ed25519 key.

```
cargo build --release --manifest-path nonos-harden/Cargo.toml
target/release/nonos-harden check --release <sealed-dir> [--tree <checkout>] [--format json] [--out FILE [--sign-key KEY]]
```

**GO** means all 13 checks ran and passed:

- **Sealed bytes.** The release's sealed bytes are the ones its record
  names.
- **Kernel signature.** The kernel's Ed25519 and ML-DSA-65 signatures verify
  under the tree's kernel keys.
- **The kernel that boots.** The kernel on the ESP is that signed kernel.
- **Kernel keys.** For a release, the kernel keys are the ones in the
  receipt's commit.
- **Source.** The tree on disk is that commit, hashed file by file. It
  differs only where the seal stages what it writes.
- **Verifiers.** The tree's own verifiers pass.

**NO-GO** means at least one check failed or could not run. The verdict names
it and gives its reason.

A release's GO exits 0. A development image's GO exits 5, so a script that
deploys on status 0 can never deploy it.

A GO always carries limits, and its line counts them. For example:

- `GO, 1 declared limit`
- `GO (development image, not a release), 4 declared limits`

Each limit is listed right under the verdict line. They come from two places:

- **The verifiers.** A verifier's gap is a limit only if the tree declares it
  in `scripts/baselines/declared-limits.txt`. Any other gap is NO-GO.
- **nonos-harden itself.** It states one limit on every GO: the release
  record is not signed, so the loader and the USB and ISO images are checked
  only against it.

GO does **not** mean the image was booted. No QEMU run or device is involved,
and every verdict says so.

A release cannot bring its own verifiers. A tree at or inside the release
directory is refused.

The full page is [docs/release/nonos-harden.md](../docs/release/nonos-harden.md).
It covers what is trusted, each check, the verdict format
`nonos-harden-verdict/1`, exit codes, signing, and how it is proved.

## Proofs

- `cargo test` runs the proofs:
  - fixtures sealed by the seal's own code
  - one proof for each attack from the adversarial review
  - proptest fuzzing of every untrusted parser
  - a self-check of the mutant list
- `python3 mutants.py` runs each mutant in `mutants.json` against a copy of
  the crate. It confirms that the proof each mutant names fails.
