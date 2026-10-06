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

## What NONOS does not address

| Threat | Why it is not covered | Where to look |
|---|---|---|
| Device DMA on a machine with no remapping unit in service, or with AMD-Vi only | the claim goes ahead unconfined and the boot log says so; AMD-Vi is driven only by kernels built with `nonos-iommu-amdvi`, which is off by default | `unconfined_allowed` (`src/hardware/broker/confine/posture.rs:32-50`) |
| A device that raises interrupts it should not by writing MSI messages | interrupt remapping is off by default | `nonos-iommu-intremap` in `Cargo.toml` |
| Reading RAM after a power cut, a forced power-off or a kernel panic | none of these runs the wipe; the panic path halts as it is | `panic` (`src/boot/panic/handler.rs:41-64`) |
| The data volume key left in RAM after an orderly shutdown | the wipe covers the heap, stacks and process memory, not kernel statics such as the one holding this key | `VOLUME` (`src/fs/blockfs_volume/state.rs:21-26`) |
| A probe on the bus between the CPU and a discrete TPM | the TPM session is unbound and unsalted, with no parameter encryption, so the machine key crosses the bus in the clear | `build_start` (`src/security/tpm/machine_key/session.rs:17-49`) |
| Malicious firmware, SMM code or a management engine | it runs below the kernel; NONOS can neither inspect nor contain it, and the PCR binding only notices a changed measurement | `BOUND_PCRS` (`src/security/tpm/machine_key/pcrs.rs:21-24`) |
| Putting back an older copy of the data volume or of a sealed record | sectors are bound to their LBA, not to a version, and sealed records carry no counter | `AAD_PREFIX` (`src/fs/cryptoblock/seal.rs:39-42`), `userland/nonos_vault/src/lib.rs` |
| A capsule that misuses the capabilities it was granted | a capability says what kind of thing a capsule may do, not who it may do it with; only the capsule named in `PEERS` is held to a list | `PEERS` (`src/services/registry/peers.rs:17-31`) |
| A capsule holding `Crypto` deriving a machine key another capsule uses | the kernel does not tie a label to a capsule; it refuses only the kernel's own labels | `is_user_label` (`src/security/tpm/machine_key/kernel_label.rs:42-45`) |
| A capsule holding `Admin` driving any device | `Admin` stands in for every broker bit | `can_driver` (`src/capabilities/token/types/authority_broker.rs:24-54`) |
| An attacker who takes over a capsule's code, for example through a memory bug in it | authority follows the process, not the code: tokens are bound to a pid, address space and boot, and `subject_measurement` is always zero, so the attacker gets every bit the capsule holds | `new_token` (`src/process/caps.rs:38-59`) |
| Speculative-execution and other side channels between capsules | on x86_64 each system call entry fills the return stack buffer and sets IBRS where the CPU has it; the exit hook that issues VERW is called only from `exec_process`, which nothing in this tree calls; the IBPB hook for context switches has no caller, the L1D flush is never issued, and KPTI is not implemented, so Meltdown-affected parts are not mitigated; cache, timing and power channels are not addressed | `kernel_entry_mitigations` (`src/security/hardening/spectre_mitigations/hooks.rs:22-37`), `exec_process` (`src/process/userspace/transitions.rs:28-33`), `l1d_flush` (`src/security/hardening/spectre_mitigations/enable.rs:46-63`) |
| Files persisted to the capsule store, read straight from the disk | the store is written as given; only records a capsule seals itself are protected | `sys_store_write` (`src/syscall/microkernel/store_write.rs:24-42`) |
| Capsule diagnostics on the serial console in the standard, qemu and dev profiles | those profiles keep `capsule-serial-debug`, so service capsules may write to serial | `debugFeatures` (`tools/nix/config.nix:60-62`) |

Two further limits are stated on other pages. Network observers and the anonymity routes are on [Privacy network](../using/privacy-network.md). What the bootloader checks is on [Boot chain and signatures](boot-chain-and-signatures.md).

## The build profile matters

The profile an image was built with decides what it can ever do. `tools/nix/config.nix` defines them, and `nonos.toml` picks one, `standard` by default.

| Profile | Security posture from the profile definition |
|---|---|
| `standard` | every driver, the desktop, first-boot setup and the installer; capsules may write to the serial console |
| `hardened` | the standard system with loader policy `production` and no serial console for capsules |
| `airgapped` | hardened, with no network driver, stack or online program compiled in |
| `qemu` | the desktop for virtual machines; the host sees everything the guest does |
| `dev` | the development loader policy and path-only attestation; never sealed for release |
| `core` | the microkernel and its base capsules, no desktop |

The profile's `drop` list is taken out of the kernel's features at build time, so a dropped feature is absent from the binary rather than switched off at run time. [Build profiles](../build/profiles.md) has the details.

## Checking a running machine

The Terminal's `log` command reads the copy of the serial console that the kernel keeps in memory, through `mk_log_tail`, which needs `AttestRead` (`userland/capsule_terminal/src/command/builtin/log.rs:32-40`). Only a standard, qemu or dev image keeps that copy; on a hardened or air-gapped image `keep` stores nothing, so `log` finds no lines (`src/sys/serial/tail.rs:51-54`).

```sh
log iommu vt-d
```

Not tested in this release.

It shows whether device DMA is confined: look for a `[VT-D]` line for each claimed device, and for `enforcing=1` and `unconfined grants=0` on the last `[IOMMU]` line.

`log` does not show capability refusals. The `[CAP-DENY]` lines go to the kernel's log manager, which writes warnings to the text-mode console and every line to its `ram_buffer`, not to the serial console (`src/log/manager/state.rs:58-82`).

The Security page in Settings shows whether the TPM gave a machine key in this boot state. See [Device secrets and keys](device-secrets-and-keys.md).
