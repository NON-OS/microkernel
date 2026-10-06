# Using NONOS

The guide for a person who uses NONOS every day: what each part of the desktop does, what is kept when the machine powers off, and where each program stops.

## Read this first

- NONOS forgets by default. Unless it was installed to a disk, everything you make lives in memory and is gone at power off. Even an installed system keeps only some things; [Files](files.md) lists them.
- Apps are [capsules](../overview/glossary.md#capsule), each with its own short list of capabilities. A window that hangs cannot take the desktop with it: `Ctrl+Alt+Esc` always brings Processes forward, where you can end it.
- The browser, Music downloads and the Terminal's `curl` and `git` leave through the default network chosen at setup or in Settings: the Nym mixnet, the Anyone network, or Direct. A route that cannot carry a request fails it and never falls back to another (`userland/policy_proto/src/route.rs`). Two things do not simply follow it: the wallet never goes direct, and Qwen model downloads take the Anyone network unless you ask for one download to go direct (`userland/nonos_route_link/src/chosen.rs`). [Privacy networks](privacy-network.md) has the whole table.
- The desktop cannot power the machine off in this release. The power key shows `Power off is not available from the desktop`.

## The pages

| Page | Read it to |
|---|---|
| [The desktop](desktop.md) | work the menu bar, the dock, the Launchpad and windows, and see which apps ship |
| [Terminal](terminal.md) | use the shell: tabs, pipes, jobs, every built-in command, and git |
| [Files](files.md) | find your files, and learn exactly what survives a reboot |
| [Settings](settings.md) | see every Settings panel and what each row changes |
| [Keyboard layouts](keyboard-layouts.md) | choose a layout at first boot and switch it while you type |
| [Sound and media](audio.md) | play music and video, set the volume, and learn why a machine may be silent |
| [Wi-Fi and networking](wifi-and-networking.md) | join a wireless network and read the network status |
| [Privacy networks](privacy-network.md) | choose between Nym, Anyone and Direct |
| [Linux programs](linux-programs.md) | run `sh`, `python3`, `sqlite3` and the other Linux tools |
| [Local model](local-ai.md) | chat with the Qwen model on this machine, offline, and pick its tier |
| [Marketplace](marketplace.md) | install signed apps and packages |
| [Wallet](wallet.md) | hold keys and make payments |
