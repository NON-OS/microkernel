# Design: Linux homes and installs kept across reboots

Status: the owner ruled on 27 October 2026. 0.9.3 ships K1, the handover
(K1b, below) and the installer's "Update NØNOS (keep my data)". Kept homes
and kept installs move to the next release. This page was first written
as a proposal, before code (S1, privacy/TCB). It builds on the homes
design already ruled on (`docs/design/files-home.md`, rulings
1-6) and
adds what those leave open: a volume key that survives an update, and the
installs themselves.

## The promise

On a machine installed to disk, with keeping chosen at setup, a Linux app
installed from the Marketplace is still installed after a reboot, and its
home (`/root`, `/home/<app>`) still holds what it wrote: a wallet, a
password database, a gpg keyring, a SimpleX profile. Nothing of it is on the
disk in plaintext, and the disk opens only on this machine. An amnesic boot
(the default) keeps nothing, as today.

## What is there

- **The opt-in.** Setup's step 4, Install, sets `Persistent`, which arms the
  kernel's `keeping` switch. Only the policy capsule can arm it, never off
  again within a boot, and an installed disk restores it on every boot
  (`src/security/zerostate/keep.rs:39-55`,
  `userland/nonos_disk/src/carry/setup.rs:17-49`). There is no finer choice.
- **The data volume.** Each 512-byte sector is sealed alone with
  ChaCha20-Poly1305 (`src/fs/cryptoblock/seal.rs:22-48`). It goes to the
  disk only when the disk is installed and keeping is on; otherwise it is a
  RAM volume under a fresh key (`src/fs/blockfs_volume/keep_switch.rs`,
  `src/security/zerostate/volume_rule.rs`). With keeping off, every disk
  write is refused (`src/fs/cryptoblock/device.rs:34-52`). One volume, one
  key per machine, flat names (`src/syscall/microkernel/data/name.rs`).
- **The key.** TPM2_HMAC under a primary whose policy is PolicyPCR over
  PCR 0, 4, 7 and 9, label `blockfs.data.v1`, never stored
  (`src/fs/blockfs_volume/open_into.rs:31-47`,
  `src/security/tpm/machine_key/pcrs.rs:21-24`). No TPM: the installed
  volume stays closed.
- **Userland reach.** Import (a file whose SHA-256 is known first, fed in
  parts), read (4 MiB a call, straight into a supervised guest), stat and
  remove. No general write, list or rename.
- **The Linux personality.** Homes are `/linux-home/<key>` in vfs RAM,
  16 MiB and 128 names each; packs `/linux-packs/<family>/<pkg>.pack`
  with a trailer LocalSign mints; records under `/nonos/linux/apps*`.
  `keep.rs:50-53` says, every install, that nothing is kept.

## The gap that comes first: PCR 9

PCR 9 is the kernel. A key bound to it is a new key with every release, so
today every NØNOS update leaves the data volume unreadable
(`docs/install/update.md:38-45`). For downloaded models that costs a
re-download. For a kept home it would cost a person their wallet at the
first update. Kept homes do not ship before this is fixed.

### K1: the volume key, wrapped under an approved chain

- A random 32-byte volume key, drawn at format, wrapped with
  ChaCha20-Poly1305 in the key header, keying kind 3 (`key_header.rs`):
  `NONOSDK1 | kind 3 | nonce | wrapped key | tag`, bytes 0..N the
  associated data, as kind 2 does.
- The wrapping key is an HMAC under the policy the device secret already
  uses: PolicyAuthorize(release key, `POLICY_REF`) over PCR 9, then
  PolicyPCR(0, 4, 7) as they are on this machine
  (`src/security/tpm/device_secret/mod.rs:20-26`), with its own label,
  `blockfs.data.v2`. A kernel the release signed computes it, so a signed
  update keeps the volume; a firmware, bootloader or Secure Boot change
  does not, as now (`docs/install/recovery.md:45`).
- A kind-1 volume is opened as today. The update that brings K1 is the
  last one that loses a volume: the old key needed the old PCR 9, so it
  cannot be re-wrapped after the fact. Nothing a person made is on the
  volume yet (models only), so this is the moment to do it.
- Format writes kind 3. A kind-3 header whose unwrap fails is refused as
  `Unopenable` and never formatted over, as a ring that will not open is
  now (`mount_or_format.rs:54-80`).
- Later, not in this unit: a recovery passphrase as a second wrapping of
  the same key (kind 2 already has the Argon2id half), so a person can
  survive a firmware update or move a disk.

| # | Rule | Mutant that must fail a test |
|---|---|---|
| K1.1 | a kind-3 header round-trips: wrap then unwrap gives the key | swap nonce and tag |
| K1.2 | any flipped byte of the header's associated data fails the unwrap | leave the kind byte out of the AD |
| K1.3 | a kind-3 volume that will not unwrap is never formatted over | format on any open error |
| K1.4 | a kind-1 volume still opens with the v1 derivation | read every header as kind 3 |
| K1.5 | format writes kind 3, never kind 1 | keep the v1 format |

### K1b: the handover, for an update that brings a new bootloader

K1 binds PCR 4, the bootloader, as it is, and most releases build the
bootloader anew, so K1 alone keeps a volume only across a kernel-only
update. Before an update the installer, on the person's word, asks
`MkDataHandover`: the volume key is sealed once more under the same policy
without PCR 4, label `blockfs.data.handover.v1`, into the key header's
sector at bytes 128..196. The next boot opens the volume under its own key
and clears the record, or, under a new bootloader, opens it by the record,
wraps the key under its own key and clears the record, in two writes each
read back. The record lasts one boot. The window it opens, and the code,
are on `docs/security/device-secrets-and-keys.md` ("The handover").

## Homes: H1 and H2, as ruled

Taken as designed in `03-homes-that-last.md`, with rulings 1-6:

- **H1.** vfs serves `/linux-home/*` and `/linux-private/*` only to a sender
  holding ForeignExec, which only capsule_linux holds. A whole-component
  prefix match. Today any FileSystem holder can read every app's home
  (`capsule_vfs/src/server/fs_gate/rule.rs:27-29`). This lands first, on
  its own, since it closes a hole in RAM homes too.
- **H2.** The RAM home stays the working copy. The volume keeps a sealed
  copy in two slots, `/h-<32 hex of the key>.a` and `.b`, as one versioned
  file (`NLXHOME1`, its decoder an untrusted parser with a fuzz target).
  It is loaded at the app's first run of the boot from the newer valid
  slot, and written to the older slot on fsync, fdatasync and sync, at most
  once a second, and when the app's family ends.
- **The kernel calls.** `MkKeptPut(name, bytes)` and `MkKeptDrop(name)`
  (H2's MkHomePut and MkHomeDrop, named for both uses below). They take a
  ForeignExec caller only, a name of the shapes below only, and write
  through `blockfs_volume::write_or_create` only when the volume is on the
  disk; otherwise ENOTSUP. Every data call that takes a name (read, stat,
  import, feed and remove) refuses those names to a caller without
  ForeignExec (`src/syscall/microkernel/data/kept_name.rs`), so no other
  capsule reads a kept home, plants one an app would load, or takes one
  away. A full volume is ENOSPC, not EIO (`src/fs/vfs/map_volume_err.rs:47`).
- **Bounds.** 16 MiB and 128 names a kept home for 0.9.3, the RAM home's
  ceiling; 256 MiB (ruling 3) when a home is written in parts. A SimpleX
  profile and a Monero wallet with a long cache may not fit in 16 MiB, and
  the app's text says so.

## Installs: new here

A kept install is its pack, its trailer and its two records. Packs are the
large part, records are small.

- **The pack** is fed into the volume through the import stream that
  exists (`src/fs/blockfs_volume/import_feed/`), with its SHA-256, as
  `/lp-<32 hex>`, the hex of BLAKE3(`"NONOS.linux.kept.pack.v1"` | family |
  package | pack digest). The install role already holds StreamImport
  (`src/userspace/capsule_linux/roles.rs:41-48`). An imported file is
  import-only: nothing can write over it, and the kernel checks its hash
  when the feed ends. Uninstall removes it (`SYS_DATA_REMOVE`).
- **The index.** One file in two slots, `/li.a` and `/li.b` (`NLXIDX1`,
  versioned, generation, BLAKE3 over all), written through `MkKeptPut`:
  each kept package's name, family, listing key and id, pack name and
  SHA-256, its manifest (files and links) and its program record. The
  newer valid slot wins, a torn one never does, the write goes to the
  older.
- **Trust at load.** The trailer LocalSign minted does not survive an
  update: LocalSign's key is the machine key, PCR 9 bound
  (`src/security/local_build/identity.rs:40-50`). So no trailer is kept. A
  kept pack is trusted because it came out of this machine's sealed volume
  and matches the SHA-256 the sealed index holds for it, which is the trust
  the install had when it was made; LocalSign mints a fresh trailer for the
  boot when the pack is landed again.
- **When.** Lazily: at boot capsule_linux reads the index only, so `linux
  <prog>` and the store know what is installed. A pack is landed into the
  vfs store at the first run of its program in the boot, as an install
  lands it now. Serving pack bytes straight from the volume (the models'
  path) is a later unit that saves the RAM.
- **Bounds.** A pack is at most the vfs file ceiling today (64 MiB; S3 is
  lifting it). The index holds at most 256 packages.

| # | Rule | Mutant that must fail a test |
|---|---|---|
| I.1 | with keeping off nothing is fed or put; the install says once it lasts until power off | drop the keeping check |
| I.2 | the index decoder refuses a bad magic, version, hash, a duplicate package, a name or path outside its shape, trailing bytes | skip the path check |
| I.3 | the newer valid index slot wins, a torn one never does, a write goes to the older | prefer slot a |
| I.4 | a kept pack whose SHA-256 is not the index's is not landed, and says so | compare the length only |
| I.5 | uninstall removes the pack, the index entry and both home slots, else REMOVE_PARTIAL | skip the pack remove |
| I.6 | a pack name is a pure function of family, package and digest; two packages never share one | name by package alone |

## Amnesic stays the default

Nothing changes on a boot where keeping is off: `MkKeptPut` answers
ENOTSUP, no feed reaches a disk (the volume is in RAM, and the device
refuses), and capsule_linux says once, per app, "this boot keeps nothing:
<app> and its files last until power off". The setup screen's Install line
says what keeping now covers: "Linux apps and their files are kept, sealed
to this computer". The Settings note that claims installed apps are kept
(`capsule_settings/src/settings/ui/field_note_user.rs:25`) becomes true
with this, and is reworded to say how.

## What it does not do

- It does not protect a home from the app itself, nor from someone who has
  the unlocked, running machine.
- No TPM: nothing is kept, and the app's text and the install line say so,
  until the passphrase unit.
- Rollback: a sector can be replaced by an older sealed copy of itself
  from the same disk. The index generation stops a whole-slot rollback
  only within one boot. A TPM counter like the saved networks' one
  (`docs/design/wifi-saved-generation.md`) is the later answer.
- One key per machine, not per app: the guard between apps is
  capsule_linux and H1, not the cipher.

## The boot plan

On an installed NVMe lane with a software TPM and keeping on:

1. Install age and monero. In age's home `age -p -o /root/a.age notes.txt`;
   in monero's, make a wallet.
2. Reboot. Both are still listed as installed and start without an
   install. `age -d` gives the notes back; the wallet opens.
3. Scan the disk image for the notes' plaintext and the wallet's address:
   absent.
4. Boot the same disk with a kernel the dev release key signed again
   (K1): the homes are still there.
5. Uninstall age: its pack, index entry and slots are gone; a reinstall
   finds an empty home.
6. The amnesic lane: steps 1 to 2 within the boot pass, and after Power
   Off nothing is left (`tools/nonos-zero-state-check` with a steps file
   that writes the secret into a home).
7. Isolation: busybox `linux ls -la /root` sees nothing of age's home; the
   file manager cannot open `/linux-home`.

## Units, in order

1. H1, the vfs gate (capsule_vfs), with H1.1-H1.5.
2. K1, the wrapped volume key (kernel), with K1.1-K1.5. It touches the
   trust path, so the owner reviews it before it merges.
3. `MkKeptPut`, `MkKeptDrop`, the read gate and ENOSPC (kernel).
4. Homes: `NLXHOME1`, load and flush (capsule_linux), with a fuzz target.
5. Installs: the pack feed, `NLXIDX1`, lazy landing, uninstall.
6. The store's "Erase home" and a "kept on this computer" line (S2, UI).
7. The boot plan, its logs under `docs/release-evidence/0.9.3/`.

## For S3

SimpleX and the dev tools depend on this. A program's home is `/root`
and its kept copy is everything under it, up to the bound above. A program
that needs more than 16 MiB before units 4 and 5 grow it should say so in
its listing text. Nothing a program does changes for keeping: it writes
files as on Linux, and fsync is the moment its files are on the disk.
