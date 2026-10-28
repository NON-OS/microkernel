# G on KVM: a Linux wallet reaches its node over Anyone and Nym

guest). Serial only: a terminal run's output is private, so the wallet's own
lines reach its window and never serial. What the window said is quoted from
the notes and marked as such. No seed was captured.

| Log | Tree | Route | What serial shows |
|---|---|---|---|
| `gaany.log` | test merge 17355d6b4c: 44535301b5 (G) + b5bb69a407 (spawn fix) | Nym. It was meant to be Anyone, but one Enter too many accepted the route step's default | `[LINUX] installed`, install heap `of 327680 KiB` |
| `gargv.log` | the same tree, window width 3 | Nym | the same: installed, `of 327680 KiB` |
| `gdb9n.log` | db9d7593ee: main 8e90ec93d4 + G + spawn fix | Nym | installed; the wallet asked for node.moneroworld.com:18089, answered by 13 `[SOCKS5] exit answered bytes 67` |
| `gdb9a-anyone-sync.log` | db9d7593ee | Anyone, chosen at setup's step 6 | installed; 57 `[ANON] stream ended N 2` (RESOLVEFAILED) for the dead name, then 3 `[ANON] stream connected` after `set_daemon xmr-node.cakewallet.com:18081`. Window: "Starting refresh...", "3775093 / 3793833", climbing |
| `gdb9n-nym-sync.log` | db9d7593ee | Nym, listing node xmr-node.cakewallet.com:18081 | installed; 3321 `[SOCKS5] exit answered` as the wallet syncs |
| `gdb9n-ringdb.log` | db9d7593ee | Nym | the serial around the wallet's "Failed to initialize ring database" (window). The cause was the unserved get_robust_list (274): musl's robust-mutex probe, which LMDB's lock setup needs |
| `gcaf-nym-ringdb-fixed.log` | test merge caf7aeba61: argv-alone d423fd27e7 + land-g e16e16d6b4 (get_robust_list served) | Nym, listing node | installed, `of 327680 KiB`; 2032 `[SOCKS5] exit answered`. Window, on reopening the wallet file: "Opened wallet", no ring database error, "Refreshed 3171087/3793858", syncing over SSL |

`[EXIT] code=5129556 app.linux.term` in the Nym logs is 0x4E4554, "NET":
the terminal role asking for its network twin, as designed, not a failure.

## The dead name, not the route

node.moneroworld.com no longer resolves: dns.google answers NXDOMAIN
(Status 3) on 28 October 2026, with only the parent's SOA left. Each Anyone
exit said so as RESOLVEFAILED, and each Nym network requester as a
connection error of 67 bytes, which these trees logged by size alone. G now
adds the refusal's class to that line (`[SOCKS5] exit refused: <class>`,
one of dns, policy, timeout, refused, unreachable, other), never its text.
The listing's own node, xmr-node.cakewallet.com:18081, resolves, and the
wallet syncs from it over both routes.

Nym's exits carry the port: the published exit policy
(nymtech.net/.wellknown/network-requester/exit-policy.txt) accepts
`*:12000-65535`, and lists 18080, 18081 and 18089 as opened within it.

## the release seal, before the spawn fix

On 694ba69799 a Monero install fails with or without a `linux` run. Alone
it ends `[EXIT] code=134` (the the developers's control run; on development
images that abort was a failed allocation in a 160 MiB heap). With a
`linux` run mid-install it ends code 5, on the line after the run's exit.
On main without the spawn fix the installer can read its arguments before
they are set, and size its heap for a program (160 MiB) rather than an
install (320 MiB). Every tree here with the fix installs at
`of 327680 KiB`.

## Not shown here

- A `linux` run started mid-install on a tree with the spawn fix: the
  lane is running.
- On every wallet open the first refresh fails with "net.socks5 sent an
  answer this cannot read" (window), and the next connects.
