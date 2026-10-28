# Update

How to move an installed NONOS to a newer release, what carries over, and what the TPM changes on the way.

## Two ways to update

From 0.9.3 the installer can update an installed NØNOS in place and keep its encrypted [data volume](../overview/glossary-claim-epoch.md#data-volume). It rewrites the boot partition and the [store](../overview/glossary-linux-personality.md#store) and nothing else. Where it cannot keep the data, it installs over the old system as before: a whole new disk from the running image, the old data volume erased. There are still no A/B system slots, and the Marketplace installs programs into the store, never the system itself. The kernel's boot slots feed the anonymous device proof, and are not an update mechanism.

The installer offers the update when the disk is laid out as the installer lays one out and the new release can open its data volume:

- a data volume kept by a passphrase can be kept at once;
- a data volume kept by the TPM under the release-approved chain can be kept once the installed system has prepared it, as the next section says;
- a data volume kept by a TPM key bound to its own kernel cannot be kept. That is how releases before 0.9.3 keyed a volume with no passphrase, and the Confirm screen says `NØNOS made before 0.9.3: its data cannot be kept`.

How a prepared volume opens under the next release is in [Measured boot and the TPM](../security/measured-boot-and-tpm.md).

## Update and keep your data

1. Start your installed NØNOS, open the Install tile on the dock, and press P on the first screen, beside `Prepare this computer for an update`. The row is there only on a boot from an installed disk. When it says `Ready. Restart from the new NØNOS stick and choose Update NØNOS (keep my data). Until you do, your data volume opens without the bootloader check for one start.`, go on. A data volume kept by a passphrase answers `Your data is kept by a passphrase: nothing to prepare.`, and needs nothing more.
2. Get the newer image and write it to a stick ([Get an image](get-an-image.md), [Write a USB stick](usb-stick.md)).
3. Boot the stick and choose `Install NØNOS` in the boot menu ([Boot modes](boot-modes.md)), and answer setup ([First boot](first-boot.md)). An update carries the disk's own answers when it has them, and these when it does not.
4. On the Disks screen choose the disk that holds the old system. Its row says `NONOS installed`.
5. The Confirm screen, titled `Your data volume will be kept`, has two rows: `Update NØNOS (keep my data)`, lit, and `Erase everything and install`. Up and Down move between them, and each move clears what you typed. Leave the update row lit, read what the plan keeps and what it does not (its `kept` and `not kept` rows, the second saying `store apps, settings since setup, the wallet's vault, Wi-Fi`), type the disk's word, and press Enter.
6. Let the installer write and read back. The Done screen is titled `Updated`. Remove the stick and restart.

When the data volume is kept under the release-approved chain and was not prepared, the update row is muted, the title is `Your data is not ready to keep`, and the screen says `To keep your data, start your installed NØNOS, open Install NØNOS, choose Prepare this computer for an update, then start from this stick again.` Nothing can be started from that row. Down lights `Erase everything and install`, which works as on any other disk.

On a NØNOS disk the update cannot keep, there is one way, the install, and the Confirm screen is as it always was, except that `holds now` says why the data cannot be kept: a key bound to its kernel, no key header, a key this release does not know, a partition table, layout or disk plan other than the installer's own, or a boot partition the new release's files do not fit.

## Install over the old system

Where the update is not offered, or to start afresh, install the newer release over the old one. Boot the stick, choose `Install NØNOS` and answer setup. On the Disks screen choose the disk that holds the old system, and on the Confirm screen, when it shows the two rows, light `Erase everything and install` with Down. The screen warns `holds NØNOS; its store and data volume are lost`. Type the disk's word, let the installer write and read back, remove the stick and restart ([Install to disk](install-to-disk.md)).

An update from one release to another is not tested in this release.

## What an update writes

An update writes the boot partition's files into the boot partition the disk already has, as the same FAT32 volume an install builds, and the new store into the store partition the disk already has. It writes nothing to the protective MBR, to either partition table, to the disk plan, to the key header, to the data volume's header ring or to any other sector of the data volume. The table keeps its disk and partition GUIDs, and the receipt names them.

The writes go in this order: the boot partition's first sectors and the store's header are cleared; the boot partition's tables, directories and files go down; the store goes down, its header last; and last of all the boot partition's first sectors, which make it a volume again. Until that last write no firmware finds a volume to boot there. A write stopped at any point leaves the data volume, the table, the plan and the key header as they were, so the disk still offers the update: choose it again and update it once more to finish, or erase it and install. Escape stops an update until its last write. The read-back reads every sector the update wrote, and no other.

## What carries over

After an update:

- the data volume, with what NØNOS keeps in it (downloaded models and imported files), its key header, the disk plan and the partition table, untouched;
- setup's answers from the disk's own store, which is not encrypted, when they are there and check as setup's own reader checks them; otherwise the answers of the running boot, as an install carries them;
- from the running boot, as an install carries them: the wallpapers those answers keep, the signed programs the stick carries, and the files the Linux programs read.

Not carried by an update:

- anything else the old store held: the programs you installed there from the Marketplace, files kept there through the file service, the consent to run installed software, and settings changed after setup;
- the NØNOS wallet's account key and recovery words. They are files kept through the file service, and the keyring seals them under a key bound to the kernel, so a new kernel could not open them even if they were carried. Write the recovery words down before you update, and restore the wallet from them ([Wallet](../using/wallet.md));
- remembered Wi-Fi networks. They are sealed under the old kernel, so enter them again;
- nothing is wiped securely. Old store and boot partition bytes past what the update writes stay on the disk until they are written over, as the rest of an install's disk does ([Install to disk](install-to-disk.md#what-is-written)).

After an install over the old system, nothing from the old disk carries over. The installer carries what the running boot holds, and when you boot the stick that is the stick's store: the kernel keeps the disk the loader was read from, and never another NONOS disk in its place. It carries setup's answers, the wallpapers they keep, and the signed programs the stick carries. The old store, the programs you installed there from the Marketplace, anything you kept there, and the old data volume are erased with the disk. If an earlier boot of the stick kept its answers, setup does not run, and those answers go to the disk. Answers kept by 0.9.2 or earlier keep everything but the wallpaper: its choice named a picture from the old list of 63, so it reads as `Canyon neon`, the default, with all four wallpapers kept ([Settings](../using/settings.md#appearance)).

NONOS has no way to copy files off the old disk first: the file service keeps files in memory and in the NONOS store, and mounts no other file system, and the USB storage driver serves sectors, not files.

## The rollback floor

Every signed kernel carries a [rollback index](../overview/glossary-rollback-index.md#rollback-index): the [seal](../overview/glossary-rollback-index.md#seal) signs it in from `rollback_index` in `nonos.toml`, 1 by default, which that file says to raise only for a security release. Each verified boot raises the machine's [TPM](../overview/glossary-stark-proof.md#tpm) [rollback floor](../overview/glossary-peer-list.md#rollback-floor) to the index of the kernel it just booted, and a kernel whose index is below the floor is refused on every entry, under the title `This kernel is older than allowed`.

So on a machine with a TPM:

- Booting a newer release's stick once is enough to raise the floor, if that release raised its index. From then on the older system on the disk is refused, so update it or install the newer one.
- There is no way back to an older release on that machine once the floor has passed its index, short of clearing the TPM, which also loses every key the TPM derives.

Without a TPM the loader keeps no floor. Hardened and Air-Gapped then refuse to boot; every other entry notes `No TPM: rollback protection is off`, and an older signed kernel boots.

## Firmware changes and the data volume

A data volume keyed by a TPM key bound to its kernel, and the key that seals remembered Wi-Fi networks, are derived by the TPM under the boot [PCRs](../overview/glossary-linux-personality.md#pcr) 0, 4, 7 and 9: the firmware code, the boot manager the firmware measured, the Secure Boot policy, and the kernel the loader measured. Nothing of the key is stored, so a change to any of these gives another key:

- after a firmware update, or after turning Secure Boot on or off, such a data volume stays closed and remembered networks read `Sealed before a kernel, firmware or Secure Boot change`;
- a newer kernel opens no volume an older one keyed this way, by design. Which volumes an update keeps is in [Two ways to update](#two-ways-to-update).

A data volume formatted by 0.9.3 on a boot the release approved is keyed another way: its key is random, wrapped under a key the TPM gives a kernel the release approved on this machine's firmware, bootloader and Secure Boot state. A signed update keeps it, and the handover a prepare leaves carries it across a new bootloader for one boot. A firmware update or a Secure Boot change still closes it, as it closes the volumes above ([Device secrets and keys](../security/device-secrets-and-keys.md#the-handover)).

The kernel never formats over a volume it cannot open: it leaves it, and a program that asks for it gets EIO while the serial console says `[DATA] refused with EIO: Unopenable`. The kernel's own line for this goes to its structured log, which nothing installs in this release, so it does not print. Putting the firmware and the Secure Boot setting back the way they were gives the old key back. Clearing the TPM changes the seed these keys come from, and every such key with it, for good.

An install over the old system is not affected: the installer clears the key header and the volume's header ring, so the new system makes a new volume on its first boot. An update clears neither.

## Where this comes from

- Two ways to update
  - The installer writes a disk from the running image: `Install` in `userland/capsule_install/src/install/mod.rs:17-40`.
  - An update rewrites the boot partition and the store and nothing else: `keep_queue` in `userland/nonos_disk/src/session/queue_keep.rs:17-72`.
  - The Marketplace installs programs into the store and writes no disk itself: `capsule_installer` in `userland/capsule_installer/README.md:5-12`.
  - The boot slots feed the device proof: `MkBootSlots` in `userland/boot_slots_proofs/README.md:3-10`.
  - Which disks an update keeps, by the layout and then the key header's way: `classify` in `userland/nonos_disk/src/keep/classify.rs:40-62`, and `key_state` in `userland/nonos_disk/src/keep/key.rs:52-68`.
  - The ways a kernel before 0.9.3 keys a volume, a TPM machine key or a passphrase: `parse_key_header` in `src/fs/blockfs_volume/key_header.rs:17-26`.
  - The text for a key bound to its kernel: `NotKept::text` in `userland/nonos_disk/src/keep/why.rs:44-56`.
- Update and keep your data
  - The Install tile on the dock: `tile(LauncherIcon::Install, ...)` in `userland/capsule_desktop_shell/src/state/apps/table.rs:39`.
  - P beside `Prepare this computer for an update`, on a boot from an installed disk only: `on_start_key` in `userland/capsule_install/src/install/event/start.rs:35-38`, `prepare_row` in `userland/capsule_install/src/install/ui/screens/welcome_prepare.rs:31-46`, and `running_installed` in `userland/nonos_blk_client/src/disks/installed.rs:31-43`.
  - What the screen says for each answer: `answer` in `userland/capsule_install/src/install/handover/answer.rs:24-38`; the call it answers: `mk_data_handover` in `userland/capsule_install/src/install/handover/call.rs:17-27`.
  - The update row lit first, and Up and Down clearing the word: `prepare` in `userland/capsule_install/src/install/job/prepare.rs:26-37`, and `on_confirm_key` in `userland/capsule_install/src/install/event/confirm.rs:40-48`.
  - The two rows: `ROWS` in `userland/capsule_install/src/install/ui/screens/confirm_choice.rs:33-36`.
  - The plan's `kept` and `not kept` rows: `describe_keep` in `userland/nonos_disk/src/describe/keep_rows.rs:31-41`.
  - The titles `Your data volume will be kept`, `Your data is not ready to keep` and `Updated`: `kept_title` in `userland/capsule_install/src/install/state/screen.rs:48-55`, chosen by `title` in `userland/capsule_install/src/install/state/choice.rs:54-62`.
  - A row not ready drawn muted, its text, and nothing planned from it: `choices` in `userland/capsule_install/src/install/ui/screens/confirm_choice.rs:42-68`, `NOT_READY` in `userland/capsule_install/src/install/ui/keep_text.rs:23`, and `replan` in `userland/capsule_install/src/install/job/prepare.rs:39-47`.
  - `holds now` saying why: `disk_rows` in `userland/capsule_install/src/install/ui/screens/confirm_disk.rs:37-56`, with the reasons from `read_table` in `userland/nonos_disk/src/keep/table.rs:38-63`, `header_holds` in `userland/nonos_disk/src/keep/header.rs:33-45`, `placed` in `userland/nonos_disk/src/keep/places.rs:31-46`, `plan_inside` in `userland/nonos_disk/src/keep/volume.rs:34-52`, `key_state` in `userland/nonos_disk/src/keep/key.rs:52-68`, and `for_image` in `userland/nonos_disk/src/keep/fit.rs:29-53`.
- Install over the old system
  - The warning when the disk is erased: `warn` in `userland/capsule_install/src/install/ui/screens/confirm_warn.rs:26-40`.
  - Down lights the erase row: `on_confirm_key` in `userland/capsule_install/src/install/event/confirm.rs:40-48`.
- What an update writes
  - The boot partition's volume over the extent it has, and the GUIDs the table has: `Plan::keep` in `userland/nonos_disk/src/session/plan_keep.rs:30-47`, and the receipt's `disk_guid` and `partitions` in `userland/nonos_disk/src/session/finish.rs:33-34`.
  - The writes and their order, and nothing outside the boot partition and the store: `keep_queue` in `userland/nonos_disk/src/session/queue_keep.rs:46-72`, held by `an_update_writes_nothing_but_the_boot_partition_and_the_store` in `userland/nonos_disk/tests/an_update_writes_the_system_part_alone.rs:39-67`.
  - No boot sector until the last write, and a stopped update offered again: `an_update_stopped_anywhere_can_be_run_again` in `userland/nonos_disk/tests/an_update_stopped_anywhere_can_be_run_again.rs:40-74`.
  - Escape until the last write: `stoppable` in `userland/nonos_disk/src/session/start.rs:56-61`, and its hint in `hints`, `userland/capsule_install/src/install/ui/hints.rs:47-50`.
  - What to do after a stopped update: `RETRY_UPDATE` in `userland/capsule_install/src/install/ui/screens/failed_text.rs:23`.
- What carries over
  - The store not encrypted: `block_device::write_store` in `src/syscall/microkernel/store_write.rs:61`.
  - Setup's answers from the disk's own store, else the running boot's: `DiskAnswers::new` in `userland/nonos_disk/src/keep/source.rs:37-43` and `kept_file` in `userland/nonos_disk/src/keep/answers.rs:36-61`, used by `keep_plan` in `userland/capsule_install/src/install/job/plan_keep.rs:32-47`.
  - A USB stick that carries NONOS comes first: `ORDER` in `src/hardware/block_device/select.rs:39-48`.
  - Setup's answers, then wallpapers, then signed programs, and nothing else: `gather` in `userland/nonos_disk/src/carry/gather.rs:17-38`.
  - The new store written over the old one: `store_jobs` in `userland/nonos_disk/src/session/queue_state.rs:48-56`.
  - Remembered networks kept in the store, sealed under the boot PCRs: `PATH` in `userland/nonos_wifi_client/src/saved/file.rs:20-21` and `BOUND_PCRS` in `src/security/tpm/machine_key/pcrs.rs:21-24`; settings changed after setup kept in the store: `VALUES_PATH` in `userland/policy_proto/src/settings_record.rs:46-47`.
  - The wallet's two records kept through the file service: `VAULT_PATH` and `WORDS_PATH` in `userland/capsule_wallet_nonos/src/wallet/vault/path.rs:23-29`, sealed under the keyring's root, a machine key: `ROOT_LABEL` in `userland/nonos_vault/src/subkey.rs:32-36`, bound with the boot PCRs above.
  - Files in memory and in the store only: `capsule_vfs` in `userland/capsule_vfs/README.md:5-9`.
  - The USB storage driver serves sectors: `capsule_driver_usb_msc` in `userland/capsule_driver_usb_msc/README.md:5-8`.
- The rollback floor
  - The index, `rollback_index`, 1 by default and raised only for a security release: `nonos.toml:27-29` and `tools/nix/config.nix:129`, signed in by `sign_kernel` in `tools/nonos_seal/chain.py:65-70`.
  - Each verified boot raises the floor: `commit_rollback` in `nonos-bootloader/src/boot/crypto/rollback/commit.rs:26-47`.
  - A kernel below the floor is refused: `enforce_floor` in `nonos-bootloader/src/boot/crypto/rollback/floor.rs:28-39`.
  - The title `This kernel is older than allowed`: `PLATFORM` in `nonos-bootloader/src/display/boot/refusal/platform.rs:21-26`.
  - Without a TPM, Hardened and Air-Gapped refused and the rest warned: `Floor::Refuse` and `Floor::Unprotected` in `nonos-bootloader/src/boot/crypto/rollback/floor.rs:41-58`.
- Firmware changes and the data volume
  - PCRs 0, 4, 7 and 9: `BOUND_PCRS` in `src/security/tpm/machine_key/pcrs.rs:21-24`.
  - A newer kernel opens no older volume, by design: the boot tool's note above `serial` in `tools/nonos_qemu/__main__.py:140-142`.
  - An update clears neither the key header nor the ring: `keep_queue` in `userland/nonos_disk/src/session/queue_keep.rs:26-32`.
  - Never formatted over: `mount_or_format` in `src/fs/blockfs_volume/mount_or_format.rs:50-76`.
  - EIO and the `[DATA] refused with EIO:` line: `errno` in `src/syscall/microkernel/data/errno.rs:30-53`.
  - Clearing the TPM changes the seed and every key: `TPM2_HMAC` in `src/security/tpm/machine_key/mod.rs:19-31`.

## See also

- [Install to disk](install-to-disk.md)
- [Recovery](recovery.md)
- [Troubleshooting](troubleshooting.md)
- [Rollback protection](../security/rollback-protection.md)
- [Measured boot and the TPM](../security/measured-boot-and-tpm.md)
- [Release notes for 0.9.2](../release/0.9.2.md)
