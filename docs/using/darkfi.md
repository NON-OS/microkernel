# DarkFi

S3, 2026-10-29. DarkFi is an anonymous peer-to-peer network with its own chain, wallet and chat. NØNOS lists two DarkFi programs in the Marketplace, DarkFi's chat app and DarkIRC in the Terminal, both built here from DarkFi v0.5.0 (commit `2e2de47c…`) without tor or i2p. Each runs in its own Linux box. Each claim is marked *booted*, *host-tested* (the listing and its content) or *design*.

## What is listed

| Listing | What it is |
|---|---|
| `linux.nonos-darkfi-darkirc` | **DarkIRC (P2P chat)**, in the Terminal: DarkFi's `darkirc` daemon with suckless `sic`, a small IRC client, in the same box. |
| `linux.nonos-darkfi-app` | **DarkFi (chat app)**, DarkFi's desktop app (`bin/app`), with DarkIRC built in. It opens in its own window. It is software-rendered: Mesa's softpipe, bundled in its content, draws on the CPU with no GPU, so it is slower than on other systems. Copy and paste works inside the app, not with other apps. |

**Not listed: drk and darkfid.** DarkFi's command-line wallet and node are built (the same commit) but not listed. Each stops on its first run after writing its default config, and drk needs a darkfid node, while a full chain does not fit an app's home.

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

*host-tested*: the launcher, darkirc and sic run together in a chroot, sic gets the welcome and the channels' topics, and darkirc stops with sic. *design*: on a device.

## Install them

Both come from the NØNOS package mirror. On the 0.9.3 release image that is `http://mirror.nonos.software/linux/<blake3>.tar`, fetched over Anyone. On a development boot it is `tools/nonos-mirror-relay --content <dir>` ([the development mirror](../build/dev-mirror.md)). Plain HTTP is enough: each listing's signature pins the content's BLAKE3, which the device checks before unpacking ([the catalogue](../userland/marketplace-catalogue.md#apps-built-from-source)). From the Terminal:

```
market install linux.nonos-darkfi-app
market install linux.nonos-darkfi-darkirc
```

or choose them in the Marketplace's Linux tab. *host-tested* (the listing and the mirror's bytes); *design* (the install)

**The boxes.** The chat app gets a window (WAYLAND), its own home (FILES_HOME) and outbound streams over the anonymity network only (NET_ANON), and listens on nothing. DarkIRC gets a terminal (TTY), its home, NET_ANON, and a listener on its own loopback (NET_LISTEN_LO), for sic. Neither sees your other files. *host-tested* (the policies)

**The app's drawing.** EGL finds the app's own Mesa driver at `/usr/lib/nonos-darkfi-app/mesa/lib/dri`, built into the driver path. The Linux box names a driver path only where the store has `/usr/lib/nonos-mesa`, so it never points the app away from its own. *host-tested*

## A live session or a kept home

- **Live, the default.** A home lasts until the machine powers off. Then the chat history and the keys are gone, for good.
- **Kept.** On a machine installed to disk, with keeping chosen at setup, a home is kept across reboots, sealed under this machine's key ([kept homes](../design/kept-homes-and-installs.md)). *design*: kept homes are not in yet.
- **The size limit.** A kept home holds 16 MiB and 128 names in 0.9.3. A long chat history can pass that.

## How it reaches the network

Both dial DarkFi's seeds, `tcp+tls://lilith0.dark.fi:25551` and `lilith1`, then the peers they name. Every peer is dialled over Anyone, or Nym when that is the chosen network, never direct, and a failure never falls back to the clear net. A seed's name is resolved at the exit, never in the clear. *design*

Port 25551 is not in every exit's policy. net.anon draws a circuit to an exit that takes the stream's port, and sends a refused open once more to another exit. The tor and i2p transports are built out. *host-tested* (net.anon's exit-port proofs)

## Where this comes from

- The recipes: `userland/capsule_market/linux-catalogue.json`. Their pins: `linux-catalogue.sources.lock.json`.
- The builds: `tools/nix/sources/darkfi-app-*.sh`, and for DarkIRC `darkfi-build.sh` then `darkirc-box.sh`, and the letters `the design notes` and `2026-10-28-s3-darkfi-gui.md`.
