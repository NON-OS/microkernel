# G on KVM: a Linux wallet reaches its node over Anyone

The local session's lanes on the owner's KVM host (32-core Xeon, 8 GiB
guest, standard development images), on db9d7593ee: main 8e90ec93d4 with G
(44535301b5) and the spawn fix (b5bb69a407) merged in. Serial only: the
wallet's own output goes to its window, and no seed was captured.

| Log | Route | What it shows |
|---|---|---|
| `gdb9a-anyone-sync.log` | Anyone, chosen at setup's step 6 | Monero installs (`[LINUX] installed`; the install heap `of 327680 KiB`). The wallet first asked for node.moneroworld.com:18089: 57 streams `[ANON] stream ended N 2`, RESOLVEFAILED. Then `set_daemon xmr-node.cakewallet.com:18081`: 3 `[ANON] stream connected`, and on screen "Starting refresh...", "3775093 / 3793833", climbing |
| `gdb9n.log` | Nym | Monero installs the same way. The wallet asked for node.moneroworld.com:18089: 13 `[SOCKS5] exit answered bytes 67` |

## The dead name, not the route

node.moneroworld.com no longer resolves: dns.google answers NXDOMAIN
(Status 3) on 28 October 2026, with only the parent's SOA left. Each Anyone
exit said so as RESOLVEFAILED, and each Nym network requester as a
connection error of 67 bytes, which the trace logged by size alone. The
listing's own node, xmr-node.cakewallet.com:18081, resolves, and the
wallet syncs from it over Anyone.

Nym's exits carry the port: the published exit policy
(nymtech.net/.wellknown/network-requester/exit-policy.txt) accepts
`*:12000-65535`, and lists 18080, 18081 and 18089 as opened within it.

## Not shown here

- The wallet syncing over Nym from xmr-node.cakewallet.com: the lane is
  running.
- The ring database: the wallet still says "Failed to initialize ring
  database". The calls around it are being placed.
