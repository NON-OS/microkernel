# Capabilities

The 36 capability bits of NONOS 0.9.2: what each one lets a [capsule](../overview/glossary.md#capsule) do, which syscalls it admits, and where else the kernel checks it.

## What a capability is

A [capability](../overview/glossary.md#capability) is one bit in a 64-bit mask. `capability_table!` is the one list of them, with the bit each occupies (`src/capabilities/types/defs.rs:21-83`), and `count` is derived from that list rather than written down (`src/capabilities/types/table.rs:39-43`). A capsule's [capability token](../overview/glossary.md#capability-token) carries the capabilities it holds as `permissions` (`src/capabilities/token/types/defs.rs:22-25`), and the syscall gates ask the token. The kernel installs a capsule's mask when it starts it, with `install_caps` (`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs:99-100`). On a [boot profile](../overview/glossary.md#boot-profile) that runs no network, `caps` removes `Network` from every capsule's mask (`src/kernel_core/process_spawn/capsule_spawn/runner/profile_gate.rs:48-55`).

`abi/caps.toml` publishes the same bits under upper-case names for toolchains. `scripts/check_caps_abi.py` fails when a published bit disagrees with the kernel, and it passes at this commit.

## The bits

Syscalls it admits lists every call whose cap table gate names the capability, alone, with another (`and`), or as one choice (`or`); see [Syscalls](syscalls.md#the-capability-check) for how each gate reads. "none" means no syscall gate names the bit; the next section says where the kernel checks it instead.

| Bit | Mask | Name | `caps.toml` | Meaning | Syscalls it admits |
|---|---|---|---|---|---|
| 0 | `0x1` | `CoreExec` | `CORE_EXEC` | Run as a process; asked for by the pid and argument calls, and with IPC and Memory by capsule loading. | `MCLD`, `MGPD`, `MKAR`, `MCVF` |
| 1 | `0x2` | `IO` | `IO` | Enforces nothing. | none |
| 2 | `0x4` | `Network` | `NETWORK` | Reach the network services. Removed from every capsule on a boot profile that runs no network. | none |
| 3 | `0x8` | `IPC` | `IPC` | Send and receive messages, and start and drive child processes. | `MISD`, `MIRC`, `MICL`, `MIRF`, `MIRY`, `MISP`, `MSVL`, `MSVR`, `MCLD`, `MWAT`, `MKIL`, `MTSP`, `MOUT`, `MPIN`, `MSRD`, `MTRN`, `MTTY`, `MSOW`, `MPVW`, `MCVF` |
| 4 | `0x10` | `Memory` | `MEMORY` | Map and unmap pages. | `MMAP`, `MUMP`, `MCLD`, `MCVF` |
| 5 | `0x20` | `Crypto` | `CRYPTO` | Use the crypto calls. | `CRND`, `CHSH`, `CENC`, `CDEC`, `CEAD`, `CDAD`, `CXPK`, `CXSH`, `CHMC`, `CHKF`, `CKEC`, `CMKY` |
| 6 | `0x40` | `FileSystem` | `FILE_SYSTEM` | Read the data volume, and reach the VFS service. | `MDIM`, `MDST`, `MDRD`, `MDPW` |
| 7 | `0x80` | `Hardware` | `HARDWARE` | Enforces nothing. | none |
| 8 | `0x100` | `Debug` | `DEBUG` | Write diagnostic lines to the boot serial. | `MDBG` |
| 9 | `0x200` | `Admin` | `ADMIN` | Reboot, shut down, push policy, grant and revoke capabilities. Accepted in place of most [broker](../overview/glossary.md#broker) and window rights. | `MCGT`, `MCRV`, `MDLS`, `MDCL`, `MDRL`, `MMMP`, `MMUM`, `MIRB`, `MIRU`, `MIRA`, `MIRP`, `MIRW`, `MDMM`, `MDMU`, `MPCR`, `MPCW`, `MPGT`, `MPRD`, `MPWR`, `MPRL`, `MIEP`, `MIED`, `MIEW`, `MSPI`, `ARBT`, `ASDN`, `APPS` |
| 10 | `0x400` | `RegisterService` | `REGISTER_SERVICE` | Register a service name at run time, from a short fixed list. | none |
| 11 | `0x800` | `GraphicsDisplayQuery` | `GRAPHICS_DISPLAY_QUERY` | Read the display size and wait on the kernel's frame clock. | `MDVW`, `GDIM` |
| 12 | `0x1000` | `GraphicsSurfaceCreate` | `GRAPHICS_SURFACE_CREATE` | Register, share and release surfaces. | `MSRG`, `MSSH`, `MSRL` |
| 13 | `0x2000` | `GraphicsSurfaceMap` | `GRAPHICS_SURFACE_MAP` | Map a surface another capsule shared. | `MSAT` |
| 14 | `0x4000` | `GraphicsPresent` | `GRAPHICS_PRESENT` | Present a surface to the display. | `MSPR` |
| 15 | `0x8000` | `DeviceEnum` | `DEVICE_ENUM` | List broker devices, and read the booted image for an install. Claiming a device is `Driver`. | `MISR`, `MDLS` |
| 16 | `0x10000` | `Driver` | `DRIVER` | Claim and release a device through the broker, and use its PCI configuration space. | `MDCL`, `MDRL`, `MPCR`, `MPCW` |
| 17 | `0x20000` | `Mmio` | `MMIO` | Map a slice of a claimed device's BAR into the holder's own address space. | `MMMP`, `MMUM` |
| 18 | `0x40000` | `Irq` | `IRQ` | Bind a claimed device's interrupt and wait on it. Also lets a driver post input events. | `MIRB`, `MIRU`, `MIRA`, `MIRP`, `MIRW`, `MIEP` |
| 19 | `0x80000` | `Dma` | `DMA` | Receive a DMA buffer a claimed device may read or write through. | `MDMM`, `MDMU` |
| 20 | `0x100000` | `Pio` | `PIO` | Get a port grant on a port BAR and have the kernel run `in` and `out` on its ports. | `MPGT`, `MPRD`, `MPWR`, `MPRL` |
| 21 | `0x200000` | `InputSource` | `INPUT_SOURCE` | Post input events, and drain and wait on the input ring. | `MIEP`, `MIED`, `MIEW` |
| 22 | `0x400000` | `TimeSet` | `TIME_SET` | Correct the wall clock. | `MTAD` |
| 23 | `0x800000` | `SpawnBroker` | `SPAWN_BROKER` | Attribute a capsule load's parent to a live pid other than the caller. | none |
| 24 | `0x1000000` | `SpawnWindow` | `SPAWN_WINDOW` | Open another window of an embedded, attested app capsule. | `MSPI` |
| 25 | `0x2000000` | `ProcessControl` | `PROCESS_CONTROL` | Terminate a process the caller does not parent, and read the full process table. | none |
| 26 | `0x4000000` | `StoreWrite` | `STORE_WRITE` | Read and write the package store; with FileSystem, import into and key the data volume. | `MSWR`, `MSRR`, `MDIM`, `MDPW` |
| 27 | `0x8000000` | `EnrolDevRoot` | `ENROL_DEV_ROOT` | Enrol a signing root, so capsules built on this machine run on it. `Admin` does not imply it. | `MDRQ`, `MDRC`, `MDRO`, `MLCG`, `MLCR` |
| 28 | `0x10000000` | `Keyring` | `KEYRING` | Reach the keyring capsule, which stores and derives key material. | none |
| 29 | `0x20000000` | `Entropy` | `ENTROPY` | Draw from the entropy capsule. | none |
| 30 | `0x40000000` | `AppInstall` | `APP_INSTALL` | Ask for a marketplace install, launch or removal, and reach the marketplace. | `MAIN`, `MAPL`, `MAIS`, `MAUN` |
| 31 | `0x80000000` | `AttestRead` | `ATTEST_READ` | Read the attestation registry, the kernel log tail and the full process table. The signed document also needs the caller to lack Network. | `MADC`, `MAEN`, `MLOG` |
| 32 | `0x100000000` | `ForeignExec` | `FOREIGN_EXEC` | Host code this kernel has not verified: create a process with no capabilities, build its address space, and answer its refused syscalls. | `MFSP`, `MFST`, `MFWT`, `MFRP`, `MFCX`, `MFSG`, `MFIN`, `MPMP`, `MPCP`, `MPPT`, `MFTH`, `MPTL`, `MFFK`, `MPUN`, `MFEX`, `MLVF` |
| 33 | `0x200000000` | `LocalSign` | `LOCAL_SIGN` | Mint a proof that this machine agreed to run bytes it installed itself. | `MLSG` |
| 34 | `0x400000000` | `StreamImport` | `STREAM_IMPORT` | Stream a file into the data volume, kept only if it hashes to the digest named first. Grants no read. | `MDFB`, `MDFD`, `MDRM` |
| 35 | `0x800000000` | `DeviceSecret` | `DEVICE_SECRET` | Receive this machine's TPM-derived device secret and its boot slots, and run enrolment's TPM half. | `MDVS`, `MBSL`, `MENR` |

## Checks outside the syscall table

Several bits gate a service or a path inside a call rather than a syscall:

- `Network`: an [endpoint](../overview/glossary.md#endpoint) in `NETWORK_SERVICES` requires it on top of `IPC`, through `required_caps` (`src/services/registry/policy.rs:26-44`). See [IPC](ipc.md).
- `RegisterService`: `caller_has_register_right` accepts it, or `Admin`, before a capsule may claim a runtime-registrable name (`src/services/registry/auth/caller_has_register_right.rs:17-20`).
- `SpawnBroker`: `attest` honours a capsule load's `on_behalf_of` pid only for a caller holding it (`src/kernel_core/process_spawn/capsule_spawn/attested_parent.rs:36-45`).
- `ProcessControl`: `sys_kill` lets a holder, or an `Admin` holder, end a process it does not parent or supervise (`src/syscall/microkernel/kill.rs:45-54`), and `sees_all` shows it the full process table (`src/syscall/microkernel/procstat_redact.rs:29-34`).
- `Keyring`: `gate_caller` refuses every keyring operation to a caller without it (`src/security/keyring_capsule/capability.rs:26-35`).
- `Entropy`: `gate_read` refuses entropy reads to a caller without it (`src/security/entropy_capsule/capability.rs:23-31`).
- `IO` and `Hardware` are marked "Enforces nothing" in the table itself (`src/capabilities/types/defs.rs:23-31`).

## Rules the predicates add

- `Admin` stands in for `Driver`, `Mmio`, `Irq`, `Dma` and `Pio`: each predicate, such as `can_driver`, accepts either (`src/capabilities/token/types/authority_broker.rs:24-54`). It also stands in for `DeviceEnum` when listing devices, for `SpawnWindow`, and for `ProcessControl`, through `can_device_enum`, `can_spawn_window` and `can_control_processes` (`src/capabilities/token/types/authority_admin.rs:30-57`).
- `Admin` does not imply `EnrolDevRoot`: `can_enrol_dev_root` asks for that bit alone (`src/capabilities/token/types/authority_admin.rs:43-52`). It does not stand in for `DeviceEnum` on `MISR`, since `can_install_source` asks for `DeviceEnum` alone (`src/syscall/caps/checks/hardware.rs:28-31`).
- `Irq` lets a driver post input events but not read them: `can_input_source` accepts `Irq`, and `can_input_consumer`, which gates draining and waiting, does not (`src/capabilities/token/types/authority_broker.rs:55-73`).
- `can_attest_doc` refuses a caller that holds `Network`, whatever else it holds, because a TPM quote names the machine for good (`src/syscall/caps/checks/system.rs:42-52`).

## Changing capabilities at run time

Three calls work on another process's mask. `sys_cap_grant` needs `Admin`, and the caller must already hold every bit it grants, so a grant never creates authority (`src/syscall/microkernel/capability/handlers.rs:33-52`). `sys_cap_revoke` needs `Admin` (`src/syscall/microkernel/capability/handlers.rs:54-66`). Both mint the target a fresh token, and `revoke` also raises the target's revocation epoch first, so a copy of its old token fails `check_revocation_epoch` (`src/process/caps.rs:99-108`). `sys_cap_check` returns 1 when the target holds every bit of the mask and 0 otherwise, for any caller with a valid token (`src/syscall/microkernel/capability/handlers.rs:68-74`). `u32_arg` refuses the pid argument, rather than truncating it, when it does not fit in 32 bits (`src/syscall/microkernel/dispatch/capability.rs:30-31`).

## Groups and delegation

`abi/caps.toml` also publishes named groups and the rights each group may hand to another. They are policy, written by hand; the file says each group is the mask of a real capsule, `SERVICE` for one being the keyring's (`abi/caps.toml:48-51`). The kernel does not read this file.

| Entry | Capabilities |
|---|---|
| `BASIC` | `CORE_EXEC`, `MEMORY` |
| `SERVICE` | `CORE_EXEC`, `MEMORY`, `IPC` |
| `OPER` | `CORE_EXEC`, `MEMORY`, `IPC`, `PROCESS_CONTROL` |
| `GRAPHICS_SERVICE` | `CORE_EXEC`, `MEMORY`, `IPC`, `GRAPHICS_DISPLAY_QUERY`, `GRAPHICS_SURFACE_CREATE`, `GRAPHICS_SURFACE_MAP`, `GRAPHICS_PRESENT` |
| `BASIC_to_BASIC` | `CORE_EXEC`, `MEMORY` |
| `OPER_to_SERVICE` | `CORE_EXEC`, `MEMORY`, `IPC` |
| `OPER_to_BASIC` | `CORE_EXEC`, `MEMORY` |
| `SERVICE_to_BASIC` | `CORE_EXEC`, `MEMORY` |
