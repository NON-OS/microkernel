# S1 G on KVM: Monero lanes, serial

Host: 32-core Xeon, KVM, 8 GiB guest, standard dev images.

- gaany.log: 44535301b5 + b5bb69a407 (test merge 17355d6b4c). Labelled
  "Anyone", but the lane script pressed one Enter too many and confirmed
  the route step on its default, Nym. Its route is Nym.
- gargv.log: the same tree, WIDTH=3, Nym. Install heap of 327680 KiB.
- gdb9n.log: db9d7593ee, normal width, Nym. Install heap of 327680 KiB,
  Monero installed; the wallet never reached node.moneroworld.com:18089
  ("exit answered bytes 67", "open refused: exit silent").

The wallet ran with --log-file /root/w.log --daemon-address
node.moneroworld.com:18089 --untrusted-daemon. Wallet output is not in
serial; seeds were never captured.
- gdb9a-anyone-sync.log: db9d7593ee, Anyone really chosen. The dead name
  gives "[ANON] stream ended N 2" (RESOLVEFAILED); then set_daemon
  xmr-node.cakewallet.com:18081 this-is-probably-a-spy-node and refresh:
  status 3775093 / 3793833, syncing.
- gcaf-nym-ringdb-fixed.log: test merge caf7aeba61 (argv-alone + land-g
  e16e16d6b4), Nym, listing node. Installed, heap of 327680 KiB; the wallet
  reopened with no ring database error and no unserved nr=274, syncing
  3171087/3793858 over SSL.
- owner-seal-694ba69799-code5.log: the owner's own release seal, real
  mirror over Nym. `linux monero-wallet-cli --version` 67 s into the
  install: [EXIT] code=1 app.linux.term, then at once [EXIT] code=5
  app.linux.install.
- gcaf2-linux-mid-install.log: caf7aeba61, Nym. `linux monero-wallet-cli
  --version` typed 15 s into `market install linux.monero`: line 1762
  "[LINUX] installing", 1779 "[EXIT] code=1 app.linux.term", 1962
  "[LINUX] installed"; heap lines of 327680 KiB.
