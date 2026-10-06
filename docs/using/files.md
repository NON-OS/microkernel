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
