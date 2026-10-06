# Files

Where your files live on NONOS, how to work with them in Files, and exactly what is kept when the machine powers off.

## The short version

- Every file you see lives in the [file store](../overview/glossary.md#file-store), `vfs_pool`, which holds it in memory. At power off it is gone, unless it was written to the disk.
- NONOS forgets by default. The first mode setup offers is `Amnesic (default)`: RAM only, nothing written to any disk (`userland/capsule_setup_wizard/src/render/screens/mode.rs`). This is an [amnesic boot](../overview/glossary.md#amnesic-boot).
- Even on a system installed to a disk, Files and Editor do not write your files to the disk in this release. The Terminal's `keep` command is the one way to keep a file you made. See [What is kept after power off](#what-is-kept-after-power-off).

## What Files shows

Files (`app.file_manager`) browses the file store and nothing else (`userland/capsule_file_manager/README.md`). Its sidebar has:

- Home, Recents and Tags. Recents lists the files you opened, from the file store's access journal.
- Favourites, the entries you pinned with `f`.
- Places: Downloads (`/downloads/`), Root (`/`), Documents (`/docs/`) and Capsules (`/capsules/`) (`userland/capsule_file_manager/src/fm/paint_sidebar.rs`).
- At the foot, the drive card, labelled `NØNOS Drive`. Its bar counts file slots in use, not bytes, because the store states no byte ceiling to divide by.

What a fresh boot holds (`userland/capsule_vfs/src/store/fdtable/seed.rs`):

| Path | What is there |
|---|---|
| `/home/nonos` | Your home folder. The desktop icons are its entries. It starts with `readme.txt`, `documents` and `workspace`. |
| `/home/nonos/music` | The Music library. Music creates it when it is missing. |
| `/docs` | `about.txt` and `demo.txt`. |
| `/images` | Four sample pictures. |
| `/tmp` | Scratch space. |
| `/capsules` | Signed programs from the disk's store. Ordinary writes are refused there (`userland/capsule_vfs/src/server/handlers/path/is_read_only.rs`). |
| `/Movies` | Sample films, on an image built with them (`mk/40-run.mk`). |

A USB stick written by another system does not show up in Files. The file store reads one store, the one on the NONOS disk the machine booted from, and has no reader for other file systems (`userland/capsule_vfs/README.md`).

## Opening a file

`Enter`, or a click, opens a folder in place and a file in the app that reads it (`userland/capsule_file_manager/src/fm/open_with_table.rs`):

| Extension | Opens in |
|---|---|
| `txt`, `md`, `log`, `rs`, `toml`, `json`, `html` | Editor |
| `mp3`, `wav` | Music |
| `avi` | Video |
| `png`, `jpg`, `jpeg`, `bmp`, `gif` | Image Viewer |

Any other file opens in the preview pane, as text or, for a binary file, as hex.

## Working with files

Files works with the mouse and with single keys. Press `?` for the key list; any key closes it (`userland/capsule_file_manager/src/fm/help.rs`).

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

- Search finds files by name and by content. Content matching skips files over 1 MiB and binary files, which are still matched by name (`SEARCH_MAX_FILE_BYTES` in `userland/capsule_vfs/src/store/fdtable/search.rs`).
- The header's undo button reverses a new file, new folder, rename, permission change, paste, move or duplicate, and says whether all, part or none of it was put back.
- A delete cannot be undone. There is no trash, so a delete also empties the undo list.
- A folder whose listing failed shows `Files are not available` with the reason, never an empty folder.

## Limits of the file store

The file store refuses a write with "no room" (`ENOSPC`) past these limits, instead of failing as a whole (`userland/capsule_vfs/src/store/fdtable/budget.rs`):

- 2048 names in all, and one program may create at most a quarter of them.
- 64 MiB for one file.
- 160 MiB for all files together (`DATA_BYTES_MAX`).
