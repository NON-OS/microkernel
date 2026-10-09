# Files

Where your files live on NONOS, how to work with them in Files, and exactly what is kept when the machine powers off.

## The short version

- Every file you see lives in the [file store](../overview/glossary.md#file-store), `vfs_pool`, which holds it in memory. At power off it is gone, unless it was written to the disk.
- NONOS forgets by default. The first mode setup offers is `Amnesic (default)`: RAM only, nothing written to any disk. This is an [amnesic boot](../overview/glossary.md#amnesic-boot).
- Even on a system installed to a disk, Files and Editor do not write your files to the disk in this release. The Terminal's `keep` command is the one way to keep a file you made. See [What is kept after power off](#what-is-kept-after-power-off).

## What Files shows

Files (`app.file_manager`) browses the file store and nothing else. Its sidebar has:

- Home, Recents and Tags. Recents lists the files you opened, from the file store's access journal.
- Favourites, the entries you pinned with `f`.
- Places: Downloads (`/downloads/`), Root (`/`), Documents (`/docs/`) and Capsules (`/capsules/`). A fresh boot does not make `/downloads/` and no app saves into it, so Downloads shows an empty folder. The Browser saves nothing, and Music keeps its downloads in `/home/nonos/music`.
- At the foot, the drive card, labelled `NØNOS Drive`. Its bar counts file slots in use, not bytes, because the store states no byte ceiling to divide by.

What a fresh boot holds:

| Path | What is there |
|---|---|
| `/home/nonos` | Your home folder. The desktop icons are its entries. It starts with `readme.txt`, `documents` and `workspace`. |
| `/home/nonos/music` | The Music library. Music creates it when it is missing. |
| `/docs` | `about.txt` and `demo.txt`. |
| `/images` | Four sample pictures. |
| `/tmp` | Scratch space. |
| `/capsules` | Signed programs from the disk's store. Ordinary writes are refused there. |
| `/Movies` | Sample films, on an image built with them. |

A USB stick written by another system does not show up in Files. The file store reads one store, the one on the NONOS disk the machine booted from, and has no reader for other file systems.

## Opening a file

`Enter`, or a click, opens a folder in place and a file in the app that reads it:

| Extension | Opens in |
|---|---|
| `txt`, `md`, `log`, `rs`, `toml`, `json`, `html` | Editor |
| `mp3`, `wav` | Music |
| `avi` | Video |
| `png`, `jpg`, `jpeg`, `bmp`, `gif` | Image Viewer, which no image in this release carries; see [Image Viewer and Clock](apps.md#image-viewer-and-clock) |

Any other file opens in the preview pane, as text or, for a binary file, as hex. So does a file whose app could not be started, and the status line says `that app could not be started; showing a preview`.

## Working with files

Files works with the mouse and with single keys. Press `?` for the key list; any key closes it.

| Key | What it does |
|---|---|
| arrows, or `j`, `k`, `h`, `l` | Move, open, go up. |
| `Enter`, `l` | Open a folder, or preview a file. |
| `Backspace`, `h` | Up one folder. |
| `Space`, `a` | Check or uncheck an entry; select everything in view. |
| `n`, `m` | New file; new folder. |
| `r` | Rename. |
| `d` | Delete the selection or the entry under the cursor. Type `y` and `Enter` to confirm. |
| `c`, `x`, `p` | Copy, cut, paste into the current folder. |
| `o` | Duplicate. |
| `u` | Switch read-only on or off. |
| `f` | Pin or unpin in Favourites. |
| `t` | Tag or untag. |
| `s` | Sort by name, size, date or type in turn. |
| `/` | Filter; type to search. |
| `Esc` | Close the window. |

- Search finds files by name and by content. Content matching skips files over 1 MiB and binary files, which are still matched by name.
- The header's undo button reverses a new file, new folder, rename, permission change, paste, move or duplicate, and says whether all, part or none of it was put back.
- A delete cannot be undone. There is no trash, so a delete also empties the undo list.
- A folder whose listing failed shows `Files are not available` with the reason, never an empty folder.

## Limits of the file store

The file store refuses a write with "no room" (`ENOSPC`) past these limits, instead of failing as a whole:

- 2048 names in all, and one program may create at most a quarter of them.
- 64 MiB for one file.
- 160 MiB for all files together.

## What is kept after power off

```mermaid
flowchart LR
    apps["Files, Editor, Music"] --> vfs_pool["file store in memory"]
    terminal["Terminal keep"] --> vfs_pool
    vfs_pool -->|"installed disk only"| store["capsule store on the NONOS disk"]
    models["Qwen models"] --> volume["data volume"]
```

The file store is memory. A file reaches the disk only when the program that created it asks the store to keep it, and the store agrees only when the policy store's `Keep data across reboots` field is on. That field is on when setup's Mode step chose `Install to this computer`, and on every boot of a disk NONOS was installed to, since the installer carries setup's answers there. Settings shows it without letting you change it.

On an amnesic boot, the default, nothing is kept:

- Every request to keep a file is refused, and the store logs `[VFS] refused persist: amnesic boot`.
- On a live stick, the [data volume](../overview/glossary.md#data-volume) that holds Qwen models is held in memory, and nothing of this machine's is written to the stick.
- Setup asks its questions again at the next boot. If you install NONOS from this boot, the installer carries your answers to the new disk.

On a machine where `Keep data across reboots` is on, as on a NONOS installed to a disk, these are kept:

- Setup's answers: your name, keyboard layout, time zone, wallpaper, the wallpapers kept, Qwen model, the apps turned off, the computer's name and the network route.
- Every value you change in [Settings](settings.md) that the policy store keeps.
- Saved Wi-Fi networks, sealed with the TPM machine key.
- The Terminal's theme, font size and side rail, Files' tags, favourites and view settings, and Snake's scores.
- The wallet's files under `/data`, a folder of the file store, not the data volume: its account key and recovery words, which the keyring seals, and three small files that are not secret. See [Wallet](wallet.md).
- Qwen models installed from the store, in the data volume.
- The data volume, on the disk, keyed by a key the TPM derives for this machine.
- Files you created in the Terminal and kept with `keep`.
- NONOS packages installed with the Terminal's `pkg install`, or from a `.nonos` file in `/pkgs`. See [Install a package file](marketplace.md#install-a-package-file).

Everything in this list but the Qwen models and the data volume that holds them is kept in the package [store](../overview/glossary.md#store) on the disk, which is not encrypted. Anyone who holds the disk can read a kept file there, unless the program that kept it sealed it first, as the Wi-Fi list and the wallet's key and recovery words are.

These are not kept, even on an installed disk:

- Files you create or change in Files or in Editor. Neither app asks for a file to be kept.
- Music downloads in `/home/nonos/music`.
- Terminal history, scrollback, variables and aliases.
- Linux packages installed from the store. Each is held in memory until restart.

To keep a file, make it in the Terminal and keep it there:

```sh
write /home/nonos/notes.txt remember the backup
keep /home/nonos/notes.txt
```

Not tested in this release.

`keep` prints `persisted` when the file reached the disk. Three rules hold it:

- Only the process that created a file may keep it, so a file made in Files, or in another Terminal window, cannot be kept from this one.
- Once a path is kept, `keep` can replace its copy on disk only with contents of exactly the same length. Any other change is refused, and removing a kept file frees no disk space.
- The disk store holds at most 512 entries and 96 MiB of loaded files, the signed programs it already carries included. A kept path is at most 96 printable ASCII characters.

## Where this comes from

The code behind each section, at the commit in the footer.

- The short version
  - The setup modes, the amnesic one first: `MODES` in `userland/capsule_setup_wizard/src/render/screens/mode.rs:18-19`.
- What Files shows
  - Files browses only the file store: `capsule_file_manager` in `userland/capsule_file_manager/README.md:5-7`.
  - The sidebar rows: `side_rows` in `userland/capsule_file_manager/src/fm/sidebar_rows.rs:31-50`, with the places in `PLACES` in `userland/capsule_file_manager/src/fm/paint_sidebar.rs:30-31`.
  - What a fresh boot holds, with no Downloads folder: `seed` in `userland/capsule_vfs/src/store/fdtable/seed.rs:27-47`.
  - Ordinary writes refused under Capsules: `is_read_only` in `userland/capsule_vfs/src/server/handlers/path/is_read_only.rs:17-18`.
  - The sample films: `NONOS_STORE_MEDIA_ENTRIES` in `mk/40-run.mk:38-45`.
  - One store, and no reader for other file systems: `vfs_pool` in `userland/capsule_vfs/README.md:5-20`.
- Opening a file
  - Which app opens which extension: `TABLE` in `userland/capsule_file_manager/src/fm/open_with_table.rs:45-61`.
  - The preview when no app starts: `open_selected` in `userland/capsule_file_manager/src/fm/event_open.rs:26-48`.
- Working with files
  - The key list, closed by any key: `KEYS` in `userland/capsule_file_manager/src/fm/help.rs:27`, and `on_key` in `userland/capsule_file_manager/src/fm/help.rs:62`.
  - Content search skips files over 1 MiB and binary files: `SEARCH_MAX_FILE_BYTES` in `userland/capsule_vfs/src/store/fdtable/search.rs:24`, applied in `userland/capsule_vfs/src/store/fdtable/search.rs:65-69`.
- Limits of the file store
  - 2048 names and 64 MiB for one file: `MAX_FILES` and `MAX_FILE_BYTES` in `userland/capsule_vfs/src/store/fdtable/types.rs:20-25`.
  - A quarter of the names for one program, 160 MiB in all, and a refusal the store survives: `DATA_BYTES_MAX` and `NAMES_PER_OWNER` in `userland/capsule_vfs/src/store/fdtable/budget.rs:26-42`.
- What is kept after power off
  - The keep gate: `require_persistent` in `userland/capsule_vfs/src/server/handlers/persist_gate.rs:35-39`.
  - An installed disk turns the field back on: `apply` in `userland/capsule_policy/src/restore/apply.rs:30-32`.
  - A live stick's volume stays in memory: `is_live` in `src/fs/blockfs_volume/plan_types.rs:57`, and `open_session_volume` in `src/fs/blockfs_volume/open_machine.rs:66-67`.
  - Setup's answers handed to the installer: `stage` in `userland/capsule_setup_wizard/src/keep/save.rs:52`.
  - Setup's answers kept: `record_of` in `userland/capsule_setup_wizard/src/keep/save.rs:68`.
  - The Settings values kept: `KEPT` in `userland/policy_proto/src/settings_record.rs:47`.
  - Packages kept only on a boot that keeps data: `may_persist` in `userland/capsule_vfs/src/server/handlers/store_install.rs:62`.
  - Kept files written to the store as they are: `append` in `userland/capsule_vfs/src/blk/store_write.rs:39-84`.
  - Linux packages held in memory until restart: `installed_line` in `userland/market_proto/src/reason.rs:106-113`.
  - Only the creating process may keep a file: `persistable` in `userland/capsule_vfs/src/store/fdtable/persist.rs:27`.
  - A kept path replaced only at the same length: `store_replace` in `userland/capsule_vfs/src/blk/store_write.rs:86-90`.
  - A changed length refused with `EEXIST`, and no space freed by a removal: `userland/capsule_vfs/README.md:114-116`.
  - 512 entries, 96 MiB and 96-byte paths: `NAME_LEN`, `MAX_ENTRIES` and `MAX_TOTAL_BYTES` in `userland/nonos_disk_map/src/container.rs:34-50`, in printable ASCII by `valid_name` in `userland/nonos_disk_map/src/container.rs:85-92`.

## See also

- [The desktop](desktop.md)
- [Terminal](terminal.md)
- [Settings](settings.md)
- [Sound and media](audio.md)
- [Install to disk](../install/install-to-disk.md)
- [Boot modes](../install/boot-modes.md)
- [Device secrets and keys](../security/device-secrets-and-keys.md)
