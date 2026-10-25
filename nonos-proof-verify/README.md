# nonos-proof-verify

The proof verifier anyone can run. It checks a kernel's, a loader's or a
capsule's v4 attestation trailer against a policy root the way the gates
check it: a strict parse, the Merkle path folded by `nonos-attest-path`, then
the STARK proof of the same slot verified by `nox_verify`. The context comes
from the image itself. `nonos-stark-enroll` re-checks every trailer it writes
with this crate's `check_v4`, so enrolling and verifying share one check.

```sh
cargo build --release
target/release/nonos-proof-verify transcript \
  ../nonos-data/trust/policy/zk_capsule_policy_root.bin.transcript \
  ../nonos-data/trust/capsules/*.zk_trailer.bin
```

| Command | Checks |
|---|---|
| `capsule --root R --image ELF --caps HEX TRAILER...` | the spawn gate's check |
| `kernel --root R --image IMAGE TRAILER...` | the loader's check of the kernel |
| `bootloader --root R --image EFI TRAILER...` | the kernel's check of its loader (Authenticode) |
| `transcript FILE [--root R] TRAILER...` | the root rebuilt from its transcript, each trailer in the slot it opens |

Options: `--json` (the v1 report), `--allow-dev` (accept path-only
development trailers, never as a release), `--help`, `--version`.

Exit 0 verified, 1 failed, 2 could not check. A run that could not check
something never exits 0. A root other than the release roots committed in
`nonos-data/trust/policy` is reported as `development root, not a release`.

The full page, with the codes and the JSON schema, is
[docs/security/checking-the-claims-attestation-proofs.md](../docs/security/checking-the-claims-attestation-proofs.md).

## In a browser

`wasm/` compiles this crate to a WebAssembly module with no imports and no
bindings. It runs the command line's own `cli` over files the page hands it,
so for the same arguments and files it prints the same report and exit code.
`web/` is the page: pick the files, and it shows the verdict, the root, each
trailer, the v1 JSON and the equivalent command. Nothing leaves the tab.

```sh
nix build .#proof-verify-web     # the module and the page, ready to serve
# or by hand:
(cd wasm && cargo build --release --target wasm32-unknown-unknown)
node web/smoke.mjs wasm/target/wasm32-unknown-unknown/release/nonos_proof_verify_wasm.wasm \
  target/release/nonos-proof-verify
```

`web/smoke.mjs` drives the module through the page's own loader. It checks
every committed capsule trailer, the fixture's cases and flipped trailers,
and compares each report byte for byte with the command line's.

## Verification

- Tests: `cargo test --release`, against the committed material and real
  fixture enrollments (`tests/fixture/README.md`).
- Fuzz: `cd fuzz && cargo +nightly-2026-01-16 fuzz run trailer` (or
  `transcript`).
- The browser build: `cd wasm && cargo test --release`, then `web/smoke.mjs`.
