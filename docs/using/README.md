# Using NONOS

The guide for a person who uses NONOS every day: what each part of the desktop does, what is kept when the machine powers off, and where each program stops.

## Read this first

- NONOS forgets by default. Unless it was installed to a disk, everything you make lives in memory and is gone at power off. Even an installed system keeps only some things; [Files](files.md) lists them.
- Apps are [capsules](../overview/glossary.md#capsule), each with its own short list of capabilities. A window that hangs cannot take the desktop with it: `Ctrl+Alt+Esc` always brings Processes forward, where you can end it.
- The browser, Music downloads and the Terminal's `curl` and `git` leave through the default network chosen at setup or in Settings: the Nym mixnet, the Anyone network, or Direct. A route that cannot carry a request fails it and never falls back to another (`userland/policy_proto/src/route.rs`). Two things do not simply follow it: the wallet never goes direct, and Qwen model downloads take the Anyone network unless you ask for one download to go direct (`userland/nonos_route_link/src/chosen.rs`). [Privacy networks](privacy-network.md) has the whole table.
- The desktop cannot power the machine off in this release. The power key shows `Power off is not available from the desktop`.
