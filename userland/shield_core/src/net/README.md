# Network

Every byte that leaves the device: the embedded Tor client, TLS, JSON-RPC and the lander. The full
transport rules are in [03-transport.md](../../../docs/03-transport.md). What is checked and what is not is in [20-security-status.md](../../../docs/20-security-status.md).

## What this owns

Every byte that leaves the device. It owns the Tor client, the TLS on top of it, the JSON-RPC
calls and the strict reading of their replies, the pools the wallet knows (`pool.rs`), the assets
each pool holds (`asset.rs`), the fee a spend pays, read from the policy of the pool and split into
its protocol part and its gas rung (`fee_schedule.rs`, `fee_quote.rs`), the pool error selectors
(`pool_errors.rs`), the lander onion service (`relay/`), and the block explorer links a person can
open (`explorer.rs`).

It owns one rule that is not a setting: a request leaves the device through the Tor client in this
module, or it does not leave. There is no direct connection to fall back to, and no proxy app.

## What it trusts

Tor, as Arti, the Rust implementation, built into the core (`tor/mod.rs`). It bootstraps from the
live consensus, checks the signatures of the directory authorities itself, and keeps its guards in
the app directory so they persist between launches. Each purpose has its own isolated circuits,
which no other purpose shares:

| Purpose | What travels on it |
|---|---|
| `Scan` | reads of the pool history |
| `Relay` | the publication of a proof to the lander, and its status |
| `Deposit` | reads of pool state that name the depositing address |
| `Mainnet` | the public account on Ethereum mainnet |
| `Sepolia` | the public account on Sepolia |

A connection, a read or a write that makes no progress for 30 s is given up and reported, and the
caller moves to its next server.

TLS end to end over Tor (`tor/tls.rs`, rustls), with each server certificate checked against the
bundled web PKI roots, so a Tor exit sees ciphertext.

Nothing about any RPC. Replies are parsed strictly (`rpc/response.rs`): a reply carrying an error is
refused, a field that is not the expected shape is refused, and a reply above 16 MiB is refused
before it is held. The pool history is read in windows of 10,000 blocks from the deploy block, and
a history with a hole is refused whole. The shield reads from
`ethereum-sepolia-rpc.publicnode.com` and then `rpc.sepolia.ethpandaops.io`.

## What it hands its neighbours

To `wallet` and `evm`: TLS streams to a named host on the circuits of a named purpose, the head of
the chain, pages of logs, and the results of view calls.

To the apps, through `ffi`: every call that reaches the network is a wallet call that needs an
unlocked session, with a single exception, described next, which touches only loopback.

The directory `socks5/` and the files `endpoint.rs`, `policy.rs` and `onion.rs` are the SOCKS5
client and endpoint policy for a platform that reaches the network through a proxy app. They remain, with their tests, behind `proxy_answers` in `ffi/proxy.rs`, which probes a
proxy port on loopback and nothing else. No screen of either app calls it, and no request of the
wallet travels over them.

## What an attacker who controls a neighbour can do

**A hostile RPC.** It sees a connection from a Tor exit, not from the device. For the pool scan it
sees the same requests every wallet sends, for every event of the pool. For the public account it
sees reads that name the account, on circuits kept apart from the scan and from the other network.
It can lie, serve a wrong head or withhold events. A history with a hole is refused, a reply for
another chain is refused before anything is read or signed (`NetError::WrongChain`), and NOX fees
are read from two servers that must agree. What remains is a denial of service.

**A hostile lander.** It holds what the chain will show once the spend settles, a little earlier.
It can refuse or sit on a spend. The wallet publishes it again after 15 minutes and offers the owner
to settle it after 30. Its replies are read strictly (`relay/reply.rs`), and its refusal reasons
reach a screen as printable text of at most 200 characters.

**A hostile policy reply.** A schedule that is not set, a word wider than its field, or a fee the
policy refuses ends the quote before any proving. Nothing is guessed from a malformed reply.

**A hostile Tor relay.** A guard sees that the device uses Tor, and when, and how much. An exit sees
a TLS connection to a public RPC and the volume of it. No relay sees the content.

**A network observer.** They see a Tor guard connection and its timing and volume, which reveals
that a privacy tool is in use and roughly when. A scan on the schedule the apps keep, every 300 s
with 60 s of jitter either side, says the app is open and not that a person picked it up.

**A DNS server.** Nothing. Host names are resolved at the Tor exit, never on the device.
