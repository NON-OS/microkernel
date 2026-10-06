# nonos_route_link

`nonos_route_link` decides which network a capsule's own connections leave
through, and carries a byte stream over it. The network is the system's
default, which setup asks for and Settings changes, read from the policy
store. One rule and one client, so the model fetcher, the terminal, the wallet,
the Linux personality, the time client and SDK programs cannot disagree about
it. It is `no_std`.

## The rule

`Route::chosen()` reads `Field::NetworkRoute` with `nonos_policy_client`
(`default_route`) and looks up `net.socks5` and `net.anon`, every time it is
called, so a choice changed in Settings holds from the next connection.
`pick(default, nym_port, anon_port)` (`src/pick.rs`) turns that into a `Route`:

- `Direct` only when the default is Direct.
- `Anon(port)` when the default is Anyone and `net.anon` runs.
- `Nym(port)` when the default is Nym, unknown or unreadable and `net.socks5`
  runs.
- `Down(reason)` when the chosen network is not running, or the default is
  unreadable and the mixnet is not running.

Nothing falls back to another network, and never to Direct, which names this
machine to the far end. `Route::name`, `label`, `patience_ms` (60 s for Nym,
30 s for Anyone, nothing extra for Direct) and `is_anonymous` describe a route
to the person and to readers.

`direct_refusal()` answers for a contact that cannot cross an anonymity network
at all, a time server, an ICMP echo, a name looked up in the clear: the reason
it must not be made, or `None` when Direct is the default
(`src/direct_only.rs`). `net.ntp.client` and the terminal's `ping`,
`nslookup` and `nox pull` ask it.

## The stream

`RouteStream::connect(route, host, port)` (`src/stream.rs`):

- `Direct`: a `nonos_socket::TcpStream` through `net.sockets`.
- `Nym` and `Anon`: a `Tunnel` (`src/tunnel.rs`) to `net.socks5` or `net.anon`
  over a bounded IPC call (`src/ipc.rs`). It sends a reset, then the SOCKS5
  greeting and a CONNECT naming the host, so the exit resolves the name and
  this machine never looks it up. Stream bytes go in numbered frames
  (`src/frame.rs`): a marker, a `u32` number and the bytes; a frame whose
  answer was lost is sent again unchanged and the proxy gives back the answer
  it kept.
- `Down(reason)`: an error with the reason.

Every wait and buffer is bounded (`src/bounds.rs`): an open within
`OPEN_MS`, 45 s; polls of 2 s; sends of 15 s; three tries per frame; at most
`PENDING_MAX`, 1 MiB, of unread bytes. A refused CONNECT, a full proxy, a
silent one or a garbled answer each becomes a sentence a person can act on
(`src/refusal.rs`). Both proxies key an unnamed conversation on the caller's pid,
so a capsule using these frames holds one stream through each at a time; dropping a
stream resets it. (The proxies also take frames that name a stream, which the
browser uses to hold several.)

## Who uses it

`capsule_model_fetch`, `capsule_terminal` (its `Wire`, `git clone` and `git push`
over HTTPS, and the direct-only commands), `capsule_wallet_nonos` (RPC),
`capsule_linux` (a guest's stream outside its family, which Direct still keeps
on the mixnet), `capsule_net_ntp` and the SDK's `nonos_std` networking.

## What it does not do

- It does not enforce the choice below its callers. A capsule holding Network
  can still open a direct socket on `net.sockets`.
- No datagrams and no listeners over Nym or Anyone.
- The browser keeps its own reader's choice and its own non-blocking proxy
  conversations, and takes from this crate `for_host` (an `.anyone` address
  goes through `net.anon` or is refused with `ANYONE_NEEDED`), the short-name
  test and `Proxy`'s words for a refused `.anyone` CONNECT.

## Capabilities

A library adds no capability of its own. A capsule using it needs IPC to make
the calls and Network to reach `net.sockets`, `net.socks5` and `net.anon`.

## Tests

`userland/route_link_proofs` includes the pure parts by `#[path]`: the pick for
every default and every network up or down, the direct-only rule and
`net.ntp`'s decision built on it, the frames, the reading of the proxies'
answers, SOCKS5, and the tunnel itself against a proxy that loses, delays,
splits and garbles its answers on purpose. `model_fetch_proofs` and
`capsule_socks5_proofs` include `src/pick.rs` too.

See [the SOCKS5 bridge and network routes](../../docs/handbook/network/socks5-and-routes.md).
