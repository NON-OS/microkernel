# How an install reaches a mirror

Every package fetch goes over the Nym mixnet, so what this machine installs
is not visible to the network it sits on, nor to the mirror as coming from it.

One exception, decided 2026-09-27: a mirror on a private or link-local
address (10/8, 172.16/12, 192.168/16, 169.254/16) is dialled directly. A
mixnet exit is on the public internet and cannot reach such an address, and a
LAN or offline mirror is how a machine without a reachable mixnet, or a site
with its own mirror, installs at all. Each such fetch is logged
(`[LINUX] mirror <ip> is on the local network: reached directly`).

What that discloses: to anyone on that local network, that this machine
fetched from that mirror, and which files. Nothing reaches the public
internet directly on this path. What it does not change: every file is still
held to its distribution's signature and checksums, whichever path fetched it.

The decision is `net/route.rs::is_local`; the proof crate pins which addresses
count, including that anything not a plain dotted quad stays on the mixnet.
