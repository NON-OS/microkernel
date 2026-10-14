# Protections and limits: What NONOS does not address

Part of [Protections and limits](protections-and-limits.md).

## What NONOS does not address

| Threat | Why it is not covered | Where to look |
|---|---|---|
| Device DMA on a machine with no remapping unit in service, or with AMD-Vi only | the claim goes ahead unconfined and the boot log says so; AMD-Vi is driven only by kernels built with `nonos-iommu-amdvi`, which no [build profile](../overview/glossary.md#build-profile) turns on | `unconfined_allowed` (`src/hardware/broker/confine/posture.rs:32-50`) |
| DMA from a device found at boot that no driver has claimed, or from a device with no PCI requester id | an unclaimed device stays in the [identity domain](../overview/glossary.md#identity-domain), which maps all the memory the kernel manages; a device found through ACPI gets no domain | `pci_address` (`src/hardware/broker/confine/table.rs:35-43`) |
| A device that raises interrupts it should not by writing MSI messages, on a machine without VT-d interrupt remapping (AMD-Vi included) | its claim is refused unless the kernel is built with the development opt-in, and then its messages reach the CPUs as written; with remapping on, every line names an entry for its own source and compatibility format messages are blocked | `admit` (`src/hardware/broker/confine/interrupts.rs`) |
| Reading RAM after a power cut, a forced power-off or a kernel panic | none of these runs the wipe; the panic path halts as it is | `panic` (`src/boot/panic/handler.rs:41-64`) |
| The data volume key, the token MAC key and a live boot's in-memory volume left in RAM after an orderly shutdown | the wipe covers the heap, stacks and process memory, not kernel statics such as the ones holding these keys, nor the frames that hold the in-memory volume | `VOLUME` (`src/fs/blockfs_volume/state.rs:21-26`), `Ram` (`src/fs/cryptoblock/ram.rs:41-47`) |
| A probe on the bus between the CPU and a discrete TPM | the TPM session is unbound and unsalted, with no parameter encryption, so the [machine key](../overview/glossary.md#machine-key) crosses the bus in the clear | `build_start` (`src/security/tpm/machine_key/session.rs:17-49`) |
| Malicious firmware, SMM code or a management engine | it runs below the kernel; NONOS can neither inspect nor contain it, and the PCR binding only notices a changed measurement | `BOUND_PCRS` (`src/security/tpm/machine_key/pcrs.rs:21-24`) |
| An older signed kernel on a machine with no TPM, or after the TPM is cleared | with no floor to read, every entry but Hardened and Air-Gapped boots with rollback protection off; a cleared TPM starts the floor again at 0 | `Floor::Unprotected` (`nonos-bootloader/src/boot/crypto/rollback/floor.rs:52-58`) |
| A changed bootloader on a machine with no TPM, or one that hands the kernel no TCG log, which any loader can do | the kernel can only hash the loader file the loader handed over, so the check rests on the loader's own word; the boot goes on and the verdict says self-reported | `self_reported` (`nonos-boot-measure/src/gate/verdict.rs:60-75`) |
| A whole image sealed with other keys, booted with Secure Boot off | the loader holds the kernel only to keys compiled into that loader, and only Secure Boot checks who signed the loader; such an image gets another machine key and opens no data sealed under this one | `resolve_public_key` (`nonos-bootloader/build.rs:135-158`) |
| Putting back an older copy of the data volume or of a sealed record | sectors are bound to their LBA, not to a version, and sealed records carry no counter | `AAD_PREFIX` (`src/fs/cryptoblock/seal.rs:39-42`), `userland/nonos_vault/src/lib.rs` |
| A capsule that misuses the capabilities it was granted | a capability says what kind of thing a capsule may do, not who it may do it with; only the capsule named in `PEERS` is held to a list | `PEERS` (`src/services/registry/peers.rs:17-31`) |
| A capsule holding `Crypto` deriving a machine key another capsule uses | the kernel does not tie a label to a capsule; it refuses only the kernel's own labels. With `FileSystem` as well, such a capsule can derive the vault root and open the wallet's sealed records under `/data`; the Linux personality, Settings and the Terminal hold both | `is_user_label` (`src/security/tpm/machine_key/kernel_label.rs:42-45`) |
| An attacker who takes over a capsule's code, for example through a memory bug in it | authority follows the process, not the code: tokens are bound to a pid, address space and boot, and `subject_measurement` is always zero, so the attacker gets every bit the capsule holds | `new_token` (`src/process/caps.rs:38-59`) |
| A memory bug in the kernel, used by someone who knows its layout | the kernel image sits at the same address on every boot, since nothing calls the KASLR code; the compiler adds no stack canaries; and the kernel stack each process makes its system calls on has no guard page | `randomize_layout_from_kaslr` (`src/memory/layout/manager/kaslr_ops.rs:117-125`), `allocate_kernel_stack` (`src/kernel_core/process_spawn/kernel_stack.rs:44-56`) |
| Speculative-execution and other side channels between capsules | on x86_64 each system call entry fills the return stack buffer and sets IBRS where the CPU has it, and STIBP and SSBD are set once at boot, on the boot CPU only; the exit hook that issues VERW is called only from `exec_process`, which nothing in this tree calls; the IBPB hook for context switches has no caller, the L1D flush is never issued, and KPTI is not implemented, so Meltdown-affected parts are not mitigated; cache, timing and power channels are not addressed | `kernel_entry_mitigations` (`src/security/hardening/spectre_mitigations/hooks.rs:22-37`), `exec_process` (`src/process/userspace/transitions.rs:28-33`), `enable_mitigations` (`src/security/hardening/spectre_mitigations/enable.rs:29-44`), `l1d_flush` (`src/security/hardening/spectre_mitigations/enable.rs:46-63`) |
| Files persisted to the capsule store, read straight from the disk | the store is written as given; only records a capsule seals itself are protected | `sys_store_write` (`src/syscall/microkernel/store_write.rs:24-42`) |
| Capsule diagnostics on the [serial console](../overview/glossary.md#serial-console): from service capsules in the standard, qemu and dev profiles, and from the Linux personality in every profile | those profiles keep `capsule-serial-debug`; `LINUX_CAPS` names `Debug` on every build, and the kernel prints the personality's lines unless they come from a Terminal's private run | `debugFeatures` (`tools/nix/config.nix:60-62`), `LINUX_CAPS` (`src/userspace/capsule_linux/spawn.rs:39-43`), `is_private_run` (`src/syscall/microkernel/debug.rs:61-65`) |

Other limits are stated on the pages that cover them. What a network observer
sees on each route is on [Privacy networks](../using/privacy-network.md), and
what the two anonymity transports lack is on [How the Nym and Anyone transports
are built](anonymity-transports.md). What TLS does not check, revocation and
name constraints among them, is on [TLS and certificate
trust](tls-and-certificates.md). What the bootloader checks is on [Boot chain
and signatures](boot-chain-and-signatures.md). On x86_64 every byte capsules
draw with `CryptoRandom` is stretched from RDRAND alone, and a machine where no
hardware source answers the kernel gets its token key and boot session nonce
from the cycle counter; [Randomness and
cryptography](randomness-and-cryptography.md#when-a-source-is-missing) gives
both.

## The build profile matters

The build profile an image was built with decides what it can ever do.
`tools/nix/config.nix` defines them, and `nonos.toml` picks one, `standard` by
default.

| Profile | Security posture from the profile definition |
|---|---|
| `standard` | every driver, the desktop, first-boot setup and the installer; capsules may write to the serial console |
| `hardened` | the standard system with [loader policy](../overview/glossary.md#loader-policy) `production`, built without `capsule-serial-debug`, so only the Linux personality can write to the serial console |
| `airgapped` | hardened, with no network driver, stack or online program compiled in |
| `qemu` | the desktop for virtual machines; the host sees everything the guest does |
| `dev` | the development loader policy and path-only attestation; never sealed for release |
| `core` | the microkernel and its base capsules, no desktop |

The profile's `drop` list is taken out of the kernel's features at build time,
so a dropped feature is absent from the binary rather than switched off at run
time. [Build profiles](../build/profiles.md) has the details.
