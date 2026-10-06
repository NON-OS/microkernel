# FAQ

Short answers to the questions people ask first about NONOS, each with a link to the full story.

## NONOS is not a Linux distribution

NONOS has its own kernel, a capability microkernel written in Rust, and no Linux kernel at all. Linux programs run on it through the [Linux personality](glossary.md#linux-personality), a [capsule](glossary.md#capsule) that answers their Linux syscalls. See [Architecture](architecture.md).

## Running Linux programs

The Linux personality runs x86_64 Linux programs in processes that hold no NONOS capabilities. A program read from the store runs only if the proof kept beside it verifies; the built-in BusyBox, which provides `sh`, is part of the personality's own signed image. With `linux = true` under `[store]` in `nonos.toml`, the default, the image's store carries Linux tools, python3, sqlite3 and john among them (`tools/nix/store.json`). A Linux call the personality does not serve gets ENOSYS, and the log names the call. A program started from the Terminal's `linux` command reaches no network. See [Linux programs](../using/linux-programs.md) and [Linux personality](../userland/linux-personality.md).
