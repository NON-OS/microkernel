# DarkFi

S3, 2026-10-29. DarkFi is an anonymous peer-to-peer network with its own chain, wallet and chat. NØNOS 0.9.3 lists one DarkFi program in the Marketplace, DarkFi's chat app, built here from DarkFi v0.5.0 (commit `2e2de47c…`) without tor or i2p. It runs in its own Linux box. Each claim is marked *booted*, *host-tested* (the listing and its content) or *design*.

## What is listed

| Listing | What it is |
|---|---|
| `linux.nonos-darkfi-app` | **DarkFi (chat app)**, DarkFi's desktop app (`bin/app`), with DarkIRC built in. It opens in its own window. It is software-rendered: Mesa's softpipe, bundled in its content, draws on the CPU with no GPU, so it is slower than on other systems. Copy and paste works inside the app, not with other apps. |

**Not in 0.9.3: drk, darkirc and darkfid.** DarkFi's command-line wallet, IRC daemon and node are built (the same commit) but not listed:

- each stops on its first run, after writing its default config;
- darkirc needs loopback listeners for its IRC and RPC ports, and has no IRC client in its box;
- drk needs a darkfid node, and a full chain does not fit an app's home.

The chat app already carries DarkIRC, so chat needs none of them. They are to be listed once those are fixed.

## Install it

It comes from the NØNOS package mirror. On the 0.9.3 release image that is `http://mirror.nonos.software/linux/<blake3>.tar`, fetched over Anyone. On a development boot it is `tools/nonos-mirror-relay --content <dir>` ([the development mirror](../build/dev-mirror.md)). Plain HTTP is enough: the listing's signature pins the content's BLAKE3, which the device checks before unpacking ([the catalogue](../userland/marketplace-catalogue.md#apps-built-from-source)). From the Terminal:

```
market install linux.nonos-darkfi-app
```

or choose "DarkFi (chat app)" in the Marketplace's Linux tab. *host-tested* (the listing and the mirror's bytes); *design* (the install)

**Its box.** A window (WAYLAND), its own home (FILES_HOME) and outbound streams over the anonymity network only (NET_ANON). It listens on nothing and sees none of your other files. *host-tested* (the policy)

**Its drawing.** EGL finds the app's own Mesa driver at `/usr/lib/nonos-darkfi-app/mesa/lib/dri`, built into the driver path. The Linux box names a driver path only where the store has `/usr/lib/nonos-mesa`, so it never points the app away from its own. *host-tested*

## A live session or a kept home

- **Live, the default.** A home lasts until the machine powers off. Then the chat history and the app's keys are gone, for good.
- **Kept.** On a machine installed to disk, with keeping chosen at setup, a home is kept across reboots, sealed under this machine's key ([kept homes](../design/kept-homes-and-installs.md)). *design*: kept homes are not in yet.
- **The size limit.** A kept home holds 16 MiB and 128 names in 0.9.3. A long chat history can pass that.

## How it reaches the network

The app dials DarkFi's seeds, `tcp+tls://lilith0.dark.fi:25551` and `lilith1`, then the peers they name. Every peer is dialled over Anyone, or Nym when that is the chosen network, never direct, and a failure never falls back to the clear net. A seed's name is resolved at the exit, never in the clear. *design*

Port 25551 is not in every exit's policy. net.anon draws a circuit to an exit that takes the stream's port, and sends a refused open once more to another exit. The tor and i2p transports are built out. *host-tested* (net.anon's exit-port proofs)

## Where this comes from

- The recipe: `userland/capsule_market/linux-catalogue.json`. Its pin: `linux-catalogue.sources.lock.json`.
- The build: `tools/nix/sources/darkfi-app-*.sh`, and the letters `HANDOFF/letters/2026-10-27-s3-darkfi-build.md` and `2026-10-28-s3-darkfi-gui.md`.
