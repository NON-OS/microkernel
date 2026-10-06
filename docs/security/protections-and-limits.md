# Protections and limits

What NONOS 0.9.2 protects against, with the code that does it, and what it does not protect against, with the reason.

## How to read this page

Each row names a threat, what the code does about it, and where. A row in the second table is a limit of this release, stated so you can decide what to trust NONOS with. Neither table is a promise beyond the code it cites.

## What NONOS addresses

| Threat | What the code does | Where |
|---|---|---|
| A [capsule](../overview/glossary.md#capsule) makes a system call its manifest did not grant | the call is refused with `EPERM` before its handler runs | `dispatch` (`src/syscall/contract/dispatch.rs:31-40`) |
| A capsule forges a token, or reuses one from another process or an earlier boot | the token's MAC is checked under a key drawn at each boot, then its boot nonce, address space and revocation epoch | `resolve` (`src/syscall/contract/resolver/resolve.rs:31-43`) |
| A capsule asks for more authority than its publisher may grant | the manifest's bits must fit under the ceiling in the publisher's certificate | `check_ceiling` (`src/security/capsule_manifest/verify/caps.rs:21-29`) |
| A capsule reads or writes another capsule's memory | every process has its own page tables, with only the kernel half shared | `create_address_space` (`src/memory/paging/manager/address_space/create.rs:37-48`) |
| A capsule hands the kernel a pointer into kernel memory | a range that ends above the user half is refused, and every page must carry the user bit | `check_range` (`src/usercopy/policy.rs:35-50`) |
| A capsule sends to a service it has no rights to | the sender must hold every bit the [endpoint](../overview/glossary.md#endpoint) requires, and an unregistered name is refused | `caller_satisfies_endpoint` (`src/syscall/microkernel/ipc/send_caps.rs:36-63`) |
| A capsule talks to a device driver directly, for example to put raw frames on the wire | driver endpoints accept only the services named for them | `HELD` (`src/services/registry/held_table.rs:20-36`) |
| A driver capsule reads another program's keystrokes | draining the input ring needs `InputSource`, and holding `Irq` is not enough | `can_input_consumer` (`src/capabilities/token/types/authority_broker.rs:64-73`) |
| A device writes outside its driver's buffers by DMA, on a machine with VT-d in service | each driver capsule gets its own [IOMMU domain](../overview/glossary.md#iommu-domain), and a claim the unit cannot confine is refused | `attach` (`src/hardware/broker/confine/attach.rs:30-101`) |
| A driver reprograms a BAR or other PCI config space | only a short allowlist of bits may change | `validate` (`src/hardware/broker/pci/allowlist.rs:37-60`) |
| A driver maps an MSI-X table to redirect interrupts | MSI-X tables and pending-bit arrays are kept out of every MMIO mapping | `protected_regions` (`src/hardware/broker/mmio/msix_exclusion.rs:39-52`) |
| One DMA buffer's contents reach the next holder of its frames | frames are zeroed when granted and again when released | `alloc_and_zero` (`src/hardware/broker/dma/map/alloc.rs:35-39`) |
| An untrusted Linux program reaches NONOS services | a guest holds no capabilities, and its system calls go to its supervisor | `empty_guest` (`src/process/foreign/spawn.rs:53-79`) |
| A Linux program listens for connections from outside | a bind or listen outside 127.0.0.0/8 is refused | `not_loopback` (`userland/capsule_linux/src/linux/net/policy.rs:33-41`) |
| Someone takes the installed disk | the [data volume](../overview/glossary.md#data-volume) is sealed per sector under a key the TPM derives | `seal` (`src/fs/cryptoblock/seal.rs:22-48`) |
| Someone edits sectors of the data volume, or moves them to another place | each sector's tag covers its LBA, and a sector that fails is not decrypted | `open_sealed` (`src/fs/cryptoblock/sector_open.rs:41-73`) |
| A changed firmware, Secure Boot setting, bootloader or kernel opens the data volume | the TPM gives a different key, and the volume stays closed without being formatted over | `BOUND_PCRS` (`src/security/tpm/machine_key/pcrs.rs:21-24`) |
| One capsule fills the keyring, or reads keys another capsule stored | 16 records per capsule, and a record answers only the pid that stored it | `MAX_KEYS_PER_OWNER` (`userland/capsule_keyring/src/store/store_key.rs:39`), `retrieve` (`userland/capsule_keyring/src/store/retrieve.rs:22-35`) |
| A capsule other than the wallet opens or replaces the wallet's sealed account key or recovery words | the keyring seals and opens those records only for the wallet's endpoints | `may_use_vault` (`userland/capsule_keyring/src/server/vault_gate/rule.rs:28-37`) |
| Memory read after an orderly shutdown or a warm reboot | devices are stopped, then DMA buffers, process memory, kernel stacks, the RAM log and the heap are wiped | `zerostate_shutdown_wipe` (`src/security/hardening/memory_sanitization/api.rs:59-110`) |
| A capsule writes to a serial line someone else can read | the hardened and air-gapped profiles build without the `Debug` grant | `serial_debug_cap` (`src/capabilities/serial_debug.rs:45-50`) |
| Network code in an image meant to stay offline | the air-gapped profile leaves every network driver, stack and online program out of the image | `networkFeatures` (`tools/nix/config.nix:48-58`) |

[Capsule isolation](capsule-isolation.md) and [Device secrets and keys](device-secrets-and-keys.md) explain these mechanisms in full.
