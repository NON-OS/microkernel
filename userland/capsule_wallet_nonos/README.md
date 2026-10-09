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
- `CAPSULE_REQUIRED_CAPS = 0x187d`: CoreExec, Network, IPC, Memory, Crypto,
  FileSystem, GraphicsDisplayQuery and GraphicsSurfaceCreate. Network is what
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
  `nonos_route_link::Route::chosen`: under Direct, `net.dns` and a socket
  through `net.sockets`; under Nym or Anyone, `net.socks5` or `net.anon` with
  the host name unresolved. A chosen network that is not running refuses with
  its reason, and nothing falls back. The TLS 1.3 client pins the Google Trust
  Services R4 root and matches the host through `nonos_tls::cert_names_host`.

## Pool seam

Swap quotes, association sets, simulation and NOX revenue sit behind traits
in `src/wallet/pool/`, and `active.rs` returns `Stub` for each, which answers
`NotWired`. Swap is therefore hidden from the home screen in 0.9.2
(`SWAP_OFFERED` in `src/wallet/screen/home_actions.rs`); it returns once a
pool is wired there.

## Privacy and persistence

The wallet writes the sealed vault, the sealed recovery words, which accounts
are in use, and whether the wallet came from a key, all under `/data/wallet.*`.
Shielded notes are the shield service's; the account view and the shield
history live in capsule memory. Under Direct the RPC provider sees
the machine's address and the accounts queried; under Nym or Anyone it sees
the accounts.

## Tests

`userland/wallet_proofs` mounts the wallet's pure helpers (amounts, scaling,
the stakeable amount, the swap curve, the route text and the routed reader,
gas and fee rules, the broadcast outcome, the snapshot request and reply on
each network, the receipt parse, the chain table and the vault replace rule)
and the keyring's key store and vault gate. `userland/shield_wire` tests the
shield calls' framing. `userland/tls_proofs` includes the
wallet's host-name check.
