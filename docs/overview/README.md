# The NONOS overview

Start here to learn what NONOS is, how its parts fit together and what it protects, before you read a detailed section.

## NONOS in four sentences

NONOS is an operating system for x86_64 computers, built on a capability microkernel written in Rust. Device drivers, the network stack, the desktop and the apps run in ring 3, each as a signed [capsule](glossary.md#capsule) that holds only the capabilities its signed manifest grants. A boot keeps nothing on a disk unless the person chooses to install. The system's own connections leave through the Nym mixnet unless the person picks another network.
