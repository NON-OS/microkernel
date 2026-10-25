# Privacy apps

The Linux apps in the App Store whose job is privacy: what each one protects, what it leaves exposed, and where its box stops. Every app here is an Alpine v3.20 package that the store installs in the box its release is sold with ([The marketplace catalogue](../userland/marketplace-catalogue.md)).

Each claim is marked with how far it has been checked:

- **booted**: seen on a NONOS boot, with the log kept;
- **host-tested**: checked by a test or a tool run on a build machine, not on a boot;
- **design**: what the code and the app's own source say should happen, not yet seen either way.

Nothing on this page is booted yet. The boot plan for each app is in `HANDOFF/catalogue/05-boot-plan.md`.

## What every networked app here shares

- **No direct connection.** A Linux app reaches the network only with NET_ANON in its box. Its streams leave over the Anyone network when Anyone is the default, and over the Nym mixnet otherwise, including when the default is Direct. A guest is never given a direct socket, and a failure on one network never falls back to another (`userland/capsule_linux/src/linux/net/guest_route.rs`). *design*
- **Names are not looked up here.** An app's lookup of a name is answered inside the personality with a stand-in address, and the name itself travels to the exit, which resolves it there (`userland/capsule_linux/src/linux/net/dns/`, the plan). So the network this machine sits on sees no lookup. Only A queries are answered: an app that asks for SRV or TXT records gets no answer and must be given a host and port. *design*
- **Any destination, not a list.** These releases carry a zero `net_dests` hash, so the box allows any destination, but only over Nym or Anyone. A market release cannot yet carry a destination list; until it can, a list would only refuse every connection. *host-tested* (the policy bytes)
- **What the exit sees.** The exit relay or mixnet exit sees the server's name and port, and, unless the app's own protocol encrypts, what is sent. Every app below uses TLS to its server.
- **One program per box.** A release's box names one program. Any other program in the same package is refused at its installed path ([The marketplace catalogue](../userland/marketplace-catalogue.md)). *host-tested*
- **Home is the app's own.** FILES_HOME gives the app its own home directory and nothing of the person's other files. On a live boot that home is in memory and gone at power off; on an installed system it is kept on the encrypted data volume ([Files](files.md)). *design*

## Profanity: XMPP chat with OMEMO

Listing `linux.profanity`, package `profanity 0.14.0-r1`, licence GPL-3.0-or-later. Box: TTY FILES_HOME NET_ANON, any destination over Nym or Anyone, system calls not yet measured. The program is `/usr/bin/profanity`, the package's only one, BLAKE3 `c56437c8…0859b22`. *host-tested* (the policy, from the package the catalogue authenticated)

### What it protects

- **Your address from your server and your contacts.** The XMPP server, and every server it federates with, see the exit's address, never this machine's. *design*
- **Message content, with OMEMO on.** With OMEMO (XEP-0384) on, messages are end-to-end encrypted to your contact's devices, and the servers carry ciphertext. *design* (Profanity's own feature; not yet seen on a boot)
- **The connection to your server.** It is TLS, checked against the CA bundle Alpine ships (`ca-certificates`, a dependency). *design*

### What it does not protect

- **Who you talk to, and when.** Your server knows your account, your contact list, your presence, and the time and size of every message. Federated servers see the same for the contacts they host. OMEMO hides none of this.
- **OMEMO is not on for a new conversation.** Profanity's default OMEMO policy is `automatic`: it encrypts a chat only once OMEMO has been started in it. Type `/omemo policy always` once to make it the default, or `/omemo start` in each chat. Its trust mode is `manual`: a contact's device is used only after you have checked its fingerprint (`/omemo fingerprint`) and run `/omemo trust`. *design* (Profanity 0.14.0's `src/config/preferences.c`)
- **Your history on this machine.** By default Profanity keeps every message in its database and chat logs in its home, including OMEMO messages after decryption. `/privacy logging off` stops that, at the cost Profanity's help names: later OMEMO messages fetched from the server's archive may not decrypt. On an installed system those files are kept on the encrypted volume; on a live boot they are gone at power off. *design*
- **What it says about itself.** By default it tells anyone who asks its name and version, and the operating system it runs on (XEP-0092). It also sends typing notifications, read receipts and message carbons. `/privacy os off`, `/states off` and `/receipts send off` turn those off. *design*
- **Server discovery.** Profanity looks up the server's SRV record, which goes unanswered here (see above). It then connects to the domain itself on port 5222. If your provider hosts XMPP on another host or port, give it with `/account set <account> server <host>` and `/account set <account> port <port>`. *design*
- **Other features.** OTR and PGP work as Profanity documents them. Anything that runs another program is refused, because the box has no EXEC: opening a link or an image in an external viewer, and a `vcard.photo.cmd` or `eval_password` command. Write your password at the prompt instead. *design*
- **Nym or Anyone exits may refuse the port.** Whether a given exit carries port 5222 is up to that exit's policy. A refused connection fails and is not retried on another network. *design*

### Using it

1. Install `linux.profanity` from the App Store's Linux tab, then run `profanity` in the Terminal.
2. `/connect you@example.org`. Give the password at the prompt.
3. `/omemo gen` once per account, then `/omemo policy always`.
4. Before you trust a contact: compare fingerprints over another channel, then `/omemo trust <contact> <fingerprint>`.
5. Optional: `/privacy os off`, `/states off`, `/receipts send off`, `/privacy logging off`.

## Not here yet, and why

| App | What blocks it |
|---|---|
| Monero wallet (`monero-wallet-cli`) | The `monero` package has 14 programs and no `usr/bin/monero`, so the installer would box the first one in the archive, not the wallet. A release has no way yet to name which program it boxes. |
| GnuPG, `pass` | `gpg` runs `gpg-agent` for every secret-key operation, and a release can box only one program (M-09). |
| KeePassXC, Feather | Need Qt and a display path the Linux personality does not have yet (G9). |
| OnionShare | Its job is serving files to others: an inbound service, which a guest is never given. |
| exiftool | A Perl script: the program that runs is the image's `perl`, and a box follows a path, not the script (M-10). |
| age, minisign | Already listed, in the catalogue's first set. |
