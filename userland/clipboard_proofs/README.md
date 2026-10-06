# clipboard_proofs

Host-runnable proofs for the clipboard service. Every app copies and
pastes through it, and any process holding the endpoint can send it any
bytes. Everything between the receive and the reply is pure, so the
protocol, the router and its handlers, the reply builder and the history
are the real source, included through `#[path]` from
`../capsule_clipboard/src/`.

`route_tests` sends any frame through `route`, the function the loop
hands every received frame to, and checks it returns a whole reply for
every input. `server.rs` gives the included files the
module tree they expect.

Run: `cargo test` in this directory. The clipboard is described in
[System apps and services](../../docs/handbook/apps/system-apps.md).
