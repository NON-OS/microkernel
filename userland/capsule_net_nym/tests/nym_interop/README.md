# Nym interop oracle

Builds packets, reply blocks and repliable messages with net.nym's own source
(pulled in by `#[path]`, crypto stood in for as in `../live_gateway`), then has
the code the network runs read them: sphinx-packet 0.6.0, which nym-node 1.41.0
locks, unwraps every hop, and nym-sphinx-anonymous-replies 1.22.1 parses the
messages and uses the reply blocks the way a network requester does.

Offline: no gateway is dialled. It needs the crates from crates.io, so it is
not part of the offline gate.

    cargo run --release

Every line should read PASS and the last `ALL PASS`. Run it whenever the
packet version, the reply block layout or the message tags change, and pin the
two reference crates to what the network's nodes run at the time.
