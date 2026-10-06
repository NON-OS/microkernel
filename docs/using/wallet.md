# Wallet

Keep an Ethereum account on NONOS: create it or restore it from recovery words, hold ETH, NOX and USDC, send, stake NOX and use the NOX Shield, with the private key kept by the keyring and never by the wallet window.

## Read this first: recovery words

- The recovery words are the account. Whoever has them can take everything it holds, on every network, and nothing can undo that.
- NONOS shows a new wallet's words once, on the `Recovery phrase` screen, straight after the wallet is made. The screen has no back button. `I wrote them down` is the only way on, and it wipes the words from the screen.
- Write them on paper, in order. Never type them into a website, and never take a picture of them.
- The copy NONOS keeps is sealed to this machine, its firmware and its kernel. After a firmware or kernel update, or on another machine, the sealed copy does not open. A live boot keeps nothing past power off, and a failed seal keeps nothing past reboot. In each case only the words bring the account back. If you lose the words then, the account is gone. NONOS cannot recover it.
- A wallet imported from a private key has no words. Keep the key itself written down, offline, before you import it.

## Create, restore or import

The first screen offers three buttons (`welcome` in `userland/capsule_wallet_nonos/src/wallet/screen/welcome.rs:37-44`):

| Button | What happens |
|---|---|
| `Create a wallet` | The keyring draws 128 bits of entropy from the machine's mixed hardware sources and makes a 12-word BIP39 phrase (`WORDS` in `userland/capsule_keyring/src/server/handlers/wallet_generate_hd.rs:32`). The phrase is shown once. |
| `Import a private key` | Type the key as 64 hex digits, `0x` optional. Each digit shows as a dot, and the key goes straight to the keyring. |
| `Restore from recovery words` | Type 12, 15, 18, 21 or 24 words in order, separated by spaces. They show as dots unless you choose `Show the words`. The BIP39 checksum is checked before anything is derived, so a mistyped phrase is refused and never opens a wrong account (`wallet_recover` in `userland/capsule_keyring/src/server/handlers/wallet_recover.rs:32-66`). |

The words derive the account at `m/44'/60'/0'/0/0`, the standard Ethereum path, with an empty BIP39 passphrase (`account_key_at` in `userland/capsule_keyring/src/server/hd.rs:46-60`). Another wallet that uses this path and no passphrase derives the same account from the same words. A phrase that you used elsewhere with a BIP39 passphrase gives a different account here, because NONOS has no passphrase field.

`Add account` opens further accounts of the same phrase, `m/44'/60'/0'/0/1` to `m/44'/60'/0'/0/7`: eight accounts in all (`MAX_ACCOUNT_INDEX` in `userland/capsule_keyring/src/server/words_own.rs:24`).

The wallet's own `Settings` screen picks the network (`Ethereum mainnet` or `Sepolia`), says whether the key is `sealed to this machine` or `RAM only, gone at reboot`, and offers `Show the private key`, `Restore from recovery words`, `Import a private key` and `Lock the screen` (`ROWS` in `userland/capsule_wallet_nonos/src/wallet/screen/settings/mod.rs:42-47`).

## Networks and assets

One address serves two networks. You switch between them in the wallet; every signature carries the chosen chain id, so a transaction reviewed on one network cannot be replayed on the other (`MAINNET` and `SEPOLIA` in `userland/capsule_wallet_nonos/src/wallet/chain.rs:73-97`).

| | Ethereum mainnet | Sepolia |
|---|---|---|
| Chain id | 1 | 11155111 |
| ETH | yes | yes |
| NOX | `0x0a26c80be4e060e688d7c23addb92cbb5d2c9eca` | `0x3e5249a65ca513d5e11260222e0d26f46b465d36` |
| USDC | `0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48` | `0x1c7d4b196cb0c7b01d743fbc6116a902379c7238` |
| NOX staking | `0xa94d6009790ba13597a1e1b7cf4e1531ea513613` | no |
| NOX Shield | no: the screen says `The shield pool is not deployed on Ethereum mainnet` and offers `Switch to Sepolia` | yes |
| RPC hosts, tried in order | `ethereum-rpc.publicnode.com`, `mainnet.gateway.tenderly.co`, `rpc.mevblocker.io` | `ethereum-sepolia-rpc.publicnode.com`, `sepolia.gateway.tenderly.co`, `rpc.sepolia.ethpandaops.io` |

The hosts and contracts are fixed in the code; the wallet has no field to add a network, a token or an RPC host.

Sepolia is Ethereum's test network, and the wallet says so: nothing on it has value. The NOX Shield is a private pool that runs only on Sepolia in this release. The `Receive` screen puts the difference this way: paid at the private address, "the sender, the amount, your balance and the notes you hold stay out of sight"; at the `0x` address, "anyone can see what this address holds and sends" (`userland/capsule_wallet_nonos/src/wallet/screen/receive.rs`). The shield's keys and notes are kept by a separate capsule, `capsule_shield`. `Receive` shows the `0x` address, and on Sepolia the private address once the shield has opened for the account.
