# How an install reaches a mirror

Every package fetch leaves through the network the person chose: the default
network setup asks for and Settings changes, as `Route::chosen` reads it
(`nonos_route_link`). That is the Nym mixnet by default, the Anyone onion
network when it is chosen, and a direct connection only when Direct is the
choice. The fetch is `install/http_route.rs`, the same `RouteStream` every
capsule holding Network leaves by.

Decided 2026-10-03, replacing the rule that every fetch went over the mixnet
whatever the setting, to a mirror given by a fixed address:

- **The mirror is named.** Alpine's mirror is `dl-cdn.alpinelinux.org`, and
  the name is resolved where the connection leaves: at the Nym exit, at the
  Anyone exit, or by this machine's resolver only when Direct is the choice.
  So an install names nothing in the clear that the person did not choose to.
- **No fallback.** A chosen network that is not running is refused before
  anything is fetched (`http::reachable`, exit `NoNetwork`), and a stream it
  cannot open is refused with its reason. No failure is answered by trying
  another network, which would name this machine to the mirror.

One exception stays, decided 2026-09-27: a mirror on a private or link-local
address (10/8, 172.16/12, 192.168/16, 169.254/16) is dialled directly,
whatever the chosen network. An exit is on the public internet and cannot
reach such an address, and a LAN or offline mirror is how a machine without
a reachable network, or a site with its own mirror, installs at all. Each
such fetch is logged (`[LINUX] mirror <ip> is on the local network: reached
directly`).

What the direct exception discloses: to anyone on that local network, that
this machine fetched from that mirror, and which files. Nothing reaches the
public internet directly on that path.

What none of this changes: every file is still held to its distribution's
signature and checksums, and the package the person chose to its market
pin, whichever path fetched it.

The decisions are pure and pinned in capsule_linux_proofs: which addresses
count as local (`net/route.rs::is_local`, `route_tests.rs`), which mirror and
Host line an image uses (`install/mirror.rs`, `mirror_tests.rs`), when an
install may start, and how a reply is read off a routed stream
(`install/route_read.rs`, `route_read_tests.rs`).
