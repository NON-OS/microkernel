# anon_link_proofs

Host-runnable proofs for `net.anon`'s link handshake: the VERSIONS cell, the
Ed25519 certificates and the CERTS cell chain that binds a TLS session to a
relay's identity. The crate includes the capsule's own `link/` and `cell/`
sources by `#[path]`, and takes Ed25519 and SHA-256 from the kernel's own
primitives through `crypto_proofs`, so both halves are real code.

The vectors in `vectors/` are a CERTS cell and the TLS leaf certificate
captured from a live Anyone relay (`vectors/source.txt`). The tests check that
the chain binds to the right identity, and that a tampered, reordered or
mismatched chain is refused (`bind_tests`, `bind_tamper_tests`,
`bind_reject_tests`), plus the certificate and version parsing.

 records this crate as failing for want of
`vectors/certs_cell.bin`. The vector is in the tree at this commit; no
committed log shows the crate passing on it.

Run: `cd userland/anon_link_proofs && cargo test --release`. `nix flake check` runs it as
`proofs-anon_link_proofs` (`tools/nix/checks.nix`): `cargo test --release` with overflow
checks on, then clippy; CI runs `nix flake check` in `.github/workflows/verify.yml`.;

See [the Anyone onion routing capsule](../../docs/handbook/network/anyone.md) and
[proofs](../../docs/handbook/verification/proofs.md).
