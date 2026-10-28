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

`src/tests/exit_ports/`, mounted from `microdesc_tests.rs`, holds the exit chosen
by port: the `p` line kept whole (accept and reject, ranges, single ports, the
64-range bound, malformed lines refused, random bytes that never panic),
`exits_to` at ports 1, 65535 and the ends of each range, the web ports read by
`exits_web` and nothing else, the exit drawn for a port with the same weights
and identity and /16 exclusions as any exit (and exactly the old draw for 443),
the bounded list of exits that refused a port, which no draw for that port
picks again, and the rule for sending an open to a second exit: reasons 4, 7,
8, 9 and 11, once only, with at most 8 KiB of early bytes kept.

Run: `cd userland/anon_ntor_proofs && cargo test --release`. `nix flake check` runs it as
`proofs-anon_ntor_proofs` (`tools/nix/checks.nix`): `cargo test --release` with overflow
checks on, then clippy; CI runs `nix flake check` in `.github/workflows/verify.yml`.

See [the Anyone onion routing capsule](../../docs/handbook/network/anyone.md) and
[proofs](../../docs/handbook/verification/proofs.md).
