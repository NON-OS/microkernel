# The NONOS overview

Start here to learn what NONOS is, how its parts fit together and what it protects, before you read a detailed section.

## NONOS in five sentences

NONOS is an operating system for x86_64 computers, built on a capability microkernel written in Rust that runs on every core the firmware enables. Device drivers, the network stack, the desktop and the apps run in ring 3, each as a signed [capsule](glossary.md#capsule) that starts with only the capabilities its signed [manifest](glossary.md#manifest) grants. On Intel VT-d machines the [IOMMU](glossary.md#iommu) confines device DMA, so each PCI device a driver claims reaches only the buffers granted to that driver; AMD-Vi is not driven and interrupt remapping is off in this release. A boot keeps nothing on a disk unless the person chooses to install. The browser and the Terminal connect through the [Nym mixnet](glossary.md#nym-mixnet) unless the person picks another network, the wallet never connects directly, and downloads for an install go over the [Anyone network](glossary.md#anyone-network) unless the person asks for a direct one.

## Read in this order

1. [Mission](mission.md): what NONOS is for, who it is for, and what it is not.
2. [Architecture](architecture.md): the whole system in one diagram, then each part with links into the detailed sections.
3. [Design principles](design-principles.md): the rules the code follows, each with the code or the check that holds it.
4. [Threat model](threat-model.md): what NONOS protects, against whom, what it assumes, and what it leaves out.
5. [Glossary](glossary.md): the terms these pages use, each with the file that defines it.
6. [FAQ](faq.md): short answers to the questions people ask first.

## Where to go next

| If you are | Read next |
|---|---|
| a person with a laptop | [Install](../install/README.md), then [Using NONOS](../using/README.md) |
| an OS developer | [Kernel](../kernel/README.md), then [Drivers](../drivers/README.md) and [Userland](../userland/README.md) |
| a security reviewer | [Threat model](threat-model.md), then [Security](../security/README.md) |
| writing a driver or an app | [Writing a driver](../drivers/writing-a-driver.md) or [Writing an app](../userland/writing-an-app.md) |
| a contributor | [Build](../build/README.md), then [Contributing](../contributing/README.md) |

## See also

- [Documentation index](../README.md)
- [Support matrix](../hardware/MATRIX.md)
- [Release notes for 0.9.2](../release/0.9.2.md)
