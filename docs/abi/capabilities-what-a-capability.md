# Capabilities: What a capability is

Part of [Capabilities](capabilities.md).

## What a capability is

A [capability](../overview/glossary-boot-root-record.md#capability) is one bit in a 64-bit mask,
the process's [capability word](../overview/glossary-boot-root-record.md#capability-word).
`capability_table!` is the one list of them, with the bit each occupies
(`src/capabilities/types/defs.rs:19-75`), and `count` is derived from that list
rather than written down (`src/capabilities/types/table.rs:39-43`). A capsule's
[capability token](../overview/glossary-boot-root-record.md#capability-token) carries the
capabilities it holds as `permissions`
(`src/capabilities/token/types/defs.rs:22-25`), and the syscall gates ask the
token. The kernel installs a capsule's mask when it starts it, with
`install_caps`
(`src/kernel_core/process_spawn/capsule_spawn/runner/install/install.rs:99-100`).
On a [boot profile](../overview/glossary-amnesic-boot.md#boot-profile) that runs no network,
`caps` removes `Network` from every capsule's mask
(`src/kernel_core/process_spawn/capsule_spawn/runner/profile_gate.rs:48-55`).

`abi/caps.toml` publishes the same bits under upper-case names for toolchains.
`scripts/check_caps_abi.py` fails when a published bit disagrees with the
kernel, and it passes at this commit.

## The bits

Syscalls it admits lists every call whose cap table gate names the capability,
alone, with another (`and`), or as one choice (`or`); see
[Syscalls](syscalls-the-capability-check.md#the-capability-check) for how each gate reads. "none"
means no syscall gate names the bit; the next section says where the kernel
checks it instead.

| Bit | Mask | Name | `caps.toml` | Meaning | Syscalls it admits |
|---|---|---|---|---|---|
| 0 | `0x1` | `CoreExec` | `CORE_EXEC` | Run as a process; asked for by the pid and argument calls, and with IPC and Memory by capsule loading. | `MCLD`, `MGPD`, `MKAR`, `MCVF` |
| 1 | `0x2` | `IO` | `IO` | Enforces nothing. | none |
| 2 | `0x4` | `Network` | `NETWORK` | Reach the network services. Removed from every capsule on a boot profile that runs no network. | none |
| 3 | `0x8` | `IPC` | `IPC` | Send and receive messages, and start and drive child processes. | `MISD`, `MIRC`, `MICL`, `MIRF`, `MIRY`, `MISP`, `MSVL`, `MSVR`, `MCLD`, `MWAT`, `MKIL`, `MTSP`, `MOUT`, `MPIN`, `MSRD`, `MTRN`, `MTTY`, `MSOW`, `MPVW`, `MCVF`, `MPJB` |
| 4 | `0x10` | `Memory` | `MEMORY` | Map and unmap pages. | `MMAP`, `MUMP`, `MCLD`, `MCVF` |
| 5 | `0x20` | `Crypto` | `CRYPTO` | Use the crypto calls. | `CRND`, `CHSH`, `CENC`, `CDEC`, `CEAD`, `CDAD`, `CXPK`, `CXSH`, `CHMC`, `CHKF`, `CKEC`, `CMKY` |
| 6 | `0x40` | `FileSystem` | `FILE_SYSTEM` | Read the data volume, and reach the VFS service. | `MDIM`, `MDST`, `MDRD`, `MDPW` |
| 7 | `0x80` | `Hardware` | `HARDWARE` | Enforces nothing. | none |
| 8 | `0x100` | `Debug` | `DEBUG` | Write diagnostic lines to the boot serial. | `MDBG` |
| 9 | `0x200` | `Admin` | `ADMIN` | Reboot, shut down, push policy, grant and revoke capabilities. Accepted in place of the window right, and of `DeviceEnum` when listing devices; never of a [broker](../overview/glossary-hardware-broker.md#broker) or input right. | `MCGT`, `MCRV`, `MDLS`, `MSPI`, `ARBT`, `ASDN`, `APPS` |
| 10 | `0x400` | `RegisterService` | `REGISTER_SERVICE` | Register a service name at run time, from a short fixed list. | none |
| 11 | `0x800` | `GraphicsDisplayQuery` | `GRAPHICS_DISPLAY_QUERY` | Read the display size and wait on the kernel's frame clock. | `MDVW`, `GDIM` |
| 12 | `0x1000` | `GraphicsSurfaceCreate` | `GRAPHICS_SURFACE_CREATE` | Register, share and release surfaces. | `MSRG`, `MSSH`, `MSRL` |
| 13 | `0x2000` | `GraphicsSurfaceMap` | `GRAPHICS_SURFACE_MAP` | Map a surface another capsule shared. | `MSAT` |
| 14 | `0x4000` | `GraphicsPresent` | `GRAPHICS_PRESENT` | Present a surface to the display. | `MSPR` |
| 15 | `0x8000` | `DeviceEnum` | `DEVICE_ENUM` | List broker devices, and read the booted image for an install. Claiming a device is `Driver`. | `MISR`, `MDLS` |
| 16 | `0x10000` | `Driver` | `DRIVER` | Claim and release a device through the broker, and use its PCI configuration space. | `MDCL`, `MDRL`, `MPCR`, `MPCW` |
| 17 | `0x20000` | `Mmio` | `MMIO` | Map a slice of a claimed device's BAR into the holder's own address space. | `MMMP`, `MMUM`, `MBFB` |
| 18 | `0x40000` | `Irq` | `IRQ` | Bind a claimed device's interrupt and wait on it. | `MIRB`, `MIRU`, `MIRA`, `MIRP`, `MIRW` |
| 19 | `0x80000` | `Dma` | `DMA` | Receive a DMA buffer a claimed device may read or write through. | `MDMM`, `MDMU` |
| 20 | `0x100000` | `Pio` | `PIO` | Get a port grant on a port BAR and have the kernel run `in` and `out` on its ports. | `MPGT`, `MPRD`, `MPWR`, `MPRL` |
| 21 | `0x200000` | `InputSource` | `INPUT_SOURCE` | Post input events. | `MIEP` |
| 22 | `0x400000` | `TimeSet` | `TIME_SET` | Correct the wall clock. | `MTAD` |
| 23 | `0x800000` | `SpawnBroker` | `SPAWN_BROKER` | Attribute a capsule load's parent to a live pid other than the caller. | none |
| 24 | `0x1000000` | `SpawnWindow` | `SPAWN_WINDOW` | Open another window of an embedded, attested app capsule. | `MSPI` |
| 25 | `0x2000000` | `ProcessControl` | `PROCESS_CONTROL` | Terminate a process the caller does not parent, and read the full process table. | none |
| 26 | `0x4000000` | `StoreWrite` | `STORE_WRITE` | Read and write the package store; with FileSystem, import into and key the data volume. | `MSWR`, `MSRR`, `MDIM`, `MDPW` |
| 27 | `0x8000000` | `EnrolDevRoot` | `ENROL_DEV_ROOT` | Enrol a signing root, so capsules built on this machine run on it. `Admin` does not imply it. | `MDRQ`, `MDRC`, `MDRO`, `MLCG`, `MLCR` |
| 28 | `0x10000000` | `Keyring` | `KEYRING` | Meant to gate the keyring capsule. Only the kernel's keyring client asks for it, and no kernel code calls that client in 0.9.2. | none |
| 29 | `0x20000000` | `Entropy` | `ENTROPY` | Meant to gate the entropy capsule. Only the kernel client's statistics and health check ask for it, and no kernel code calls them in 0.9.2. | none |
| 30 | `0x40000000` | `AppInstall` | `APP_INSTALL` | Ask for a marketplace install, launch or removal, and reach the marketplace. | `MAIN`, `MAPL`, `MAIS`, `MAUN` |
| 31 | `0x80000000` | `AttestRead` | `ATTEST_READ` | Read the attestation registry, the kernel log tail and the full process table. The signed document also needs the caller to lack Network. | `MADC`, `MAEN`, `MLOG` |
| 32 | `0x100000000` | `ForeignExec` | `FOREIGN_EXEC` | Host code this kernel has not verified: create a process with no capabilities, build its address space, and answer its refused syscalls. | `MFSP`, `MFST`, `MFWT`, `MFRP`, `MFCX`, `MFSG`, `MFIN`, `MPMP`, `MPCP`, `MPPT`, `MFTH`, `MPTL`, `MFFK`, `MPUN`, `MFEX`, `MLVF`, `MLRN`, `MSNT` |
| 33 | `0x200000000` | `LocalSign` | `LOCAL_SIGN` | Mint a proof that this machine agreed to run bytes it installed itself. | `MLSG` |
| 34 | `0x400000000` | `StreamImport` | `STREAM_IMPORT` | Stream a file into the data volume, kept only if it hashes to the digest named first. Grants no read. | `MDFB`, `MDFD`, `MDRM` |
| 35 | `0x800000000` | `DeviceSecret` | `DEVICE_SECRET` | Receive this machine's TPM-derived device secret and its boot slots, and run enrolment's TPM half. | `MDVS`, `MBSL`, `MENR`, `MSLR` |
| 36 | `0x1000000000` | `InputConsumer` | `INPUT_CONSUMER` | Drain and wait on the input ring, which carries every keystroke. Held by input_router alone. | `MIED`, `MIEW` |
| 37 | `0x2000000000` | `CameraCapture` | `CAMERA_CAPTURE` | Take frames from a camera. The kernel gates no call with it; the USB stack and the compositor's indicator check a peer with `MkCapCheck`. Held by no capsule until Decision 9 (`scripts/check_capture_caps.py`). | none |
| 38 | `0x4000000000` | `MicrophoneCapture` | `MICROPHONE_CAPTURE` | Take samples from a microphone, checked as `CameraCapture` is by the audio service and the indicator. Held by no capsule until Decision 9. | none |
| 39 | `0x8000000000` | `ScreenCapture` | `SCREEN_CAPTURE` | Copy the whole screen through the compositor, which shows a notice while it does. Held by no capsule until Decision 9. | none |
