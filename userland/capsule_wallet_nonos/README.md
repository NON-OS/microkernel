# capsule_wallet_nonos

`app.nonos_wallet`, the NONOS wallet window: an Ethereum account on mainnet
and Sepolia that holds ETH, NOX and USDC, stakes NOX on mainnet, and pays
privately through the NOX Shield on Sepolia. It never holds the private key. The key lives in
`capsule_keyring`, which generates it from a BIP39 phrase, derives
`m/44'/60'/0'/0/0`, signs on request and seals it to the machine's TPM; the
wallet asks for each of those over IPC and keeps only the sealed blob. The
handbook page is [docs/handbook/apps/wallet.md](../../docs/handbook/apps/wallet.md),
and what 0.9.2 does flow by flow is in
[wallet-0.9.2-notes.md](../../docs/handbook/apps/wallet-0.9.2-notes.md).

## Microkernel contract

- Service `service:4734:app.nonos_wallet`, reply
  `reply:4735:endpoint.app.nonos_wallet.reply`. Two more windows can open on
  the instance endpoints `app.nonos_wallet.1` and `app.nonos_wallet.2`
  (ports 4854 to 4857).
- `CAPSULE_REQUIRED_CAPS = 0x1000187d`: CoreExec, Network, IPC, Memory, Crypto,
  FileSystem, GraphicsDisplayQuery, GraphicsSurfaceCreate and Keyring. Network is what
  the network services check; FileSystem is what vfs requires before it saves
  or reads the vault blob (`src/wallet/vault/vfs.rs`). It has no raw device
  authority.
- The kernel mirror is `src/userspace/capsule_wallet_nonos`. The capsule is
  turned on by `nonos-capsule-wallet-nonos` in `microkernel-desktop-offline`,
  so every desktop image carries it.

## What it does

- **Custody.** Generate, import and recover go to the keyring
  (`src/wallet/ipc/`). The keyring returns the 12 word indices once, for the
  backup screen, and the wallet wipes its copy.
- **Keeping a wallet.** After generate, import or recover the wallet asks the
  keyring to seal the account (`OP_VAULT_SEAL`), writes the 72-byte blob to
  `/data/wallet.vault` and asks vfs to persist it. On the next boot it reads
  the blob back and asks the keyring to open it. The keyring answers either
  request only for a sender that owns one of the wallet's endpoints.
- **Networks.** `src/wallet/chain.rs` holds Ethereum mainnet and Sepolia: the
  RPC host, the NOX and USDC contracts, staking (mainnet) and the shield pool
  (Sepolia). A switch drops every reading and review from the other network,
  closes the shield store, and is held while a broadcast or shield job runs.
- **Reads.** One JSON-RPC batch per refresh (`src/wallet/net/read_snapshot.rs`)
  over one TLS 1.3 connection, stepped a slice per tick (`src/wallet/net/step`).
  A direct socket sends at most one call frame's payload per send
  (`src/wallet/net/send_cap.rs`).
- **Signing.** A payment or a staking transaction is planned, reviewed on a
  fresh nonce, fee and gas estimate, and signed only by the review's Confirm,
  through the keyring's generic `OP_SIGN_TX` under the picked chain id
  (`src/wallet/act`, `src/wallet/send`). It is broadcast once and its receipt
  followed. No fee is signed at zero or above 1000 gwei.
- **Shield.** The shield's keys and notes live in `capsule_shield`
  (`nonos.shield`), which runs shield_core. The wallet starts its jobs and
  polls their result (`src/wallet/shield`); the screens are
  `src/wallet/screen/shield`. On mainnet the screens say the pool is not
  deployed there and offer Sepolia.
- **Network route.** Every request opens a `Link` on
  `nonos_route_link::Route::for_wallet`, over Nym (`net.socks5`) or Anyone
  (`net.anon`) with the host name unresolved, never Direct: a Direct choice
  goes over Nym, else Anyone. When neither is up the wallet says it is waiting
  for Nym or Anyone, and opens no socket of its own. The TLS 1.3 client pins the Google Trust
  Services R4 root and matches the host through `nonos_tls::cert_names_host`.

## Pool seam

Swap quotes come through one trait, `QuoteProvider` in `src/wallet/pool/`,
and `active.rs` returns `Unwired`, which answers None. Swap is therefore
hidden from the home screen in 0.9.2 (`SWAP_OFFERED` in
`src/wallet/screen/home_actions.rs`); it returns once a pool is wired there.

## Privacy and persistence

The wallet writes the sealed vault, the sealed recovery words, which accounts
are in use, and whether the wallet came from a key, all under `/data/wallet.*`.
Shielded notes are the shield service's; the account view and the shield
history live in capsule memory. Under Direct the RPC provider sees
the machine's address and the accounts queried; under Nym or Anyone it sees
the accounts.

Secrets in memory are wiped through zeroize: a key or words typed, a key
shown, the keyring's replies, and every string the screens draw from them.
One thing is kept across a lock on purpose: the recovery words of a wallet
just made, until the person confirms they wrote them down. They are the
only copy, and the phrase cannot be shown again, so losing them would lose
the wallet. While locked they stay in memory but are never drawn; unlocking
returns straight to the backup screen. They are wiped on that confirmation,
when a generate fails, and when the next generate writes over them.

## The purchase consent and what it cannot stop

A purchase the store names shows nothing until the market's four answers
are in, are for the names asked and pass `read_named`
(`src/wallet/order/terms`); an id the wallet hashes as all zeros, a hash
the crypto service did not give, is refused before any compare. The consent
then says, by the wallet's own name, whom it pays in full, and the wallet
asks to raise and focus its window; a refusal is no failure and nothing is
said of it. Buy is armed only once the consent is drawn and the window
manager says the wallet holds focus (`OP_QUERY_FOCUS`), and only after a
further second; the second starts again whenever focus is lost and won
back, and focus lost over the consent sends the purchase back to reading
its terms (`src/wallet/order/guard`). Every other press that moves value or
signs is armed the same way (`src/wallet/arm`); off the purchase, focus lost
disarms it until a fresh second has passed.

These blunt focus stealing and a fake window drawn to look like the
wallet. A click falling through an overlay that another capsule draws above
the wallet is closed by the unit T1b: the input router checks the
compositor's layer under the pointer (`OP_LAYER_AT`) against the window
manager's hit before it delivers the click. T1b also takes a client's own
raise or focus only within a second of input the router gave it, so the
wallet's own raise after the store's POST is refused (`E_PERM`) and the
dock marks the wallet for attention: bringing the wallet forward is the
person's act, and the second starts from that.

## Tests

`userland/wallet_proofs` mounts the wallet's pure helpers (amounts, scaling,
the stakeable amount, the swap curve, the route text and the routed reader,
gas and fee rules, the broadcast outcome, the snapshot request and reply on
each network, the receipt parse, the chain table and the vault replace rule)
and the keyring's key store and vault gate. `userland/shield_wire` tests the
shield calls' framing. `userland/tls_proofs` includes the
wallet's host-name check.
