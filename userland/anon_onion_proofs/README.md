# anon_onion_proofs

`net.anon`'s onion service client, run end to end on the host. The crate compiles the capsule's own `cell`, `circuit`, `crypto`, `directory`, `manager`, `ntor`, `onion`, `path`, `stream` and `tcp_client` modules by `#[path]`, unchanged. Two things are replaced: the link to the guard (`src/link.rs`, a pair of queues) and `nonos_libc` (`shim/`: a settable clock, a seeded random source, the pool's primitives done in place, and a log of every trace line).

The tests drive the manager's own ticks against a simulated network (`src/tests/world/`). Its relays answer CREATE2 and EXTEND2 with relay-side ntor. Its HSDirs serve a descriptor built and signed with a blinded key derived with `curve25519-dalek` and `ed25519-dalek`, as the fork's `hs_descriptor.c` and `ed25519_donna_blind_secret_key` do. A rendezvous point matches cookies. Introduction points carry INTRODUCE1 to a service that decrypts it, checks its MAC and padding, and answers with RENDEZVOUS1. The service then answers on the fourth hop. All of the network's crypto comes from crates of its own (`sha3`, `aes`, `ctr`, `hkdf`, `x25519-dalek`), so the client is checked against an implementation that is not itself.

What the tests hold:

- `connect_tests`: a `.anyone` stream opens, carries `GET /v1/status` and its answer, and ends cleanly. The log names every step. No relay ever sees the address or the identity key. A second stream reuses the rendezvous circuit, and an idle one is retired with DESTROY after 600 s. The rooted and upper case forms reach the same service. An invalid address is refused at open, and nothing is sent for it.
- `retry_tests`: HSDirs without the descriptor are passed over. None holding it ends the stream with reason 2 after each responsible HSDir is asked once. Refusing introduction points are passed over. All refusing ends it with reason 3.
- `failure_tests`: a forged descriptor ends the stream with reason 2, and nothing is introduced. A restricted service with no key held is named, ending with reason 2. A service that never comes runs the lookup out of time, reason 7. A rendezvous handshake that does not verify ends the stream with reason 1, and no BEGIN is sent. Each leaves no lookup or onion circuit behind.
- `retry_tests` also: an HSDir answering something that is not a descriptor is passed over, and an introduction point whose certificate is forged is never used.
- `log_tests`: no line of the serial log names the service, in any form (address, identity or blinded key, as hex or base64), on a connect, on each failure, or for a refused address.
- `cache_tests`: a second lookup takes the descriptor from the cache and asks no HSDir. One past its lifetime is fetched again. Cached introduction points that have all moved send the lookup back to the HSDirs, and it opens on the new ones. A failed lookup caches nothing.
- `client_auth_tests`: a service that restricts its clients is reached with the key it lists, and refused, with the reason named, without one or with another. A key belongs to the caller that gave it. A malformed line keeps nothing. The key never reaches the log.
- `abandon_tests`: a stream closed mid lookup destroys its circuits at the relays. A lost link ends the lookup. Bytes sent before the stream opens get `WouldBlock`. There are four lookups at once and no more.

Not covered: the live Anyone network. `docs/handbook/network/anyone.md` has the check for a real boot.

Run: `cd userland/anon_onion_proofs && cargo test --release`. `nix flake check` runs it as `proofs-anon_onion_proofs` (`tools/nix/checks.nix`).

See [the Anyone onion routing capsule](../../docs/handbook/network/anyone.md) and [proofs](../../docs/handbook/verification/proofs.md).
