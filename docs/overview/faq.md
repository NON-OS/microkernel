# FAQ

Short answers to the questions people ask first about NONOS, each with a link to the full story.

## NONOS is not a Linux distribution

NONOS has its own kernel, a capability microkernel written in Rust, and no Linux kernel at all. Linux programs run on it through the [Linux personality](glossary.md#linux-personality), a [capsule](glossary.md#capsule) that answers their Linux syscalls. See [Architecture](architecture.md).
