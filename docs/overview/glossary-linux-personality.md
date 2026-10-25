# Glossary: Linux personality

Part of [Glossary](glossary.md).

## Linux personality

The capsule `capsule_linux`, `app.linux`, which runs unmodified x86_64 Linux
programs as guests and answers the Linux system calls the kernel hands it. It is
the only capsule that holds ForeignExec, and it runs a program from the store
only when the proof kept beside it verifies. Explained in [The Linux
personality](../userland/linux-personality.md). Code:
`userland/capsule_linux/Capsule.mk`.

## Loader policy

The bootloader policy an image is built with, weakest first: `dev-qemu`,
`standard-qemu`, `standard` or `production`. Each build profile sets the weakest
it allows, `production` for hardened and airgapped, and `dev-qemu`, which
compiles in the development override, is never sealed for release. Explained in
[Profiles](../build/profiles.md#the-floors). Code: `tools/nix/config.nix`.

## Machine key

A 32-byte key the TPM derives on request as an HMAC over a label, under a
primary key whose policy binds PCRs 0, 4, 7 and 9. Nothing is stored: one
machine in one boot state gets the same key every time, and a firmware, Secure
Boot, loader or kernel change gives another. The data volume key and the key
that seals saved Wi-Fi networks are machine keys; a capsule holding Crypto asks
for one with `CryptoMachineKey`, under any label except the kernel's own, which
start with a zero byte. Explained in [Device secrets and
keys](../security/device-secrets-and-keys.md#the-machine-key). Code:
`src/security/tpm/machine_key/mod.rs`, `src/security/tpm/machine_key/pcrs.rs`,
`src/security/tpm/machine_key/kernel_label.rs`.

## Manifest

<a id="capsule-manifest"></a>The publisher-signed binary record, schema version 3, that describes one capsule: its certificate id, namespace, version, target triple, the BLAKE3 hash of its ELF, its required and optional capabilities, up to 16 endpoints and up to 4 publisher signatures. `verify_with_publisher` checks it at every spawn, and a grant outside it is refused with `GrantOutsideManifest`. Explained in [Manifests and capabilities](../userland/manifests-and-capabilities.md). Code: `src/security/capsule_manifest/schema/manifest.rs`, `src/security/capsule_manifest/verify/mod.rs`.

## NONOS ID certificate

A publisher's certificate, signed by the trust anchor with both Ed25519 and
ML-DSA-65. It binds the publisher's NONOS ID to its public keys, the namespaces
it may publish under, its capability ceiling and the trust-anchor epoch it was
issued under. Explained in [Signing and publisher
keys](../userland/signing-and-publisher-keys.md). Code:
`src/security/nonos_id_cert/schema/cert.rs`,
`src/security/nonos_id_cert/policy.rs`.

## Nym mixnet

The Nym anonymity network, route value `NYM`, and the default network for the
browser, the Terminal and the wallet. A program reaches it through `net.socks5`,
and when the policy store does not answer, the route is read as Nym, never as
Direct. Explained in [Privacy networks](../using/privacy-network.md). Code:
`userland/policy_proto/src/route.rs`, `userland/nonos_route_link/src/chosen.rs`.

## Package store

<a id="store"></a>The region of a NONOS disk from sector 256 up to the disk plan at sector 245,760, headed by the magic `NONOSTR1`. The seal fills it with what `tools/nix/store.json` declares, the Linux userland, demo capsules, sample films and wallpapers among them; on a boot that keeps data, the file store adds each file a program asks it to keep, setup's answers and kept settings among them. It is written unencrypted, and a Linux package install is not added to it but held in memory until restart. Explained in [Storage drivers](../drivers/storage/README.md#the-disk-layout). Code: `userland/nonos_disk_map/src/places.rs`, `userland/capsule_vfs/src/server/handlers/store_persist.rs`, `userland/capsule_linux/src/linux/install/place.rs`.

## PCR

A TPM platform configuration register. The firmware extends PCR 4 with each UEFI
application it starts, the loader extends PCR 9 once for the admitted kernel,
NONOS binds its machine keys to PCRs 0, 4, 7 and 9, and a quote covers PCRs 0,
1, 2 and 7. Explained in [Measured boot and the
TPM](../security/measured-boot-and-tpm.md#pcrs). Code:
`src/security/tpm/machine_key/pcrs.rs`, `src/security/tpm/boot_reads/pcr4.rs`.
