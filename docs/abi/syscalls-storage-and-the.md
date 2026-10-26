# Syscalls: Storage and the data volume

Part of [Syscalls](syscalls.md).

### Storage and the data volume

The store calls read and write the [package
store](../overview/glossary-linux-personality.md#package-store), the part of the disk from sector
256 up to the [disk plan](../overview/glossary-claim-epoch.md#disk-plan). The data calls
work on files in the encrypted [data
volume](../overview/glossary-claim-epoch.md#data-volume) the plan names.

| Tag | Number | Name | Capability | Gate | Meaning |
|---|---|---|---|---|---|
| `MSWR` | `0x5257534D` | `MkStoreWrite` | StoreWrite | `can_store_write` `src/syscall/contract/cap_table/mk_device.rs:42` | Write up to 8 KiB of whole 512-byte sectors of the package store, at block 256 and above and below the disk plan. |
| `MSRR` | `0x5252534D` | `MkStoreRead` | StoreWrite | `can_store_write` `src/syscall/contract/cap_table/mk_device.rs:42` | Read up to 32 KiB of whole sectors from the same window of the package store. |
| `MKEP` | `0x50454B4D` | `MkKeeping` | valid token | `is_valid` `src/syscall/contract/cap_table/mk.rs:39` | 1 once this boot keeps what it writes, which only the policy service turns on; 0 before. |
| `MDIM` | `0x4D49444D` | `MkDataImport` | FileSystem and StoreWrite | `can_store_write` `src/syscall/contract/cap_table/mk_device.rs:43` | Import a file into the data volume, kept only if it matches its SHA-256. |
| `MDST` | `0x5453444D` | `MkDataStat` | FileSystem | `can_open_files` `src/syscall/contract/cap_table/mk_device.rs:45` | The size of a file on the data volume. |
| `MDRD` | `0x4452444D` | `MkDataRead` | FileSystem | `can_open_files` `src/syscall/contract/cap_table/mk_device.rs:45` | Read a range of a file on the data volume. |
| `MDPW` | `0x5750444D` | `MkDataPassphrase` | FileSystem and StoreWrite | `can_store_write` `src/syscall/contract/cap_table/mk_device.rs:44` | Key the data volume with a passphrase, or create it. |
| `MDFB` | `0x4246444D` | `MkDataFeedBegin` | StreamImport | `can_stream_import` `src/syscall/contract/cap_table/mk_device.rs:48` | Begin an import fed chunk by chunk, pinned to a digest. |
| `MDFD` | `0x4446444D` | `MkDataFeed` | StreamImport | `can_stream_import` `src/syscall/contract/cap_table/mk_device.rs:48` | Feed the next chunk of that import. |
| `MDRM` | `0x4D52444D` | `MkDataRemove` | StreamImport | `can_stream_import` `src/syscall/contract/cap_table/mk_device.rs:48` | Take an imported file off the data volume, with its record. |

### Attestation, signing roots and installs

| Tag | Number | Name | Capability | Gate | Meaning |
|---|---|---|---|---|---|
| `MAST` | `0x5453414D` | `MkAttestStatus` | valid token | `is_valid` `src/syscall/contract/cap_table/mk.rs:39` | The boot-chain attestation result the bootloader recorded. |
| `MADC` | `0x4344414D` | `MkAttestDoc` | AttestRead, without Network | `can_attest_doc` `src/syscall/contract/cap_table/mk.rs:50` | A TPM-signed attestation document over a challenge the caller chose. |
| `MAEN` | `0x4E45414D` | `MkAttestEntries` | AttestRead | `can_attest_read` `src/syscall/contract/cap_table/mk.rs:57` | The entries the signed registry root folds, to recompute it. |
| `MLOG` | `0x474F4C4D` | `MkLogTail` | AttestRead | `can_attest_read` `src/syscall/contract/cap_table/mk.rs:58` | The last of what the kernel wrote to its serial console, kept in memory. |
| `MAPY` | `0x5950414D` | `MkAttestPolicy` | valid token | `is_valid` `src/syscall/contract/cap_table/mk.rs:39` | The roots, depths and epochs the gates check against, as a versioned record. |
| `MBTA` | `0x4154424D` | `MkBootAttest` | valid token | `is_valid` `src/syscall/contract/cap_table/mk.rs:39` | The kernel's verdict on the bootloader that started it, as a versioned record. |
| `MDVS` | `0x5356444D` | `MkDeviceSecret` | DeviceSecret | `can_device_secret` `src/syscall/contract/cap_table/mk.rs:60` | This machine's TPM-derived device secret. |
| `MBSL` | `0x4C53424D` | `MkBootSlots` | DeviceSecret | `can_device_secret` `src/syscall/contract/cap_table/mk.rs:62` | The bootloader's and the kernel's slot of the device proof. |
| `MENR` | `0x524E454D` | `MkEnroll` | DeviceSecret | `can_device_secret` `src/syscall/contract/cap_table/mk.rs:64` | The TPM half of enrolment: endorsement key, certificate, attestation key, activation, signing. |
| `MISR` | `0x5253494D` | `MkInstallSource` | DeviceEnum | `can_install_source` `src/syscall/contract/cap_table/mk.rs:70` | A chunk of the image this machine booted, for the installer to write. Kind `0x101` is instead a four-byte record naming the disk this boot's block layer chose, backend and instance, which the installer never offers (`record` in `src/syscall/microkernel/install_source_records.rs`). |
| `MDRQ` | `0x5152444D` | `MkDevRootRequest` | EnrolDevRoot | `can_enrol_dev_root` `src/syscall/contract/cap_table/mk_proc.rs:34` | Ask to enrol a signing root; prints a confirmation code and enrols nothing on its own. |
| `MDRC` | `0x4352444D` | `MkDevRootConfirm` | EnrolDevRoot | `can_enrol_dev_root` `src/syscall/contract/cap_table/mk_proc.rs:34` | Complete a pending enrolment with the code the kernel displayed. |
| `MDRO` | `0x4F52444D` | `MkDevRootLocal` | EnrolDevRoot | `can_enrol_dev_root` `src/syscall/contract/cap_table/mk_proc.rs:34` | Ask to enrol this machine's own build root. |
| `MLCG` | `0x47434C4D` | `MkLocalConsent` | EnrolDevRoot | `can_enrol_dev_root` `src/syscall/contract/cap_table/mk_proc.rs:34` | Grant or withdraw consent to run what this machine installs. |
| `MLCR` | `0x52434C4D` | `MkLocalRestore` | EnrolDevRoot | `can_enrol_dev_root` `src/syscall/contract/cap_table/mk_proc.rs:34` | Restore that consent at setup, from the token a grant returned. |
| `MLSG` | `0x47534C4D` | `MkLocalSign` | LocalSign | `can_local_sign` `src/syscall/contract/cap_table/mk_trust.rs:28` | Mint a trailer for something this machine is installing. |
| `MLVF` | `0x46564C4D` | `MkLocalVerify` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_trust.rs:31` | Ask whether an image is proved under a root this machine trusts. |
| `MSNT` | `0x544E534D` | `MkSealNote` | ForeignExec | `can_foreign_exec` `src/syscall/contract/cap_table/mk_trust.rs:38` | A supervisor's note in its family's record: the app, or a diagnostic. |
| `MSLR` | `0x524C534D` | `MkSealRead` | DeviceSecret | `can_device_secret` `src/syscall/contract/cap_table/mk_trust.rs:39` | Take the newest final record sealed for an app, with its witness and measurements; it is dropped as it is read. |
| `MAIN` | `0x4E49414D` | `MkAppInstall` | AppInstall | `can_app_install` `src/syscall/contract/cap_table/mk_trust.rs:45` | Ask for a marketplace listing to be installed. |
| `MAPL` | `0x4C50414D` | `MkAppLaunch` | AppInstall | `can_app_install` `src/syscall/contract/cap_table/mk_trust.rs:45` | Start the program an installed package provides. |
| `MAIS` | `0x5349414D` | `MkAppInstallStatus` | AppInstall | `can_app_install` `src/syscall/contract/cap_table/mk_trust.rs:45` | Where an asked-for install stands. |
| `MAUN` | `0x4E55414D` | `MkAppUninstall` | AppInstall | `can_app_install` `src/syscall/contract/cap_table/mk_trust.rs:45` | Ask for an installed Linux package to be removed. |
| `MPJB` | `0x424A504D` | `MkProveJob` | IPC | `can_ipc` `src/syscall/contract/cap_table/mk_trust.rs:47` | Ask for the headless prover, `app.prove.job`: PROVE for the store, DELIVER for a vendor supervisor of a live family. |

### Capabilities and administration

See [Capabilities](capabilities.md) for the bits.

| Tag | Number | Name | Capability | Gate | Meaning |
|---|---|---|---|---|---|
| `MCGT` | `0x5447434D` | `MkCapGrant` | Admin | `can_admin` `src/syscall/contract/cap_table/mk_ipc.rs:53` | Grant capability bits to a process; the caller must hold Admin and every bit it grants. |
| `MCRV` | `0x5652434D` | `MkCapRevoke` | Admin | `can_admin` `src/syscall/contract/cap_table/mk_ipc.rs:53` | Revoke capability bits from a process. |
| `MCCK` | `0x4B43434D` | `MkCapCheck` | valid token | `is_valid` `src/syscall/contract/cap_table/mk.rs:39` | 1 when a process holds every bit of a mask, else 0. |
| `ARBT` | `0x54425241` | `AdminReboot` | Admin | `can_admin` `src/syscall/contract/cap_table/admin.rs:24` | Reboot. |
| `ASDN` | `0x4E445341` | `AdminShutdown` | Admin | `can_admin` `src/syscall/contract/cap_table/admin.rs:24` | Shut down. |
| `APPS` | `0x53505041` | `AdminPolicyPush` | Admin | `can_admin` `src/syscall/contract/cap_table/admin.rs:24` | Push one policy field. |
