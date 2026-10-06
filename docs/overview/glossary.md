# Glossary

Every term the NONOS pages use, defined in a few sentences, with the page that explains it in full and the code it comes from.

Terms are in alphabetical order. Where two names mean one thing, one entry carries both, and a link to either name lands on it.

## Amnesic boot

<a id="amnesic"></a>A boot that keeps nothing on a disk. Every boot is amnesic until first-boot setup records the choice to install in the policy store's `Persistent` field, labelled `Keep data across reboots`; until then the file store refuses every request to keep a file, and an image built with `install = false` has no setup, so it never keeps anything. Explained in [Design principles](design-principles.md#amnesic-by-default). Code: `userland/capsule_vfs/src/server/handlers/persist_gate.rs`.

## Anyone network

The Anyone onion network, route value `ANYONE`, which puts three relays between this machine and the site and is reached through `net.anon`. App installs, Linux package installs and Qwen model downloads take it whatever the default network is; `qwen get --direct` sends one model download directly instead. Explained in [Privacy networks](../using/privacy-network.md). Code: `userland/nonos_route_link/src/chosen.rs`, `userland/capsule_model_fetch/src/get/offer.rs`.

## Application processor

Any CPU other than the boot CPU. On x86_64 the boot CPU starts each one in the last stage of kernel init, with INIT and two STARTUP interrupts through a real-mode trampoline at physical 0x8000. Explained in [Scheduler and SMP](../kernel/scheduler-and-smp.md). Code: `src/smp/init/ap_unit.rs`.
