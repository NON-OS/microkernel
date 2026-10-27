# SimpleX Chat

First draft (S3, 2026-10-27). SimpleX Chat is a messenger with no user identifiers: no phone number, no account name and no ID that a server can tie your contacts together by. On NØNOS it comes in two parts:

- **SimpleX Chat core** is the Marketplace listing `linux.kali.nonos-simplex-chat`. It is SimpleX's own `simplex-chat` program, the release binary SimpleX publishes, not one built here. It runs in a Linux box with no window, and the app drives it over its local API.

Each claim is marked with how far it has been checked, as in [Privacy apps](privacy-apps.md): **booted**, **host-tested** or **design**. Nothing on this page is booted yet.

## What you install

- **The binary.** `simplex-chat-ubuntu-22_04-x86_64` from SimpleX's v7.0.3 release. It is pinned by sha256 `5afb1d25…2e77` and by BLAKE3. That sha256 is the one SimpleX's `_sha256sums` gives it, and that file is signed by SimpleX's release key `BBDF 7BDA D154 8B16 836A F5B9 D53B DFD1 53C3 66BA` ([the catalogue](../userland/marketplace-catalogue.md#upstreams-own-release-binary)). *host-tested*
- **Its libraries.** It is built for glibc, so it runs in the Debian family's tree. It brings nine pinned Kali packages, each held to its SHA-256: libc6, libgcc-s1, gcc-16-base, libc-gconv-modules-extra, libgmp10, libssl3t64, libzstd1, zlib1g and openssl-provider-legacy. *host-tested* (the closure and the libraries the binary names); *design* (the install on a device)
- **Its box.** Its own home (FILES_HOME), outbound streams over the anonymity network only (NET_ANON), and a listener on its own loopback for the app (NET_LISTEN_LO). It gets no window, no terminal and none of your other files. *design*

## Create a profile

The first time it starts, the core has no profile. The app starts it with `--user-display-name <name>`, so it creates one without asking. Without a terminal it cannot ask. *design*

- **The display name.** Your contacts see it when you connect. It stays on this machine, and you can change it later (`/_profile`).
- **Profiles.** A profile has no identifier a server sees: each contact is reached through queues of its own.
- **Bots.** If you run the core yourself, `--create-bot-display-name` makes a bot profile instead. That is SimpleX's [bot API](https://github.com/simplex-chat/simplex-chat/blob/stable/bots/README.md).

## Connect to a contact

There are two ways. Either way the link goes to the other person by some other channel: show it as a QR code, or send it on a channel you already trust.

- **You invite.** The app asks the core for a one-time invitation, `/_connect <userId>`, and shows you the link.
- **They invite.** You paste their link, and the app sends `/connect <link>`. The core answers `sentConfirmation` for a one-time link, or `sentInvitation` for an address.

When both sides have joined, the core reports `contactConnected`. After that, messages go both ways (`/_send`, and `newChatItems` as they arrive). A long-term address (`/_address <userId>`) lets people ask to connect with you more than once. You accept each one (`receivedContactRequest`, then `/_accept`). *design*

## Where your messages are

- **The database.** The core keeps everything in two SQLite files in its home: `/root/.simplex/simplex_v1_chat.db`, the profile, contacts and messages, and `/root/.simplex/simplex_v1_agent.db`, the connection keys. These are simplex-chat's defaults (`-d` changes the prefix). *design*
- **Files.** Files you receive go where the app tells the core (`--files-folder`). By default that is inside the same home. *design*
- **Who can read it.** No other Linux program sees this home. The files are not encrypted by SimpleX unless a database passphrase is set (SimpleX's `--key`, or the app's own setting). The home itself is on NØNOS's sealed volume only when it is kept (below).

## A live session or a kept home

- **Live, the default.** A home lasts until the machine powers off. Turn it off and the profile, contacts and messages are gone, and so are the keys: nobody can read them later, including you. *booted* for homes in general (age and git, [Privacy apps](privacy-apps.md)); *design* for SimpleX.
- **Kept.** On a machine installed to disk, with keeping chosen at setup, the home is kept across reboots, sealed under this machine's key ([kept homes](../design/kept-homes-and-installs.md)). A write is kept once the core's fsync returns, which SQLite does on each commit. *design*: kept homes are not in yet.
- **The size limit.** A kept home holds 16 MiB and 128 names in 0.9.3. A busy profile, or received files, can pass that. Keep files out of the home, or expect the app to say when the home is full. *design*

The app should show which one you have, live or kept, before you create a profile.

## How it reaches SimpleX servers

Messages travel through SMP relays. The core never connects to one directly.

- **Anyone only.** NET_ANON is the only network the box has. Its streams leave over Anyone, or over Nym when that is the network chosen, never directly, and a failure on one never falls back to another. A name it looks up travels to the exit, and only the exit resolves it ([Privacy apps](privacy-apps.md#what-every-networked-app-here-shares)). *design*
- **SimpleX's own protections still apply.** Private message routing (`--smp-proxy`) means your relay does not learn whose relay you send to. Each queue is end-to-end encrypted.

**Pending (S1).** Two pieces of the network path are not in yet:

- the guest-connect fix, so a listed box's outbound stream completes over Anyone;
- a SOCKS5 endpoint at `127.0.0.1:9050` inside the box.

Once the endpoint exists, the intended default is that the app starts the core with `--socks-proxy=127.0.0.1:9050`. simplex-chat then hands each relay's name to SOCKS rather than resolving it itself, and can keep connections apart per contact (SOCKS isolation). This includes `.onion` relays, which only work through SOCKS. Until both land, the core starts but does not connect.

## What the core does not do

- **No window.** The app is the window, and the core has no display.
- **No clear net.** See above.
