# tls_proofs

Host-runnable proofs for `nonos_tls`, which five capsules depend on. The crate
includes the real `nonos_tls` source by `#[path]`. Its stand-in `nonos_libc`
answers every crypto call with the crypto capsule's own handlers, so
signatures are verified by the pool's code.

The inputs are bytes captured from a live Anyone relay, the RFC 8448 handshake
trace, the chain a re-signing gateway served, and a TLS server played in
`src/tests/fake_server.rs` that signs its CertificateVerify. The tests
(`src/tests/`) cover the key schedule against RFC 8448, the handshake step by
step, HelloRetryRequest, the P-256 share, the flight order (one Certificate,
none after CertificateVerify), the certificate message and SPKI parsing, the
chain walk's length, authority, validity and name rules (forged names
included), the record size limits, alerts, and application data reads.

Run: `cd userland/tls_proofs && cargo test --release`. `nix flake check` runs it
as `proofs-tls_proofs` (`tools/nix/checks.nix`), and `.github/workflows/fuzz.yml`
fuzzes it.

See [the network stack](../../docs/handbook/network/stack.md) and
[proofs](../../docs/handbook/verification/proofs.md).
