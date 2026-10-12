# Capabilities: What each bit admits

Part of [Capabilities](capabilities.md).

## What each bit admits

The gates below are read from the cap table: `check` for the microkernel calls
(`src/syscall/contract/cap_table/mk.rs` and the `mk_*.rs` files beside it), and
the crypto, admin and graphics tables beside it. `Admin` stands in for
`SpawnWindow` and `RegisterService`, for `DeviceEnum` in `MkDeviceList` but not
in `MkInstallSource`, and for `ProcessControl` in `MkKill` but not in
`MkProcStat`. It stands in for no broker or input bit: `can_driver` and its
siblings ask for their own bit alone
(`src/capabilities/token/types/authority_broker.rs:29-66`).

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
| 18 | `Irq` | `MkIrqBind`, `MkIrqUnbind`, `MkIrqAck`, `MkIrqPoll`, `MkIrqWait`. |
| 19 | `Dma` | `MkDmaMap`, `MkDmaUnmap`. |
| 20 | `Pio` | `MkPioGrant`, `MkPioRead`, `MkPioWrite`, `MkPioRelease`. |
| 21 | `InputSource` | `MkInputEventPost`. |
| 22 | `TimeSet` | `MkTimeAdjust`. `Admin` does not stand in for it. |
| 23 | `SpawnBroker` | Naming another live pid as the parent of a capsule load. |
| 24 | `SpawnWindow` | `MkSpawnInstance`. |
| 25 | `ProcessControl` | `MkKill` on a process the caller does not parent, and every field of other processes in `MkProcStat`. |
| 26 | `StoreWrite` | `MkStoreWrite`, `MkStoreRead`; with `FileSystem`, the data import calls. |
| 27 | `EnrolDevRoot` | `MkDevRootRequest`, `MkDevRootConfirm`, `MkDevRootLocal`, `MkLocalConsent`, `MkLocalRestore`. `Admin` does not imply it. |
| 28 | `Keyring` | Nothing that runs. The kernel's keyring client checks it in `gate_caller` (`src/security/keyring_capsule/capability.rs:26-34`), but no kernel code calls that client, and a capsule sends to the keyring with `IPC` alone. |
| 29 | `Entropy` | Nothing that runs. The kernel's entropy client checks it in `gate_read` (`src/security/entropy_capsule/capability.rs:23-32`) for statistics and health checks no kernel code asks for, and a capsule sends to the entropy service with `IPC` alone. |
| 30 | `AppInstall` | `MkAppInstall`, `MkAppLaunch`, `MkAppInstallStatus`, `MkAppUninstall`. |
| 31 | `AttestRead` | `MkAttestEntries`, `MkLogTail`, every field of other processes in `MkProcStat`; `MkAttestDoc` only without `Network`. |
| 32 | `ForeignExec` | The 15 `MkForeign` and `MkPeer` calls that host a Linux guest, and `MkLocalVerify`. |
| 33 | `LocalSign` | `MkLocalSign`. |
| 34 | `StreamImport` | `MkDataFeedBegin`, `MkDataFeed`, `MkDataRemove`. |
| 35 | `DeviceSecret` | `MkDeviceSecret`, `MkBootSlots`, `MkEnroll`. |
| 36 | `InputConsumer` | `MkInputEventDrain`, `MkInputEventWait`; input_router alone holds it. |

Some calls need only a valid token: `MkExit`, `MkPidAlive`, `MkYield`, the futex
and time reads, `MkBatteryStatus`, `MkProcStat`, `MkAttestStatus`,
`MkAttestPolicy`, `MkBootAttest` and `MkCapCheck`, listed together under
`is_valid` (`src/syscall/contract/cap_table/mk.rs:25-38`). A valid token is one
that has not expired and grants at least one capability, as `is_valid` reads
(`src/capabilities/token/types/query.rs:44-47`). `MkSetTls` needs the same,
since it changes nothing outside the caller
(`src/syscall/contract/cap_table/mk_ipc.rs:27-31`). `MkTtyQuery` admits every
caller whose token passed the resolver
(`src/syscall/contract/cap_table/mk_surface.rs:50`). `MkProcStat` shows a caller
without `AttestRead` or `ProcessControl` only the identity and state of other
processes, as `sees_all` decides
(`src/syscall/microkernel/procstat_redact.rs:29-34`).

Two refusals are written into the predicates themselves. `can_attest_doc`
refuses a caller that holds `Network`, because a TPM quote names the machine for
good (`src/syscall/caps/checks/system.rs:42-47`). `can_input_consumer` takes
`InputConsumer` alone, so neither a device driver, which holds `Irq`, nor a
holder of `Admin` reads the keystroke stream
(`src/capabilities/token/types/authority_broker.rs:64-66`).
