# G on KVM: a Linux wallet reaches its node over Anyone and Nym

The runs on the owner's KVM host (32-core Xeon, 8 GiB
guest). Serial only: a terminal run's output is private, so the wallet's own
lines reach its window and never serial. What the window said is quoted from
the maintainers' notes and marked as such. No seed was captured.

On development images the installer fetches Alpine's packages from the
host's relay, dialled directly (docs/build/dev-mirror.md). So every
"installed" below on a development image says nothing about fetching from
the real mirror over a network; the `diag-` logs and the owner's seal do.
The route column is the wallet's.

| Log | Tree | Route | What serial shows |
|---|---|---|---|
| `gaany.log` | test merge 17355d6b4c: 44535301b5 (G) + b5bb69a407 (spawn fix) | Nym. It was meant to be Anyone, but one Enter too many accepted the route step's default | `[LINUX] installed`, install heap `of 327680 KiB` |
| `gargv.log` | the same tree, window width 3 | Nym | the same: installed, `of 327680 KiB` |
| `gdb9n.log` | db9d7593ee: main 8e90ec93d4 + G + spawn fix | Nym | installed; the wallet asked for node.moneroworld.com:18089, answered by 13 `[SOCKS5] exit answered bytes 67` |
| `gdb9a-anyone-sync.log` | db9d7593ee | Anyone, chosen at setup's step 6 | installed; 57 `[ANON] stream ended N 2` (RESOLVEFAILED) for the dead name, then 3 `[ANON] stream connected` after `set_daemon xmr-node.cakewallet.com:18081`. Window: "Starting refresh...", "3775093 / 3793833", climbing |
| `gdb9n-nym-sync.log` | db9d7593ee | Nym, listing node xmr-node.cakewallet.com:18081 | installed; 3321 `[SOCKS5] exit answered` as the wallet syncs |
| `gdb9n-ringdb.log` | db9d7593ee | Nym | the serial around the wallet's "Failed to initialize ring database" (window). The cause was the unserved get_robust_list (274): musl's robust-mutex probe, which LMDB's lock setup needs |
| `gcaf-nym-ringdb-fixed.log` | test merge caf7aeba61: argv-alone d423fd27e7 + land-g e16e16d6b4 (get_robust_list served) | Nym, listing node | installed, `of 327680 KiB`; 2032 `[SOCKS5] exit answered`. Window, on reopening the wallet file: "Opened wallet", no ring database error, "Refreshed 3171087/3793858", syncing over SSL |
| `gcaf2-linux-mid-install.log` | caf7aeba61 | Nym | `linux monero-wallet-cli --version` typed 15 s into `market install linux.monero`: `[LINUX] installing` (line 1762), `[EXIT] code=1 app.linux.term` (1779), `[LINUX] installed` (1962), heap `of 327680 KiB`. A `linux` run during an install leaves it to finish |
| `owner-seal-694ba69799-code5.log` | the owner's release seal of 694ba69799, real mirror | Nym | `linux monero-wallet-cli --version` typed 67 s into the install: `[EXIT] code=1 app.linux.term` (line 994), then `[EXIT] code=5 app.linux.install` (995) |
| `owner-seal-694ba69799-no-typing-134.log` | the same seal, nothing typed | Nym | `[EXIT] code=134 app.linux.install` (line 990) |
| `diag-694ba69799-realmirror-nym.log` | development image of 694ba69799 with the dev mirror off: the real dl-cdn.alpinelinux.org, serial debug on | Nym chosen; the install goes over Anyone | both indexes whole (473606, 1848333 bytes) on one stream; then 6 package GETs on streams opened three at a time, every one "gave no whole 200 reply"; `[LINUX-TIME] packages 6 gets … 246.7 s 0.00 MB/s`; the pin refuses the empty package, code 5 |
| `diag-694ba69799-realmirror-anyone.log` | the same image | Anyone | the same indexes; streams 5, 6, 7 opened at once and 8, 9 beside them: five GETs with no whole reply; stream 10 alone: `monero-0.18.3.3-r1.apk, 24698520 bytes`, then code 134 in the 160 MiB heap |

`[EXIT] code=5129556 app.linux.term` in the Nym logs is 0x4E4554, "NET":
the terminal role asking for its network twin, as designed, not a failure.

## Why a real-mirror install failed: one stream per caller

Every install goes over Anyone (`Route::for_installs`), whatever route was
chosen. route_link's frames name no stream, and both proxies key such a
conversation on the caller's pid: "a capsule holds one stream through each
at a time: opening one ends the last" (`userland/nonos_route_link/src/tunnel.rs`).
The install window (fast downloads) opened three streams for one
installer, so each took the conversation from the one before and no
package GET was answered; each waited out its 120 s. A lone stream carried
24.7 MB. The window now holds one GET at a time and reuses its kept stream
(`get/window_room.rs`, WIDTH 1, proven in `capsule_linux_proofs`
`install_window_tests.rs`, where WIDTH 2 fails two tests). Several at once
waits for route streams that name themselves, which both proxies already
read (markers 3 and 4).

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

## The owner's seal, before these fixes

On 694ba69799 a Monero install failed two ways. The window's parallel
streams got no package (code 5, above). And without the spawn fix the
installer could read its arguments before they were set and size its heap
for a program (160 MiB) rather than an install (320 MiB): alone it ended
`[EXIT] code=134`. Every tree here with the spawn fix installs at
`of 327680 KiB`.

## Not shown here

- A real-mirror install over Nym and Anyone with one GET at a time: the
  maintainers's runs are going.
- On every wallet open the first refresh fails with "net.socks5 sent an
  answer this cannot read" (window), and the next connects.
