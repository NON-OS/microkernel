# shield_core

The NOX Shield wallet core the `nonos.shield` capsule runs: NON-OS/shield-core
at `210adf4`, the Rust core both phone apps run, taken whole with its tests
and pinned vectors, so the capsule makes the same bytes the phones make.

What differs from the phones:

| Phones | Here |
|---|---|
| `net::tor`: an embedded Tor (arti, tokio, rustls) | `net::tor` keeps the same interface and carries each stream over the network NONOS has chosen, Nym or Anyone, through `nonos_route_link`, with TLS 1.3 from `nonos_tls`. Direct is refused, never fallen back to |
| `.onion` landers | out of reach on NONOS; the lander list there is the landers' four `.anyone` services, in the STARK lane's order, which nonos_route_link sends through net.anon whatever the default network |
| uniffi bindings | none: the capsule calls the Rust API |
| Sepolia reads from publicnode first | Tenderly's gateway first, then ethpandaops, then publicnode: the STARK lane found publicnode returns partial logs. A history short of `nextLeafIndex()` is refused on any server, as before |
| `ffi::proxy` (the phone's SOCKS probe), `net::socks5`, `net::endpoint`, `net::policy` and their test | removed: there is no proxy to probe |
| a spend proves with no progress reported | `prove_spend` passes `nox_prover`'s own progress callback to the cancel token, and `CancelToken::progress` gives the last phase finished and the fraction done, for the wallet's bar. The prover is unchanged |

The prover is STARKs' `nox_prover`, unchanged, with `parallel` and
`not_before`, at the commit the flake pins.

## Tests

    cargo test --release                       # shield-core's unit and integration tests
    PROD_VECTORS=<starks>/spec/wallet-vectors-not-before PROD_VECTORS_OUT=<dir> \
      cargo test --release --test prod_vectors -- --ignored

On 3 October 2026 on the host: every test passes, and the four pinned
production vectors prove byte for byte, the proof, its format 7 form and the
single-call layout (241 s on 4 cores).
