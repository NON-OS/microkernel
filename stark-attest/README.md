# stark-attest

Attest a set of artifacts with one 32-byte statement, transparently, and
post-quantum. Then verify it anywhere, including in a browser tab.

`stark-attest` commits a set of files (a release, a firmware image and its
components, a package registry snapshot, an operating system's userland) to a
single Merkle root, and issues each member a self-contained trailer: a STARK
membership proof binding that member's measurement, a caller-chosen context,
and an epoch, under that root. Anyone holding the root can verify any member
in milliseconds. There is no trusted setup, no ceremony, and no toxic waste;
the construction rests on hashing (Poseidon over the Goldilocks field, FRI
for low-degree testing), which is also why it is post-quantum by construction
rather than by patch.

The engine is the `nonos-stark` crate the NONOS operating system built its
first gates on. The gates on NONOS main
have since moved to v4 trailers and the `nox_verify` verifier from
NON-OS/STARKs; this crate checks the v3 trailers. The crate is also
the engine of the zkolang shielded-transfer stack, and the two deployments
develop it against each other's test corpora, which has repeatedly caught
defects neither could see alone. The prover and the verifier read their
soundness parameters from one module (`attest_params.rs`), so a drift between
the two is a compile error, not a quiet downgrade.

## Verify it yourself, right now

This crate's verifier compiles to WebAssembly. `verifier-wasm/` builds
`verify_attestation_trailer`, the v3 check the NONOS kernel ran at spawn before
it moved to `nox_verify`, into 94 KB of wasm, and
`web/` is a dependency-free page that loads a real enrolled kernel's trailer
and root, verifies the STARK in your tab in about 100 ms, and lets you flip
one bit of the proof to watch it get refused. Nothing leaves your machine.

```
cargo build --release --target wasm32-unknown-unknown -p stark-attest-verifier-wasm
node web/smoke.mjs        # headless: verifies the fixture, then refuses a tampered one
```

In the NONOS tree, no workflow builds or tests this crate on push. Only
`ci-release-artifacts.yml` builds the wasm verifier, for a release. The CI
this README mentions below is the crate's own, from before it was folded in.
`userland/stark_proofs` tests the engine through `nix flake check`.

## The numbers, and where they come from

Every figure this repository publishes is produced by CI on a public runner
and committed into `bench/` as JSON with raw samples, host, commit, and the
compiled parameters. The canonical records are in the repo; the discipline
behind them, including the audit that once caught this repository's own
benchmark reporting elided work, is [docs/methodology.md](docs/methodology.md).

From the current records, on the CI runner:

| operation | measured |
|---|---|
| verify one member | 8.9 ms |
| reject a tampered trailer | 8.9 ms |
| prove one member | 1.2 s |
| enroll 64 MiB of artifacts, bytes through the sponge | 21.95 s |
| enroll 64 MiB of artifacts, hybrid measurement | 16.7 ms |

The last two rows are the same security statement for this deployment,
because the proof context already binds each artifact's BLAKE3 digest; the
hybrid removes redundant work rather than adding an assumption. The docs say
precisely when that reasoning does not transfer to other systems:
[docs/membership.md](docs/membership.md).

Soundness is published under both models, side by side, in
`bench/soundness.json`: the provable unique-decoding floor and the
list-decoding regime the deployed literature analyses under. Reporting only
one of them would be advocacy.

## What is machine-checked

`lean/` holds a Lean 4 development, core library only, no `sorry`, with CI
printing the axiom dependencies of every advertised theorem and failing on
anything beyond the two core axioms. It covers the security seams, not just
algorithms: the trailer parse is total (no input reads out of bounds), no
admissible artifact can occupy a reserved padding slot even against a broken
hash, the verifier's soundness parameters are its own whatever a proof
claims, and the fast Goldilocks reduction agrees with the plain modulus on
all inputs. What the proofs are about, and what they are not, is stated in
[lean/README.md](lean/README.md); the model-versus-code gap is real and one
consequence of it is documented honestly in the changelog.

The in-circuit Poseidon's parameters are derived, not tabulated: round
constants from BLAKE3 of a public domain string, a Cauchy diffusion matrix
that is MDS by theorem. A binary re-derives all of them independently and CI
fails if the compiled crate disagrees: [docs/poseidon-spec.md](docs/poseidon-spec.md).

## What problem this solves

Release signing tells you who published a set of bytes. It does not give you
a single small statement that a whole set of artifacts is exactly what it
claims, verifiable per-member without fetching or trusting anything else.
Projects hack around this with detached signature lists, SHA256SUMS files
signed as a blob, or transparency logs that need an online service. A
`stark-attest` root is 32 bytes you can print in a release note, carve into a
bootloader, or pin in a client; each artifact then carries its own proof of
membership under it, offline, with a verifier small enough to embed anywhere,
including a web page.

The proofs bind three things at once, and all three are checked in one
verification: the artifact's measurement, a context the issuer chooses (in
NONOS it is the capability mask a capsule was granted), and an epoch (so
yesterday's set cannot answer for today's). A swapped artifact, a tampered
context, and a replayed epoch all fail the same gate. Ten adversarial forgery
classes are exercised in CI on every push, and all of them are refused.

## Verified interoperability

This held while `nonos-stark-enroll` wrote v3 trailers with this crate. It
no longer does: the enroll tool now writes v4 trailers (`NATTV4`), proven with
`stark_proofs` from NON-OS/STARKs and checked with `nox_verify`, and this CLI
does not read them. The example below is the v3 interoperability as it was.
An in-crate round-trip test also pins the builder and the canonical
verifier to one wire format, a guard that exists because the two once
disagreed and nothing noticed until the gate was compiled for a second
consumer. That story is in the changelog, deliberately.

```
nonos-stark-enroll capsules root.bin 0x19:a.bin:a.zk 0x1819:b.bin:b.zk
# nonos binds context = caps as 8 big-endian bytes, then epoch as 8
stark-attest verify root.bin \
    00000000000000190000000000000001:a.bin:a.zk \
    00000000000018190000000000000001:b.bin:b.zk
# => ok a.bin, ok b.bin, verified 2 proofs
```

## Repository map

```
crates/stark-core/   the engine: field, Poseidon, FRI, AIRs, prover, verifier
cli/                 enroll / verify / selftest, plus the measurement binaries
  src/bin/           emit-bench, soundness, hash-study, compare-bench,
                     regenerate-constants, gen-web-fixture
  tests/             adversarial suite, committer, closed-form agreement, hybrid
verifier-wasm/       the gate as a cdylib for wasm32
web/                 the browser verifier page and its real fixture
lean/                machine-checked facts, core-only, axiom-audited in CI
bench/               the CI-produced measurement records
docs/                the specification and engineering documentation
```

## Status and honesty

- The prover streams per coset; large traces are not memory-bound. The
  poseidon-column path still materializes per-column trees upstream and the
  documentation of the streaming claim is scoped accordingly.
- Soundness of the construction is inherited from its standard components.
  The Lean development machine-checks specific load-bearing facts; it is not
  a soundness proof of FRI, and no independent cryptanalysis of this crate
  has been commissioned yet. Both facts are stated wherever the numbers are.
- Proving is the expensive side; verification is milliseconds. That asymmetry
  is inherent and correct: pay at release, verify everywhere, forever.
- One root commits one set. Changing any member changes the root and
  invalidates every proof; that is the semantics of a set commitment, not a
  limitation. Re-enroll on release, not on iteration.

## License

AGPL-3.0-or-later, as inherited from the source tree it was extracted from.
