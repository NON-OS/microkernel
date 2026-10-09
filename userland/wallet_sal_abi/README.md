# wallet_sal_abi

`nonos_wallet_sal_abi` (directory `wallet_sal_abi`) defines the IPC interface
of a Salvium wallet capsule: the opcodes `OP_CREATE` to `OP_UNLOCK` (create,
restore from seed or keys, open sealed, locking and reopening a session, address, balance,
sync step, history, build, sign and broadcast a send, proof, status, close),
the wire framing, the limits, and the types those operations carry
(`SalWalletId`, `SalAddress`, `SalBalance`, `SalTxDraft`, `SalProof` and the
rest). It is `no_std` with `unsafe` forbidden.

Nothing implements it. No crate in this tree depends on it, no Salvium wallet
capsule exists, and the keyring lists the Salvium rail as reserved. The NONOS
wallet is described in
[docs/handbook/apps/wallet.md](../../docs/handbook/apps/wallet.md).

The crate has no tests.
