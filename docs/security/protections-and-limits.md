# Protections and limits

What NONOS 0.9.2 protects against, with the code that does it, and what it does not protect against, with the reason.

## How to read this page

Each row names a threat, what the code does about it, and where. A row in the second table is a limit of this release, stated so you can decide what to trust NONOS with. Neither table is a promise beyond the code it cites.

## What NONOS addresses

| Threat | What the code does | Where |
|---|---|---|
| A [capsule](../overview/glossary.md#capsule) makes a system call its [manifest](../overview/glossary.md#manifest) did not grant | the call is refused with `EPERM` before its handler runs | `dispatch` (`src/syscall/contract/dispatch.rs:31-40`) |
| A capsule forges a token, or reuses one from another process or an earlier boot | no call takes a token from a capsule: `Capability::resolve` reads the caller's token from its process entry in the kernel, then checks its MAC under a key drawn at each boot, its boot nonce, address space and revocation epoch | `resolve` (`src/syscall/contract/capability.rs:34-46`, `src/syscall/contract/resolver/resolve.rs:31-43`) |
| A capsule asks for more authority than its [publisher](../overview/glossary.md#publisher) may grant | the manifest's bits must fit under the [ceiling](../overview/glossary.md#capability-ceiling) in the publisher's certificate | `check_ceiling` (`src/security/capsule_manifest/verify/caps.rs:21-29`) |
| A capsule reads or writes another capsule's memory | every process has its own page tables, with only the kernel half shared | `create_address_space` (`src/memory/paging/manager/address_space/create.rs:37-48`) |
| A capsule hands the kernel a pointer into kernel memory | a range that ends above the user half is refused, and every page must carry the user bit | `check_range` (`src/usercopy/policy.rs:35-50`) |
| The kernel is made to run code from a capsule's pages, or to touch them outside its copy routines | SMEP and SMAP where CPUID reports them, with NX and CR0.WP, are set on every CPU and read back; a CPU without NX does not boot, a secondary CPU weaker than the boot CPU runs no capsule code, and no mapping may be both writable and executable | `apply` (`src/memory/mmu/mmu/protect/apply.rs:26-44`), `finish` (`src/smp/ap/user_setup.rs:46-59`), `is_wx_violation` (`src/memory/paging/manager/mapping/map.rs:41-43`) |
| A capsule sends to a service it has no rights to | the sender must hold every bit the [endpoint](../overview/glossary.md#endpoint) requires, and an unregistered name is refused | `caller_satisfies_endpoint` (`src/syscall/microkernel/ipc/send_caps.rs:36-63`) |
| A capsule talks to a device driver directly, for example to put raw frames on the wire | the driver endpoints in `HELD` accept only the services named for them, and the three PCI storage drivers check each sender for `StoreWrite` themselves | `HELD` (`src/services/registry/held_table.rs:20-36`) |
| A [driver capsule](../overview/glossary.md#driver-capsule) reads another program's keystrokes | draining the input ring needs `InputSource`, and holding `Irq` is not enough | `can_input_consumer` (`src/capabilities/token/types/authority_broker.rs:64-73`) |
| A device writes outside its driver's buffers by DMA, on a machine with VT-d in service | each driver capsule gets its own [IOMMU domain](../overview/glossary.md#iommu-domain), and a claim the unit cannot confine is refused | `attach` (`src/hardware/broker/confine/attach.rs:30-101`) |
| A driver reprograms a BAR or other PCI config space | only a short allowlist of bits may change | `validate` (`src/hardware/broker/pci/allowlist.rs:37-60`) |
| A driver maps an MSI-X table to redirect interrupts | MSI-X tables and pending-bit arrays are kept out of every MMIO mapping | `protected_regions` (`src/hardware/broker/mmio/msix_exclusion.rs:39-52`) |
| One DMA buffer's contents reach the next holder of its frames | frames are zeroed when granted and again when released | `alloc_and_zero` (`src/hardware/broker/dma/map/alloc.rs:35-39`) |
| An untrusted Linux program reaches NONOS services | a guest holds no [capabilities](../overview/glossary.md#capability), so the kernel refuses its NONOS calls, and its Linux calls go to its supervisor | `empty_guest` (`src/process/foreign/spawn.rs:53-79`) |
| A Linux program listens for connections from outside | a bind or listen outside 127.0.0.0/8 is refused | `not_loopback` (`userland/capsule_linux/src/linux/net/policy.rs:33-41`) |
| Someone takes the installed disk | the [data volume](../overview/glossary.md#data-volume) is sealed per sector under a key the [TPM](../overview/glossary.md#tpm) derives | `seal` (`src/fs/cryptoblock/seal.rs:22-48`) |
| Someone edits sectors of the data volume, or moves them to another place | each sector's tag covers its LBA, and a sector that fails is not decrypted | `open_sealed` (`src/fs/cryptoblock/sector_open.rs:41-73`) |
| A changed firmware, [Secure Boot](../overview/glossary.md#secure-boot) setting, bootloader or kernel opens the data volume | the TPM gives a different key, and the volume stays closed without being formatted over | `BOUND_PCRS` (`src/security/tpm/machine_key/pcrs.rs:21-24`) |
| A changed kernel image is booted | on the verdict of its verification module, the loader stops the boot when the kernel's signature fails, in every mode but Development, and when its STARK trailer fails, in every mode | `verify_signature` (`nonos-bootloader/src/boot/crypto/signature/verify.rs:25-40`), `attest_kernel` (`nonos-bootloader/src/boot/attestation/kernel_gate.rs:34-60`) |
| An older signed kernel is put back on a machine with a TPM | the loader refuses a kernel whose signed [rollback index](../overview/glossary.md#rollback-index) is below the TPM floor, and raises the floor to each kernel it admits | `enforce_floor` (`nonos-bootloader/src/boot/crypto/rollback/floor.rs:28-40`), `commit_rollback` (`nonos-bootloader/src/boot/crypto/rollback/commit.rs:26-47`) |
| A changed or unenrolled bootloader starts the kernel and hands over the TPM's event log | before init the kernel replays the log against [PCR](../overview/glossary.md#pcr) 4, holds the loader it names to its STARK slot under the signed [boot-root record](../overview/glossary.md#boot-root-record), and starts no program when that fails | `refuse_unchecked_loader` (`src/kernel_core/init/entry/loader_refusal.rs:27-48`) |
| One capsule fills the keyring, or reads keys another capsule stored | 16 records per capsule, and a record answers only the pid that stored it | `MAX_KEYS_PER_OWNER` (`userland/capsule_keyring/src/store/store_key.rs:39`), `retrieve` (`userland/capsule_keyring/src/store/retrieve.rs:22-35`) |
| A capsule other than the wallet asks the keyring to open or replace the wallet's sealed account key or recovery words | the keyring seals and opens those records only for the wallet's endpoints; a capsule holding `Crypto` and `FileSystem` can still open them without the keyring, as the second table says | `may_use_vault` (`userland/capsule_keyring/src/server/vault_gate/rule.rs:28-37`) |
| Memory read after an orderly shutdown or a warm reboot | devices are stopped, then DMA buffers, process memory, kernel stacks, file system caches, the key vault, the RAM log and the heap are wiped; in 0.9.2 only the installer's restart runs this | `zerostate_shutdown_wipe` (`src/security/hardening/memory_sanitization/api.rs:59-110`) |
| A capsule writes to a serial line someone else can read | the hardened and air-gapped profiles drop `capsule-serial-debug`, so `serial_debug_cap` adds `Debug` to no capsule's grant; the [Linux personality](../overview/glossary.md#linux-personality) is the exception, in the second table | `serial_debug_cap` (`src/capabilities/serial_debug.rs:45-50`) |
| Network code in an image meant to stay offline | the air-gapped profile leaves every network driver, stack and online program out of the image | `networkFeatures` (`tools/nix/config.nix:48-58`) |

[Capsule isolation](capsule-isolation.md) and [Device secrets and keys](device-secrets-and-keys.md) explain these mechanisms in full.

## What NONOS does not address

| Threat | Why it is not covered | Where to look |
|---|---|---|
| Device DMA on a machine with no remapping unit in service, or with AMD-Vi only | the claim goes ahead unconfined and the boot log says so; AMD-Vi is driven only by kernels built with `nonos-iommu-amdvi`, which no [build profile](../overview/glossary.md#build-profile) turns on | `unconfined_allowed` (`src/hardware/broker/confine/posture.rs:32-50`) |
| DMA from a device found at boot that no driver has claimed, or from a device with no PCI requester id | an unclaimed device stays in the [identity domain](../overview/glossary.md#identity-domain), which maps all the memory the kernel manages; a device found through ACPI gets no domain | `pci_address` (`src/hardware/broker/confine/table.rs:35-43`) |
| A device that raises interrupts it should not by writing MSI messages | interrupt remapping is off in every build profile | `nonos-iommu-intremap` in `Cargo.toml` |
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
| A capsule holding `Admin` driving any device | `Admin` passes every broker gate; only the PCI configuration calls ask for `Driver` again | `can_driver` (`src/capabilities/token/types/authority_broker.rs:24-54`), `sys_pci_config_read` (`src/syscall/microkernel/pci.rs:28-35`) |
| A capsule that asks for bits beyond what the image as a whole was built to grant | the image-wide capability ceiling is only logged: spawn prints `[CEILING] not enforced, would refuse` and goes on; the publisher certificate's own ceiling still holds | `would_refuse` (`src/security/image_ceiling/admits.rs:53-60`) |
| An attacker who takes over a capsule's code, for example through a memory bug in it | authority follows the process, not the code: tokens are bound to a pid, address space and boot, and `subject_measurement` is always zero, so the attacker gets every bit the capsule holds | `new_token` (`src/process/caps.rs:38-59`) |
| A memory bug in the kernel, used by someone who knows its layout | the kernel image sits at the same address on every boot, since nothing calls the KASLR code; the compiler adds no stack canaries; and the kernel stack each process makes its system calls on has no guard page | `randomize_layout_from_kaslr` (`src/memory/layout/manager/kaslr_ops.rs:117-125`), `allocate_kernel_stack` (`src/kernel_core/process_spawn/kernel_stack.rs:44-56`) |
| Speculative-execution and other side channels between capsules | on x86_64 each system call entry fills the return stack buffer and sets IBRS where the CPU has it, and STIBP and SSBD are set once at boot, on the boot CPU only; the exit hook that issues VERW is called only from `exec_process`, which nothing in this tree calls; the IBPB hook for context switches has no caller, the L1D flush is never issued, and KPTI is not implemented, so Meltdown-affected parts are not mitigated; cache, timing and power channels are not addressed | `kernel_entry_mitigations` (`src/security/hardening/spectre_mitigations/hooks.rs:22-37`), `exec_process` (`src/process/userspace/transitions.rs:28-33`), `enable_mitigations` (`src/security/hardening/spectre_mitigations/enable.rs:29-44`), `l1d_flush` (`src/security/hardening/spectre_mitigations/enable.rs:46-63`) |
| Files persisted to the capsule store, read straight from the disk | the store is written as given; only records a capsule seals itself are protected | `sys_store_write` (`src/syscall/microkernel/store_write.rs:24-42`) |
| Capsule diagnostics on the [serial console](../overview/glossary.md#serial-console): from service capsules in the standard, qemu and dev profiles, and from the Linux personality in every profile | those profiles keep `capsule-serial-debug`; `LINUX_CAPS` names `Debug` on every build, and the kernel prints the personality's lines unless they come from a Terminal's private run | `debugFeatures` (`tools/nix/config.nix:60-62`), `LINUX_CAPS` (`src/userspace/capsule_linux/spawn.rs:39-43`), `is_private_run` (`src/syscall/microkernel/debug.rs:61-65`) |

Other limits are stated on the pages that cover them. What a network observer sees on each route is on [Privacy networks](../using/privacy-network.md), and what the two anonymity transports lack is on [How the Nym and Anyone transports are built](anonymity-transports.md). What TLS does not check, revocation and name constraints among them, is on [TLS and certificate trust](tls-and-certificates.md). What the bootloader checks is on [Boot chain and signatures](boot-chain-and-signatures.md). On x86_64 every byte capsules draw with `CryptoRandom` is stretched from RDRAND alone, and a machine where no hardware source answers the kernel gets its token key and boot session nonce from the cycle counter; [Randomness and cryptography](randomness-and-cryptography.md#when-a-source-is-missing) gives both.

## The build profile matters

The build profile an image was built with decides what it can ever do. `tools/nix/config.nix` defines them, and `nonos.toml` picks one, `standard` by default.

| Profile | Security posture from the profile definition |
|---|---|
| `standard` | every driver, the desktop, first-boot setup and the installer; capsules may write to the serial console |
| `hardened` | the standard system with [loader policy](../overview/glossary.md#loader-policy) `production`, built without `capsule-serial-debug`, so only the Linux personality can write to the serial console |
| `airgapped` | hardened, with no network driver, stack or online program compiled in |
| `qemu` | the desktop for virtual machines; the host sees everything the guest does |
| `dev` | the development loader policy and path-only attestation; never sealed for release |
| `core` | the microkernel and its base capsules, no desktop |

The profile's `drop` list is taken out of the kernel's features at build time, so a dropped feature is absent from the binary rather than switched off at run time. [Build profiles](../build/profiles.md) has the details.

## Checking a running machine

The Terminal's `log` command reads the copy of the serial console that the kernel keeps in memory, through `mk_log_tail`, which needs `AttestRead` (`userland/capsule_terminal/src/command/builtin/log.rs:32-40`). Only a standard, qemu or dev image keeps that copy; on a hardened or air-gapped image the kernel keeps no copy, so `log` finds no lines (`keep` in `src/sys/serial/tail.rs:51-54`).

```sh
log iommu vt-d
```

Not tested in this release.

It shows whether the DMA of claimed devices is confined: look for a `[VT-D] pid=` line for each claimed PCI device that says `confined to its capsule's domain`, and for `enforcing=1` and `unconfined grants=0` on the last `[IOMMU] <vendor> present` line (`posture_line` in `src/memory/iommu/posture.rs:69-79`). A device found at boot that no driver claimed still reaches all memory through the identity domain, whatever these lines say.

Capability refusals are not visible anywhere in this release. The `[CAP-DENY]` lines go through the kernel's structured `log`, and nothing in the kernel calls its `init` (`src/log/manager/api.rs:26-53`), so they are dropped; [Logging](../kernel/logging.md) explains.

The Security page in Settings shows whether the TPM gave a machine key in this boot state. See [Device secrets and keys](device-secrets-and-keys.md).

The tools in the tree that test these claims, from the capability audit to the tamper tests, are on [Checking the security claims yourself](checking-the-claims.md), with what each printed at this commit.

## See also

- [Threat model](../overview/threat-model.md)
- [Capsule isolation](capsule-isolation.md)
- [Device secrets and keys](device-secrets-and-keys.md)
- [Measured boot and the TPM](measured-boot-and-tpm.md)
- [Rollback protection](rollback-protection.md)
- [TLS and certificate trust](tls-and-certificates.md)
- [How the Nym and Anyone transports are built](anonymity-transports.md)
- [Checking the security claims yourself](checking-the-claims.md)
- [Reporting a vulnerability](reporting-a-vulnerability.md)
