# Manifests and capabilities: What the kernel checks

Part of [Manifests and capabilities](manifests-and-capabilities.md).

## What the kernel checks

`verify_with_publisher` runs these checks in this order, and the first failure
refuses the capsule (`src/security/capsule_manifest/verify/mod.rs:37-63`,
`verify_with_publisher`):

1. The manifest decodes.
2. Its certificate id equals the BLAKE3 of the certificate presented with it.
3. Its namespace matches one of the certificate's namespace globs.
4. Its required and optional capabilities stay under the certificate's ceiling.
5. For each algorithm the production policy requires, one signature verifies
   under a publisher key the certificate carries and the [trust
   anchor](../overview/glossary-stark-proof.md#trust-anchor) policy has not
   revoked. The policy requires both Ed25519 and ML-DSA-65
   (`src/security/nonos_id_cert/policy.rs:30-32`, `NONOS_PRODUCTION_POLICY`).
6. The payload hash equals the BLAKE3 of the ELF being loaded.
7. The target triple equals the one the spawn site names. A capsule in the
   kernel image is held to the kernel's user target; a capsule loaded from the
   [file store](../overview/glossary-dma-pool.md#file-store) names its own, so
   for it this check adds nothing
   (`src/kernel_core/process_spawn/capsule_spawn/from_vfs/load/spawn.rs:65`,
   `target_triple`).
8. Every endpoint the spawn site is about to register is declared in the
   manifest.
9. The grant stays inside the manifest's required and optional sets.

Each failure has its own variant of `ManifestVerifyError`, from
`NonosIdCertIdMismatch` to `GrantOutsideManifest`
(`src/security/capsule_manifest/error.rs:45-58`). The certificate itself is
checked before any of this; [Signing and publisher
keys](signing-and-publisher-keys.md) lists those refusals.

## Capability bits

The bits are defined once in the kernel (`src/capabilities/types/defs.rs`) and
published in `abi/caps.toml`, which `scripts/gen_caps_abi.py` regenerates from
that file. There are 37, from CoreExec at bit 0 to InputConsumer at bit 36
(`src/capabilities/types/defs.rs`, `InputConsumer`). Two of them, IO at bit
1 and Hardware at bit 7, enforce nothing
(`src/capabilities/types/defs.rs:23-31`, `Hardware`). These are the ones a
capsule meets most:

| Bit | Value | Name | What it opens |
|---|---|---|---|
| 0 | `0x1` | CoreExec | process basics such as `MkGetPid` and arguments |
| 2 | `0x4` | Network | the eleven network services |
| 3 | `0x8` | IPC | sending to a service at all |
| 4 | `0x10` | Memory | `MkMmap` and `MkMunmap` |
| 5 | `0x20` | Crypto | the kernel's hash, cipher and random calls |
| 6 | `0x40` | FileSystem | being served by `vfs_pool` |
| 8 | `0x100` | Debug | lines on the kernel log |
| 15 | `0x8000` | DeviceEnum | listing devices |
| 16 to 20 | `0x1_0000` to `0x10_0000` | Driver, Mmio, Irq, Dma, Pio | the [hardware broker](../overview/glossary-hardware-broker.md#hardware-broker) |
| 32 | `0x1_0000_0000` | ForeignExec | creating Linux [guests](../overview/glossary-dma-pool.md#guest); only the [Linux personality](../overview/glossary-linux-personality.md#linux-personality) holds it |
| 35 | `0x8_0000_0000` | DeviceSecret | the [device secret](../overview/glossary-claim-epoch.md#device-secret); the signing step refuses it to every capsule but `prove` |

The full table is in [ABI: capabilities](../abi/capabilities.md).
