# Protections and limits: How to read this page

Part of [Protections and limits](protections-and-limits.md).

## How to read this page

Each row names a threat, what the code does about it, and where. A row in the
second table is a limit of this release, stated so you can decide what to trust
NONOS with. Neither table is a promise beyond the code it cites.

## What NONOS addresses

| Threat | What the code does | Where |
|---|---|---|
| A [capsule](../overview/glossary.md#capsule) makes a system call its [manifest](../overview/glossary.md#manifest) did not grant | the call is refused with `EPERM` before its handler runs | `dispatch` (`src/syscall/contract/dispatch.rs:31-40`) |
| A capsule forges a token, or reuses one from another process or an earlier boot | no call takes a token from a capsule: `Capability::resolve` reads the caller's token from its process entry in the kernel, then checks its MAC under a key drawn at each boot, its boot nonce, address space and revocation epoch | `resolve` (`src/syscall/contract/capability.rs:34-46`, `src/syscall/contract/resolver/resolve.rs:31-43`) |
| A capsule asks for more authority than its [publisher](../overview/glossary.md#publisher) may grant | the manifest's bits must fit under the [ceiling](../overview/glossary.md#capability-ceiling) in the publisher's certificate | `check_ceiling` (`src/security/capsule_manifest/verify/caps.rs:21-29`) |
| A capsule asks for bits beyond what the image as a whole was built to grant | spawn is refused, and the log names the capsule and each bit over: `[CEILING] refused <name>: image forbids ...` | `gate` (`src/security/image_ceiling/rule.rs`), called in `preflight::run` |
| A capsule reads or writes another capsule's memory | every process has its own page tables, with only the kernel half shared | `create_address_space` (`src/memory/paging/manager/address_space/create.rs:37-48`) |
| A capsule hands the kernel a pointer into kernel memory | a range that ends above the user half is refused, and every page must carry the user bit | `check_range` (`src/usercopy/policy.rs:35-50`) |
| The kernel is made to run code from a capsule's pages, or to touch them outside its copy routines | SMEP and SMAP where CPUID reports them, with NX and CR0.WP, are set on every CPU and read back; a CPU without NX does not boot, a secondary CPU weaker than the boot CPU runs no capsule code, and no mapping may be both writable and executable | `apply` (`src/memory/mmu/mmu/protect/apply.rs:26-44`), `finish` (`src/smp/ap/user_setup.rs:46-59`), `is_wx_violation` (`src/memory/paging/manager/mapping/map.rs:41-43`) |
| A capsule sends to a service it has no rights to | the sender must hold every bit the [endpoint](../overview/glossary.md#endpoint) requires, and an unregistered name is refused | `caller_satisfies_endpoint` (`src/syscall/microkernel/ipc/send_caps.rs:36-63`) |
| A capsule talks to a device driver directly, for example to put raw frames on the wire | the driver endpoints in `HELD` accept only the services named for them, and the three PCI storage drivers check each sender for `StoreWrite` themselves | `HELD` (`src/services/registry/held_table.rs:20-36`) |
| A [driver capsule](../overview/glossary.md#driver-capsule) reads another program's keystrokes | draining the input ring needs `InputConsumer`, which input_router alone holds; `Irq` and `Admin` are not enough | `can_input_consumer` (`src/capabilities/token/types/authority_broker.rs:64-66`) |
| A capsule holding `Admin` driving any device | every broker gate takes its own bit alone; `Admin` passes none | `can_driver` (`src/capabilities/token/types/authority_broker.rs:29-31`) |
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

[Capsule isolation](capsule-isolation.md) and [Device secrets and
keys](device-secrets-and-keys.md) explain these mechanisms in full.
