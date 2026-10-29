# DarkFi

First draft (S3, 2026-10-28). DarkFi is an anonymous peer-to-peer network with its own chain, wallet and chat. NØNOS lists four DarkFi programs in the Marketplace, all built here from DarkFi v0.5.0 (commit `2e2de47c…`) without tor or i2p. Each runs in its own Linux box. Nothing on this page has been booted yet: each claim is *host-tested* (the listing and its content) or *design*.

## What is listed

The command-line apps open in the Terminal:

| Listing | What it is |
|---|---|
| `linux.nonos-darkfi-drk` | **drk**, the wallet. It keeps its keys in its home and asks a darkfid node over JSON-RPC, one you run or a remote one you name. A remote node never learns your address, but it sees which coins your wallet asks about. |
| `linux.nonos-darkfi-darkirc` | **darkirc**, the peer-to-peer IRC daemon. Peers see the channels you join and when you talk. An encrypted channel hides what is said only from those without its key. |
| `linux.nonos-darkfi-darkfid` | **darkfid**, the full node, for a **local or dev chain only** (`-n localnet`). A mainnet or testnet chain does not fit an app's 16 MiB home. Its box may JIT, as wasmer does for each contract and RandomX for its proof of work. |

The chat app opens in its own window:

| Listing | What it is |
|---|---|
| `linux.nonos-darkfi-app` | **DarkFi (chat app)**, DarkFi's desktop app (`bin/app`), with DarkIRC built in. It is software-rendered: mesa's softpipe draws on the CPU, with no GPU, so it is slower than on other systems. Copy and paste works inside the app, not with other apps. |

## Install them

They come from the NØNOS package mirror, so they install only on an image that has one: today that is a development boot with `tools/nonos-mirror-relay --content <dir>` serving the content ([the development mirror](../build/dev-mirror.md)). No release profile lists a source recipe yet, so a release image does not carry these listings. On such an image, from the Terminal:

```
market install linux.nonos-darkfi-app
market install linux.nonos-darkfi-drk
market install linux.nonos-darkfi-darkirc
market install linux.nonos-darkfi-darkfid
```

or choose them in the Marketplace's Linux tab. Each is pinned by the BLAKE3 of its content, which the listing signs, and the device checks it before unpacking ([the catalogue](../userland/marketplace-catalogue.md#apps-built-from-source)). *host-tested* (the listings); *design* (the install)

**The boxes.** drk and darkirc get a terminal (TTY), their own home (FILES_HOME) and outbound streams over the anonymity network only (NET_ANON). darkfid adds a listener on its own loopback (NET_LISTEN_LO) and JIT. The chat app gets a window (WAYLAND), its home and NET_ANON. None of them sees your other files. *host-tested* (the policies)

**Between boxes.** drk reaches darkfid's `127.0.0.1:8240` only if the two share a loopback, and darkirc alone has no IRC client in its box. *design*: how boxes share a loopback is not settled yet.

## A live session or a kept home

- **Live, the default.** A home lasts until the machine powers off. Then the wallet's keys, the node's chain and the chat history are gone, for good.
- **Kept.** On a machine installed to disk, with keeping chosen at setup, a home is kept across reboots, sealed under this machine's key ([kept homes](../design/kept-homes-and-installs.md)). *design*: kept homes are not in yet.
- **The size limit.** A kept home holds 16 MiB and 128 names in 0.9.3. A local chain or a long chat history can pass that.

## How it reaches the network

Every peer is dialled over Anyone, or Nym when that is the chosen network, never direct, and a failure never falls back to the clear net. A seed's name is resolved at the exit, never in the clear. The tor and i2p transports are built out. *design*

**Pending (S1).** DarkFi's seeds listen on port 18340 (`tcp+tls://lilith0.dark.fi:18340` and `lilith1`). A listed box's outbound stream to that port does not complete until S1's guest exit-port fix is in. Until then the apps install and start, and darkfid can run its own local chain, but drk, darkirc and the chat app reach no peer.

## Where this comes from

- The recipes: `userland/capsule_market/linux-catalogue.json`; their pins: `linux-catalogue.sources.lock.json`.
- The builds: `tools/nix/sources/darkfi-*.sh`, and the letters `HANDOFF/letters/2026-10-27-s3-darkfi-build.md` and `2026-10-28-s3-darkfi-gui.md`.
