# Monero wallet

How to use the Monero command-line wallet from the App Store: make or restore a wallet, choose the node it talks to, what that node and the network can learn, how to keep the seed, and what is kept between runs. The listing is `linux.monero`, the program `/usr/bin/monero-wallet-cli` from Alpine's `monero 0.18.3.3-r1`, and its box is TTY FILES_HOME NET_ANON ([Privacy apps](privacy-apps.md#monero-wallet-monero-wallet-cli)).

Each claim is marked as [Privacy apps](privacy-apps.md) marks them: *booted*, *host-tested* or *design*. Nothing on this page has been seen on a boot yet.

## How it starts

Install `Monero wallet (CLI)` from the App Store's Linux tab, then run it in the Terminal with `linux monero-wallet-cli` and your own options after it.

The listing starts the wallet with five options before yours:

```text
--daemon-address=xmr-node.cakewallet.com:18081
--untrusted-daemon
--log-file=/root/monero-wallet-cli.log
--max-log-file-size=1048576
--max-log-files=1
```

They are part of the release the publisher signs, and the machine checks them again before each start (`docs/userland/marketplace-catalogue.md`, Listed arguments). *host-tested*

- **No local node.** Without a node named, the wallet looks for one on this machine at `localhost:18081`, and fails with `wallet failed to connect to daemon`. NONOS runs no Monero node: `monerod` is in the package but has no box, and is refused. The listed address gives it a remote node from the start. *design*
- **A log in its home.** By default the wallet writes its log next to its program, `/usr/bin/monero-wallet-cli.log`, and the box refuses that path. The listed `--log-file` puts it in the wallet's home, `/root`, instead, at most 1 MiB and one file, so the 16 MiB home is left for the wallet's cache. *design*
- **No mining question.** The wallet offers background mining only when it trusts its node. With an untrusted node it prints `Using an untrusted daemon, skipping background mining check` and does not ask (monero's `src/simplewallet/simplewallet.cpp`, `check_background_mining`). Mining would run on the node, not here, so it has no use with a remote node. *design*
- **The setting itself.** Monero 0.18 has no command-line option to turn the setting off. In the wallet, `set setup-background-mining 0` records "no" in the wallet file, so it is not asked even if you later trust a node. *design*

## Make a wallet

```text
linux monero-wallet-cli --generate-new-wallet mywallet
```

The wallet asks for a password twice and a seed language. It then prints the 25-word seed. Write it down (see [The seed](#the-seed)). `address` shows your address, `balance` your balance, `refresh` fetches new blocks from the node, and `exit` saves and quits. The wallet's files are written in its own home. Open it again, while that home lasts, with `linux monero-wallet-cli --wallet-file mywallet`. *design*

## Restore a wallet

```text
linux monero-wallet-cli --restore-deterministic-wallet
```

The wallet asks for a new file name, the 25 words, a seed offset passphrase (empty if you set none), and a restore height. Give the height, or a date as `YYYY-MM-DD`, from just before the wallet was first used. From height 0 it reads the whole chain from the node over the anonymity network, which takes hours. `--restore-height <n>` gives the height on the command line instead. *design*

## Choose a node

- **The listed node.** `xmr-node.cakewallet.com:18081` is a public node run by a third party, chosen in the listing so the wallet works on first start. NONOS does not run it and does not vouch for it. A connection to it over Anyone or Nym has not been seen on a boot yet. *design*
- **Your own choice.** Give `--daemon-address <host>:<port>`, or `--daemon-address=<host>:<port>`, and it replaces the listed one: a listed option is left out when you name the same option yourself (`userland/capsule_linux/src/linux/app/args_listed.rs`). Write the option out in full. Only the whole name is matched, so an abbreviation leaves the listed address in place as well. *host-tested*
- **Inside the wallet.** `set_daemon <host>:<port> untrusted` switches node for the rest of the run. *design*
- **The port.** Public nodes usually serve wallets on 18081 or 18089. The exit relay of the Anyone or Nym network must allow that port. Which ports their exits allow has not been checked here. If the wallet cannot connect, try a node on another port. *design*
- **No .onion nodes.** A Tor onion address does not work. Anyone is a network of its own: its onion services end in `.anyone`, and it cannot reach Tor's. A `.onion` name goes to the exit as an ordinary name, and the exit cannot connect to it (`userland/nonos_route_link/src/pick.rs`, `is_anyone`). The Nym mixnet cannot reach Tor onion services either. So no onion nodes are listed. *design*
- **Do not use `--proxy`.** Every connection the wallet makes already leaves over Anyone or Nym, never direct (`userland/capsule_linux/src/linux/net/guest_route.rs`). `--proxy` points the wallet at a SOCKS proxy, by default on this machine, and there is none here, so every connection would fail. *design*
- **Do not use `--trusted-daemon`.** The listing already gives `--untrusted-daemon`, and the wallet refuses to start with both. A node you do not run should not be trusted anyway. *design*

## What the node and the network learn

- **Your address stays hidden.** The node sees an Anyone or Nym exit, never this machine. *design*
- **The node sees what the wallet asks for.** A remote node sees which blocks and outputs your wallet asks about and when, and the transactions you send through it. From that it can try to link them to one wallet. Running your own node is the stronger choice, and it is not possible here. *design*
- **What `--untrusted-daemon` changes.** The wallet treats the node as possibly hostile. It refuses commands that need a trusted node, such as `start_mining`. On its first refresh it tells the node less exactly which blocks it already has. It does not stop the node from seeing what the wallet asks about. *design*
- **What the exit can read.** The wallet tries TLS to the node first, and falls back to plain HTTP when the node has none (its `--daemon-ssl autodetect` default). Over plain HTTP the exit relay can read the same requests the node reads. *design*
- **Names given as addresses.** A Monero address written as a name (OpenAlias) is looked up as a TXT record, and those are not answered here. Use the full address. *design*

## The seed

- The 25 words are the wallet. Anyone who has them can spend everything in it, from anywhere.
- Write them on paper, by hand. Never take a screenshot or a photo of them, never copy them to the clipboard, and never type them into any other program or website.
- `seed` in the wallet shows them again after your password.
- Never type the seed into anything you have not checked. Restore only with `--restore-deterministic-wallet` in this wallet.

## A live session and a kept home

- **Today: kept until power off.** The wallet writes its files in its own home. That home is kept in memory from one run to the next until the machine powers off, and erased when the app is uninstalled. No other Linux program sees it. *booted* for other apps' homes ([Privacy apps](privacy-apps.md#what-every-networked-app-here-shares)); not yet for this wallet.
- **After power off, nothing is left.** A home kept across a reboot is designed (Kept homes and installs) and not in yet. Until it is, the seed on paper is the only copy that survives. After each boot, restore the wallet from the seed, with a restore height so it does not read the whole chain.
- **A kept home, when it comes.** It will keep the wallet file on a disk installed with keeping chosen at setup, sealed to this machine. The seed stays the copy that survives losing the machine.
