# DarkFi

S3, 2026-10-29. DarkFi is an anonymous peer-to-peer network with its own chain, wallet and chat. NØNOS lists four DarkFi programs in the Marketplace, all built here from DarkFi v0.5.0 (commit `2e2de47c…`) without tor or i2p. Each runs in its own Linux box. Each claim is marked *booted*, *host-tested* (the listing and its content) or *design*.

## What is listed

| Listing | What it is |
|---|---|
| `linux.nonos-darkfi-darkirc` | **DarkIRC (P2P chat)**, in the Terminal: DarkFi's `darkirc` daemon with suckless `sic`, a small IRC client, in the same box. |
| `linux.nonos-darkfi-drk` | **DarkFi wallet (drk)**, in the Terminal: a prompt that runs DarkFi's `drk` once per line, with a darkfid node you name. |
| `linux.nonos-darkfi-darkfid` | **DarkFi local dev chain**, in the Terminal: darkfid on its localnet, the minerd that mines it, and the drk prompt, in one box with no network. |
| `linux.nonos-darkfi-app` | **DarkFi (chat app)**, DarkFi's desktop app (`bin/app`), with DarkIRC built in. It opens in its own window. It is software-rendered: Mesa's softpipe, bundled in its content, draws on the CPU with no GPU, so it is slower than on other systems. Copy and paste works inside the app, not with other apps. |

**Not on NØNOS: a testnet or mainnet node.** A home holds 16 MiB, and a chain that syncs from its peers does not fit. The wallet asks a node elsewhere; the local dev chain is a chain of its own.

## The DarkFi wallet

Opening it runs `nonos-drk` (tools/nix/sources/drk-prompt.c):

1. **A passphrase.** The first run asks for the wallet's passphrase, twice, 8 characters or more. It writes drk's own config at the commit into the home, `/root/.config/darkfi/drk_config.toml` (mode 0600), with that passphrase in place of drk's default `changeme`, which drk refuses. drk's `fun`, a sound for each payment, is off, as a box has no audio.
2. **The wallet.** It makes the wallet (`wallet --initialize`), its first key (`wallet --keygen`), and makes that the default address.
3. **The prompt.** `drk testnet>` runs drk once per line: `wallet --address`, `wallet --balance`, `scan`, `transfer …`. One `|` joins two commands, as in `transfer 1 DRK <address> | broadcast`.
4. **The node.** `:node tcp://host:port` (or `tcp+tls://`) names the darkfid it asks, a node you run elsewhere, reached over Anyone. It never learns your address, but it sees which coins your wallet asks about. Until one is set, a command that needs a node says the connection was refused.

| Type | To |
|---|---|
| `:node tcp://host:port` | name the darkfid to ask |
| `:help` | drk's own help, and these |
| `:quit` | leave |

*host-tested*: in a chroot, the passphrase is asked twice, the wallet and its key are made, `wallet --balance` answers, and `:node` sets the testnet endpoint. The home is 144 KB. *design*: on a device.

## The DarkFi local dev chain

Opening it runs `nonos-darkfi-localnet` (tools/nix/sources/darkfi-localnet.c):

1. **The wallet first.** It is made as above, on localnet, so darkfid's config in the home names its address as the recipient of what is mined. Its drk, prompt and drk config are its own, under `/usr/lib/nonos-darkfi-localnet`, so it installs beside the wallet listing, which owns `/usr/bin/drk`.
2. **The chain.** It starts minerd with its config at the commit, then `darkfid -n localnet`. Both listen on the box's own loopback only: darkfid's RPC on 8240, minerd on 28467.
3. **The prompt.** Once darkfid's RPC answers, it runs `drk localnet>`. `scan` reads the mined blocks into the wallet, and `wallet --balance` shows the DRK mined so far.
4. **The end.** `:quit` stops darkfid and minerd.

**Its box.** A terminal (TTY), its home (FILES_HOME), listeners on its own loopback (NET_LISTEN_LO), and JIT, as wasmer compiles each contract and RandomX the proof of work. It has no network at all: the chain, and its coins, are this box's alone.

**How long a home lasts.** The chain starts at about 5.7 MB and grows about 20 KB a block. A 16 MiB home holds some hundreds of blocks, a few hours. Live, the default, it is gone at power off.

*host-tested*: in a chroot, 6 blocks were mined in 5 minutes on a slow CPU, `scan` found each reward, `wallet --balance` showed 120 DRK, `:quit` stopped both daemons, and the home was 5.8 MB; it also runs with the wallet listing's files beside it. *booted* on KVM for the other three DarkFi listings and SimpleX; this one was refused there before it carried its own drk (another installed package held `/usr/bin/drk`). *design*: on a device.

## DarkIRC in the Terminal

Opening it runs `nonos-darkirc` (tools/nix/sources/darkirc-launch.c):

1. **A nick.** The first run asks for one, letters and digits, and keeps it in the home, `/root/.config/nonos-darkirc/nick`.
2. **darkirc.** It starts with the config its content carries, `/usr/share/nonos-darkirc/darkirc_config.toml`, which is DarkFi's own default at the commit. So it does not stop to write one, as darkirc alone does on a first run. It listens on the box's loopback only: IRC on 6667, its RPC on 26660. No other box reaches either.
3. **sic.** Once darkirc's IRC port answers, sic connects to it, and darkirc joins you to its default channels (`#dev`, `#random` and the rest). The history fills in as the channels sync from peers.

| Type | To |
|---|---|
| `:j #dev` | join a channel |
| `:s #dev` | send plain lines there |
| `:m #dev text` | say something in a channel |
| `:l #dev` | leave a channel |
| Ctrl-D | quit. darkirc stops with sic. |

**Its home stays small.** darkirc as DarkFi ships it builds two RLN proving keys on its first start and keeps them, about 275 MB, far past an app's 16 MiB home. They would only sign messages for an RLN identity, which is not set by default, and darkirc sends without one, so the NØNOS build makes neither (darkfi.patch). The home holds about 1.2 MB after a first run.

*host-tested*: the launcher, darkirc and sic run together in a chroot, sic gets the welcome and the channels' topics, a line sent to `#dev` comes back, darkirc stops with sic, and the home is 1.2 MB. *design*: on a device.

## Install them

They come from the NØNOS package mirror. On the 0.9.3 release image that is `http://mirror.nonos.software/linux/<blake3>.tar`, fetched over Anyone. On a development boot it is `tools/nonos-mirror-relay --content <dir>` ([the development mirror](../build/dev-mirror.md)). Plain HTTP is enough: each listing's signature pins the content's BLAKE3, which the device checks before unpacking ([the catalogue](../userland/marketplace-catalogue.md#apps-built-from-source)). From the Terminal:

```
market install linux.nonos-darkfi-app
market install linux.nonos-darkfi-darkirc
market install linux.nonos-darkfi-drk
market install linux.nonos-darkfi-darkfid
```

or choose them in the Marketplace's Linux tab. *host-tested* (the listing and the mirror's bytes); *design* (the install)

**The boxes.** The chat app gets a window (WAYLAND), its own home (FILES_HOME) and outbound streams over the anonymity network only (NET_ANON), and listens on nothing. DarkIRC gets a terminal (TTY), its home, NET_ANON, and a listener on its own loopback (NET_LISTEN_LO), for sic. Neither sees your other files. *host-tested* (the policies)

**The app's drawing.** EGL finds the app's own Mesa driver at `/usr/lib/nonos-darkfi-app/mesa/lib/dri`, built into the driver path. The Linux box names a driver path only where the store has `/usr/lib/nonos-mesa`, so it never points the app away from its own. *host-tested*

## A live session or a kept home

- **Live, the default.** A home lasts until the machine powers off. Then the chat history and the keys are gone, for good.
- **Kept.** On a machine installed to disk, with keeping chosen at setup, a home is kept across reboots, sealed under this machine's key ([kept homes](../design/kept-homes-and-installs.md)). *design*: kept homes are not in yet.
- **The size limit.** A kept home holds 16 MiB and 128 names in 0.9.3. A long chat history can pass that.

## How it reaches the network

The chat app and DarkIRC dial DarkFi's seeds, `tcp+tls://lilith0.dark.fi:25551` and `lilith1`, then the peers they name. Every peer is dialled over Anyone, or Nym when that is the chosen network, never direct, and a failure never falls back to the clear net. A seed's name is resolved at the exit, never in the clear. *design*

The wallet dials only the node you name, and the local dev chain dials nothing. Port 25551 is not in every exit's policy. net.anon draws a circuit to an exit that takes the stream's port, and sends a refused open once more to another exit. The tor and i2p transports are built out. *host-tested* (net.anon's exit-port proofs)

## Where this comes from

- The recipes: `userland/capsule_market/linux-catalogue.json`. Their pins: `linux-catalogue.sources.lock.json`.
