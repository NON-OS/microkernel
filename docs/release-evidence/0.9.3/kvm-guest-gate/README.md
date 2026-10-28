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
