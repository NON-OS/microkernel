# Userland

Every program NONOS runs outside the kernel runs in ring 3, as a [capsule](../overview/glossary.md#capsule) or as a Linux [guest](../overview/glossary.md#guest) of one; this section covers what a capsule is, how it goes from source to a running process, and what lives under `userland/`.

## What a capsule is

A capsule is an ELF and three files that vouch for it, and the kernel reads the four together. The build writes the three into `nonos-data/trust/capsules/` as `<bin>.nonos_id_cert.bin`, `<bin>.manifest.bin` and `<bin>.zk_trailer.bin` (`nonos-mk/capsule.mk:101-103`, `CAPSULE_BIN_NAME`).

- The ELF is a static, position independent executable for the `x86_64-nonos-user` target, whose `vendor` is `nonos` and whose linker runs with `-nostdlib -pie` (`userland/x86_64-nonos-user.json:4-27`).
- The [NONOS ID certificate](../overview/glossary.md#nonos-id-certificate) names the [publisher](../overview/glossary.md#publisher)'s two public keys, the namespaces the publisher may sign for and a ceiling on capabilities.
- The [manifest](../overview/glossary.md#manifest) binds the BLAKE3 hash of the ELF, its namespace, its IPC endpoints and the capabilities it asks for, signed by the publisher.
- The [attestation trailer](../overview/glossary.md#attestation-trailer) comes from one STARK enrollment of the whole capsule set.

The kernel does not take a capsule's word for anything. It computes a [capability word](../overview/glossary.md#capability-word) from the verified manifest. Every system call passes a gate, most of them a bit in that word, and every IPC send is checked against it. [Manifests and capabilities](manifests-and-capabilities.md) has the rules.
