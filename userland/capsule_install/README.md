# capsule_install

`capsule_install` is the desktop installer. It writes a whole NONOS disk onto a disk the user
names: the running bootloader and kernel image on an EFI system partition, a package store
carrying this boot's setup answers and signed programs, the disk plan, and the data volume's
cleared key header, then reads every written sector back (`Cargo.toml`,
`src/install/mod.rs`). It is `no_std` and built on `nonos_app_skeleton`, `nonos_disk`,
`nonos_disk_map` and `nonos_blk_client`. The handbook page is
[docs/handbook/apps/installer.md](../../docs/handbook/apps/installer.md).

The capsule is turned on by `nonos-capsule-install` in `microkernel-desktop-offline`, so every
desktop image carries it, the hardened `microkernel-setup-wizard` image included. The kernel
spawns it on demand, or first and full screen when setup or the boot menu asks for an install.

## Role

Screens run in a fixed order (`src/install/state/screen.rs`): Welcome, Proofs, Disks,
Confirm, Writing, Verifying, Done, with Failed after Writing or Verifying. Nothing touches the
disk before Confirm (`src/install/job/prepare.rs`, `src/install/job/start.rs`).

- The image bytes come from the kernel via `mk_install_source` and are copied once into
  capsule memory (`src/install/source/load.rs`); the heap is sized from those sizes
  (`src/main.rs`).
- The store contents come from the vfs over IPC (`src/install/carry.rs`); the plan is refused
  until `vfs::store_settled()` is true.
- The write and read-back advance 2 MiB per tick (`Job::BUDGET`, `src/install/job/work.rs`).
- With no `wm` service running it takes the whole screen instead of a window
  (`src/install/full/active.rs`).
- The disk list comes from `nonos_blk_client::survey` (`src/install/survey.rs`): when the
  Disks screen opens, on R, when Enter on Failed comes back to it, and every two seconds
  while it has nothing to install to or a row for a missing driver (`src/install/rescan.rs`).
  Rows name the bus, the model when the part gives one and the size; a controller whose driver
  did not come up gets a row saying so. With no disk and an Intel RST or VMD controller on the
  bus, the screen says to set the firmware's storage mode to AHCI.

## Capabilities

From `Capsule.mk`:

```make
CAPSULE_REQUIRED_CAPS    := 0x84009A79
CAPSULE_OPTIONAL_CAPS    := 0x100
```

The comment above it reads
`CoreExec|IPC|Memory|Crypto|FileSystem|Admin|GraphicsDisplayQuery|GraphicsSurfaceCreate|`
`DeviceEnum|StoreWrite|AttestRead`, which matches the bits set; the optional bit is Debug.
FileSystem is for what it carries, which it reads from vfs, and vfs serves only a holder of FileSystem.

The header comment gives the reasons: the graphics pair for a window, DeviceEnum to list disks
and read the boot image, Crypto for the GUIDs it mints (`crypto_random` in `prepare.rs`),
StoreWrite for the disk itself (the disk drivers serve raw sectors to the kernel and to
StoreWrite holders alone), Admin for the reboot (`mk_admin_reboot` in
`src/install/event/after.rs`), and AttestRead for
the capsule census (`mk_attest_entries` in `src/install/source/census.rs`). Debug is used by
`mk_debug` in `src/install/full/start.rs`; only a `capsule-serial-debug` build grants it.

## Interface

- Service endpoint `service:4932:app.install`, reply endpoint
  `reply:4933:endpoint.app.install.reply`, handle `app.install`.
- Keyboard only (`src/install/manifest.rs` sets the key down bit alone).
- The erase starts only when the typed word equals the selected disk's `confirm_word()` and
  a plan exists (`src/install/event/confirm.rs`); the word is the last four serial characters
  or the bus name (`userland/nonos_blk_client/src/disks/describe.rs`).

## State and privacy

The Proofs screen shows what the kernel reports: `mk_attest_status`, `mk_attest_policy`,
`boot_attest` and the census (`src/install/source/attest.rs`). The TPM is probed by asking for
a key under the label `install.tpm-probe`; the key is overwritten with volatile writes and
not used (`src/install/source/tpm.rs`). All install state lives in capsule memory.

## Build and test

- Build: `make nonos-mk-install`, with `-sign` and `-verify` variants (`nonos-mk/capsule.mk`).
- The rescan timing is tested in `userland/setup_layout_proofs/src/installer_rescan_tests.rs`,
  the screens' text and layout in the other `installer_*` tests there.
- The capsule has no other tests of its own. The disk writer it drives has integration tests in
  `userland/nonos_disk/tests/` (for example `verify_reads_back_and_catches_corruption.rs`);
  no CI job runs that crate (neither `.github/workflows/verify.yml` nor `tools/nix/checks.nix`
  names it).

## Not done yet

- The image buffers are leaked on purpose (`Vec::leak` in `src/install/source/load.rs`) so
  the writer's borrows outlive every frame; they are never freed for the life of the capsule.
- A disk with no serial falls back to `nvme`, `sata` or `virtio` as its confirm word, with
  the driver instance after it past the first (`sata1`); two disks served by one instance
  would share it, which no driver does today.
- A disk with 4096-byte blocks is listed but refused at the plan: the writer lays the GPT out
  in 512-byte blocks, and firmware would not find it there.
