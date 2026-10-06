# anon_ntor_proofs

Host-runnable proofs for most of `net.anon`. The crate includes the capsule's
own source by `#[path]` (cells, circuits, crypto, the directory parsers, the
link step and parts of the manager, path and stream code), so what is checked
is what ships. The expected values are not produced by this code: the ntor
values come from `src/test/ntor_ref.py` in the anyone-protocol/ator-protocol
tree, AES from FIPS 197 and SHA-1 from FIPS 180.

The tests (`src/tests/`) cover the ntor handshake and the relay cell crypto it
keys; SHA-1, SHA-256 and the running digest; base64; authority certificates,
the consensus header and relays, a live consensus fixture, microdescriptors
and their join; the path draw, its bias and family exclusion; the guard pick
that avoids guards given up on; the directory refresh rule; circuit build
answers, onion layers, DESTROY and cell order; SENDME windows and backpressure;
RELAY_BEGIN; stream ownership, scope and the per-owner share; and the SOCKS
front's handshake, limits, exit replies, replays and missed answers.

Run: `cd userland/anon_ntor_proofs && cargo test --release`. `nix flake check` runs it as
`proofs-anon_ntor_proofs` (`tools/nix/checks.nix`): `cargo test --release` with overflow
checks on, then clippy; CI runs `nix flake check` in `.github/workflows/verify.yml`.

See [the Anyone onion routing capsule](../../docs/handbook/network/anyone.md) and
[proofs](../../docs/handbook/verification/proofs.md).
