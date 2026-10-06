# Capabilities

How the NONOS kernel decides what a process may ask of it: the [capability](../overview/glossary.md#capability) bits, the token each process holds, the check on every system call, and how bits are granted and taken back.

## One definition

The kernel names every capability once, in the `capability_table` list, with the bit it occupies (`src/capabilities/types/defs.rs:17-83`). There are 36, on bits 0 to 35, and `count` is derived from the list so it cannot drift from it (`src/capabilities/types/table.rs:39-43`).

[abi/caps.toml](../../abi/caps.toml) publishes the same bits for toolchains and [capsule](../overview/glossary.md#capsule) authors. Its `[bits]` table, starting at `CORE_EXEC`, is generated from the kernel list by `scripts/gen_caps_abi.py` (`abi/caps.toml:3-6`). The kernel never reads the file; `scripts/check_caps_abi.py` compares the two:

```
$ python3 scripts/check_caps_abi.py
caps-abi: 36 published bits agree with the kernel
```

## The capability word

A [capability word](../overview/glossary.md#capability-word) is a `u64` with one bit per capability. `caps_to_bits` and `bits_to_caps` convert between the word and a list (`src/capabilities/bits.rs:63-71`). The word is what a [manifest](../overview/glossary.md#manifest) declares, what an [endpoint](../overview/glossary.md#endpoint) requires, and what `MkCapGrant` and `MkCapRevoke` take.

Each process control block holds its [capability token](../overview/glossary.md#capability-token) as the source of truth and `caps_bits` as a cached copy of the word (`src/process/caps.rs:17-22`). `has` asks whether a pid holds every bit of a mask and answers no for an unknown pid (`src/process/caps.rs:82-86`).

## What each bit admits

The gates below are read from the cap table: `check` for the microkernel calls (`src/syscall/contract/cap_table/mk.rs:20-224`), and the crypto, admin and graphics tables beside it. `Admin` also passes the gates of `Driver`, `Mmio`, `Irq`, `Dma`, `Pio`, `InputSource`, `SpawnWindow`, `ProcessControl` and `RegisterService`, and of `MkDeviceList`, for example in `can_driver` (`src/capabilities/token/types/authority_broker.rs:24-26`).

| Bit | Kernel name | What it admits |
|---:|---|---|
| 0 | `CoreExec` | `MkGetPid`, `MkArgs`; with `IPC` and `Memory`, `MkCapsuleLoad` and `MkCapsuleVerify`. |
| 1 | `IO` | Nothing. The kernel list marks it as enforcing nothing. |
| 2 | `Network` | Reaching the network services. Removed on offline boots. A holder is refused `MkAttestDoc`. |
| 3 | `IPC` | The eight IPC calls, `MkThreadSpawn`, `MkWait`, `MkKill`, `MkProcOutput`, `MkProcInput`, `MkStdinRead`, `MkStdoutWrite`, `MkPrivateWrite`, `MkToolRun`, `MkTtySet`. |
| 4 | `Memory` | `MkMmap`, `MkMunmap`. |
| 5 | `Crypto` | The 12 crypto calls, `CRND` to `CMKY`. |
| 6 | `FileSystem` | `MkDataStat`, `MkDataRead`; with `StoreWrite`, `MkDataImport` and `MkDataPassphrase`. |
| 7 | `Hardware` | Nothing. The kernel list marks it as enforcing nothing. |
| 8 | `Debug` | `MkDebug`. |
| 9 | `Admin` | `AdminReboot`, `AdminShutdown`, `AdminPolicyPush`, `MkCapGrant`, `MkCapRevoke`, and the stand in role above. |
| 10 | `RegisterService` | Claiming one of the run time service names. |
| 11 | `GraphicsDisplayQuery` | `GraphicsDisplayDimensions`, `MkDisplayVsyncWait`. |
| 12 | `GraphicsSurfaceCreate` | `MkSurfaceRegister`, `MkSurfaceShare`, `MkSurfaceRelease`. |
| 13 | `GraphicsSurfaceMap` | `MkSurfaceAttach`. |
| 14 | `GraphicsPresent` | `MkSurfacePresent`. |
| 15 | `DeviceEnum` | `MkDeviceList`, `MkInstallSource`. |
| 16 | `Driver` | `MkDeviceClaim`, `MkDeviceRelease`, `MkPciConfigRead`, `MkPciConfigWrite`. |
| 17 | `Mmio` | `MkMmioMap`, `MkMmioUnmap`. |
| 18 | `Irq` | `MkIrqBind`, `MkIrqUnbind`, `MkIrqAck`, `MkIrqPoll`, `MkIrqWait`, and posting input with `MkInputEventPost`. |
| 19 | `Dma` | `MkDmaMap`, `MkDmaUnmap`. |
| 20 | `Pio` | `MkPioGrant`, `MkPioRead`, `MkPioWrite`, `MkPioRelease`. |
| 21 | `InputSource` | `MkInputEventPost`, `MkInputEventDrain`, `MkInputEventWait`. |
| 22 | `TimeSet` | `MkTimeAdjust`. `Admin` does not stand in for it. |
| 23 | `SpawnBroker` | Naming another live pid as the parent of a capsule load. |
| 24 | `SpawnWindow` | `MkSpawnInstance`. |
| 25 | `ProcessControl` | `MkKill` on a process the caller does not parent, and every field of other processes in `MkProcStat`. |
| 26 | `StoreWrite` | `MkStoreWrite`, `MkStoreRead`; with `FileSystem`, the data import calls. |
| 27 | `EnrolDevRoot` | `MkDevRootRequest`, `MkDevRootConfirm`, `MkDevRootLocal`, `MkLocalConsent`, `MkLocalRestore`. `Admin` does not imply it. |
| 28 | `Keyring` | Reaching the keyring capsule; the kernel side checks it in `gate_caller` (`src/security/keyring_capsule/capability.rs:26-34`). |
| 29 | `Entropy` | Drawing from the entropy capsule; the kernel side checks it in `gate_read` (`src/security/entropy_capsule/capability.rs:23-32`). |
| 30 | `AppInstall` | `MkAppInstall`, `MkAppLaunch`, `MkAppInstallStatus`, `MkAppUninstall`. |
| 31 | `AttestRead` | `MkAttestEntries`, `MkLogTail`, every field of other processes in `MkProcStat`; `MkAttestDoc` only without `Network`. |
| 32 | `ForeignExec` | The 15 `MkForeign` and `MkPeer` calls that host a Linux guest, and `MkLocalVerify`. |
| 33 | `LocalSign` | `MkLocalSign`. |
| 34 | `StreamImport` | `MkDataFeedBegin`, `MkDataFeed`, `MkDataRemove`. |
| 35 | `DeviceSecret` | `MkDeviceSecret`, `MkBootSlots`, `MkEnroll`. |

Some calls need only a valid token: `MkExit`, `MkPidAlive`, `MkYield`, the futex and time reads, `MkBatteryStatus`, `MkProcStat`, `MkAttestStatus`, `MkAttestPolicy`, `MkBootAttest` and `MkCapCheck`, listed together under `is_valid` (`src/syscall/contract/cap_table/mk.rs:22-35`). A valid token is one that has not expired and grants at least one capability, as `is_valid` reads (`src/capabilities/token/types/query.rs:44-47`). `MkSetTls` needs the same, since it changes nothing outside the caller (`src/syscall/contract/cap_table/mk.rs:98-102`). `MkTtyQuery` admits every caller whose token passed the resolver (`src/syscall/contract/cap_table/mk.rs:220`). `MkProcStat` shows a caller without `AttestRead` or `ProcessControl` only the identity and state of other processes, as `sees_all` decides (`src/syscall/microkernel/procstat_redact.rs:29-34`).

Two refusals are written into the predicates themselves. `can_attest_doc` refuses a caller that holds `Network`, because a TPM quote names the machine for good (`src/syscall/caps/checks/system.rs:42-47`). `can_input_consumer` leaves out `Irq`, so a device driver may post input but not read the keystroke stream (`src/capabilities/token/types/authority_broker.rs:64-73`).
