# Privacy apps

The Linux apps in the App Store whose job is privacy: what each one protects, what it leaves exposed, and where its box stops. Every app here is an Alpine v3.20 package that the store installs in the box its release is sold with ([The marketplace catalogue](../userland/marketplace-catalogue.md)).

Each claim is marked with how far it has been checked:

- **booted**: seen on a NONOS boot, with the log kept;
- **host-tested**: checked by a test or a tool run on a build machine, not on a boot;
- **design**: what the code and the app's own source say should happen, not yet seen either way.


## What every networked app here shares

- **No direct connection.** A Linux app reaches the network only with NET_ANON in its box. Its streams leave over the Anyone network when Anyone is the default, and over the Nym mixnet otherwise, including when the default is Direct. A guest is never given a direct socket, and a failure on one network never falls back to another (`userland/capsule_linux/src/linux/net/guest_route.rs`). *design*
- **Names are not looked up here.** An app's lookup of a name is answered inside the personality with a stand-in address, and the name itself travels to the exit, which resolves it there (`userland/capsule_linux/src/linux/net/dns/`, the plan). So the network this machine sits on sees no lookup. Only A queries are answered: an app that asks for SRV or TXT records gets no answer and must be given a host and port. *design*
- **Any destination, not a list.** These releases carry a zero `net_dests` hash, so the box allows any destination, but only over Nym or Anyone. A market release cannot yet carry a destination list; until it can, a list would only refuse every connection. *host-tested* (the policy bytes)
- **What the exit sees.** The exit relay or mixnet exit sees the server's name and port, and, unless the app's own protocol encrypts, what is sent. Every app below uses TLS to its server.
- **One program per box.** A release's box names one program. Any other program in the same package is refused at its installed path ([The marketplace catalogue](../userland/marketplace-catalogue.md)). *host-tested*
- **Home is the app's own, and today it does not last.** FILES_HOME gives the app its own home directory and nothing of the person's other files. Today that home is made for each run and removed when the program exits ([Linux programs](linux-programs.md)); a home kept between runs is not in yet. *design*

## Profanity: XMPP chat with OMEMO

Listing `linux.profanity`, package `profanity 0.14.0-r1`, licence GPL-3.0-or-later. Box: TTY FILES_HOME NET_ANON, any destination over Nym or Anyone, system calls not yet measured. The program is `/usr/bin/profanity`, the package's only one, BLAKE3 `c56437c8…0859b22`. *host-tested* (the policy, from the package the catalogue authenticated)

### What it protects

- **Your address from your server and your contacts.** The XMPP server, and every server it federates with, see the exit's address, never this machine's. *design*
- **Message content, with OMEMO on.** With OMEMO (XEP-0384) on, messages are end-to-end encrypted to your contact's devices, and the servers carry ciphertext. *design* (Profanity's own feature; not yet seen on a boot)
- **The connection to your server.** It is TLS, checked against the CA bundle Alpine ships (`ca-certificates`, a dependency). *design*

### What it does not protect

- **Who you talk to, and when.** Your server knows your account, your contact list, your presence, and the time and size of every message. Federated servers see the same for the contacts they host. OMEMO hides none of this.
- **OMEMO is not on for a new conversation.** Profanity's default OMEMO policy is `automatic`: it encrypts a chat only once OMEMO has been started in it. Type `/omemo policy always` once to make it the default, or `/omemo start` in each chat. Its trust mode is `manual`: a contact's device is used only after you have checked its fingerprint (`/omemo fingerprint`) and run `/omemo trust`. *design* (Profanity 0.14.0's `src/config/preferences.c`)
- **Nothing it keeps lasts past one run, today.** Because the home is removed when Profanity exits, its account settings, its OMEMO identity keys and its history are gone at every exit. Each start is a new OMEMO device that your contacts must trust again, and nothing written during a run survives it. Until homes are kept, use it for a session at a time. *design*
- **Your history on this machine.** By default Profanity keeps every message in its database and chat logs in its home while it runs, including OMEMO messages after decryption. `/privacy logging off` stops that, at the cost Profanity's help names: later OMEMO messages fetched from the server's archive may not decrypt. *design*
- **What it says about itself.** By default it tells anyone who asks its name and version, and the operating system it runs on (XEP-0092). It also sends typing notifications, read receipts and message carbons. `/privacy os off`, `/states off` and `/receipts send off` turn those off. *design*
- **Server discovery.** Profanity looks up the server's SRV record, which goes unanswered here (see above). It then connects to the domain itself on port 5222. If your provider hosts XMPP on another host or port, give it with `/account set <account> server <host>` and `/account set <account> port <port>`. *design*
- **Other features.** OTR and PGP work as Profanity documents them. Anything that runs another program is refused, because the box has no EXEC: opening a link or an image in an external viewer, and a `vcard.photo.cmd` or `eval_password` command. Write your password at the prompt instead. *design*
- **Nym or Anyone exits may refuse the port.** Whether a given exit carries port 5222 is up to that exit's policy. A refused connection fails and is not retried on another network. *design*

### Using it

2. `/connect you@example.org`. Give the password at the prompt.
3. `/omemo gen`, then `/omemo policy always`. Until homes are kept between runs, do this at every start.
4. Before you trust a contact: compare fingerprints over another channel, then `/omemo trust <contact> <fingerprint>`.
5. Optional: `/privacy os off`, `/states off`, `/receipts send off`, `/privacy logging off`.

## Monero wallet: monero-wallet-cli

Listing `linux.monero`, package `monero 0.18.3.3-r1`, licence BSD-3-Clause. Box: TTY FILES_HOME NET_ANON, any destination over Nym or Anyone, system calls not yet measured. The program is `/usr/bin/monero-wallet-cli`, BLAKE3 `2073f1b6…cc669c83`, named by the recipe's `program`: the package has 14 programs and no `usr/bin/monero`, and the installer records the one whose bytes the policy names (`install/market/choose.rs`). *host-tested* (the policy, from the package the catalogue authenticated)

Not booted yet: install and run wait on a boot, and the package (24.7 MB) is large enough that the install's memory limit on big packages may stop it until that is fixed.

### What it protects

- **Your address from the node.** The wallet talks to a remote node you name with `--daemon-address host:port`, over Nym or Anyone, never direct, so the node sees an exit, not this machine. *design*
- **Your keys from the network.** The wallet's keys and its files stay in its own home; nothing else on the machine reads them. *design*

### What it does not protect

- **What the node learns from your wallet.** A remote node sees which blocks and outputs your wallet asks for and when, and can try to link them to your wallet. Running your own node is the stronger choice, and it is not possible here: the box runs only the wallet, and `monerod` is refused.
- **Nothing it keeps lasts past one run, today.** The home is removed when the wallet exits, so a wallet file made in a run is gone with it, and with it the keys. Write down the seed phrase the wallet shows, on paper, and restore from it with `--restore-deterministic-wallet`; until homes are kept, every start is a restore. Never type a seed into anything you have not checked. *design*
- **The other programs in its package.** `monerod`, `monero-wallet-rpc` and the blockchain tools have no box of their own and are refused at their installed paths.
- **Name lookups for OpenAlias.** A Monero address written as a name is looked up with DNS TXT records, which are not answered here (see above). Use the full address.
- **The network from the Terminal.** As for Profanity: started from the Terminal it runs without the network today, so it cannot reach a node from there. *booted* for Profanity, the same rule (`run_net.rs`)

## Not here yet, and why

| App | What blocks it |
|---|---|
| GnuPG, `pass` | `gpg` runs `gpg-agent` for every secret-key operation, and a release can box only one program (M-09). |
| KeePassXC, Feather | Need Qt and a display path the Linux personality does not have yet (G9). |
| OnionShare | Its job is serving files to others: an inbound service, which a guest is never given. |
| exiftool | A Perl script: the program that runs is the image's `perl`, and a box follows a path, not the script (M-10). |
| age, minisign | Already listed, in the catalogue's first set. |
