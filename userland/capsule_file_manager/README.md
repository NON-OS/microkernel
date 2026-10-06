# capsule_file_manager

## Role

`capsule_file_manager` is the desktop's Files window, `app.file_manager`, an
860 by 560 app on `nonos_app_skeleton`. It browses the `vfs_pool` file store
under its own pid through `nonos_app_skeleton::clients::vfs`: grid and list
views, a Home screen with recents and categories, search by name or content,
tags, favourites, a preview pane, copy, move, rename, delete, duplicate and
permissions. The handbook page is
[System apps and services](../../docs/handbook/apps/system-apps.md).

```text
file manager (App trait)
    |
    | vfs client: list, stat, read, copy, rename, unlink, chmod,
    |             search, journal, persist, usage
    v
vfs_pool (service:4104)
    |
    `-- desktop_shell OP_OPEN_WITH for images
```

## Microkernel contract

The window, input and frame loop come from `nonos_app_skeleton::run`. File
operations are IPC calls to `vfs_pool`; the owner pid in each is the
capsule's own, from `mk_getpid`. Opening an image (PNG, JPEG, BMP, GIF) asks
the desktop shell to `OP_OPEN_WITH` the image viewer (`src/fm/open_with.rs`).

## Authority

`CAPSULE_REQUIRED_CAPS = 0x1859`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x0001 | CoreExec | run user code |
| 0x0008 | IPC | vfs, desktop shell, window services |
| 0x0010 | Memory | heap and window backing |
| 0x0040 | FileSystem | `vfs_pool` serves only a holder of it |
| 0x0800, 0x1000 | GraphicsDisplayQuery, GraphicsSurfaceCreate | its window |

Endpoints: `service:4724:app.file_manager`, reply `4725`, and the instance
windows `app.file_manager.1` (4858) and `app.file_manager.2` (4860). The
kernel mirror is `src/userspace/capsule_file_manager`.

## Privacy and persistence

Files live in `vfs_pool`, in RAM. Tags, favourites and the manager's own
preferences are written to the store and then committed to the disk with
`persist`, which `vfs_pool` allows only when the user chose persistence at
setup; `persist_meta` runs on session boundaries rather than per keystroke,
since each changed record costs a store extent that a removal does not give
back (`src/fm/persist_meta.rs`). Opening a file records it in the vfs access
journal with `journal_touch`, which is what Recents reads.

## Failure model

- A folder whose listing failed shows "Files are not available" with the
  reason from `empty_listing`, never an empty folder; entries already on
  screen stay only if they are that folder's own (`src/fm/listing_state.rs`).
- New file, new folder, rename, permissions, paste, move and duplicate push
  their inverse onto an undo stack, an action on several entries as one
  entry, and `undo` replays it and says whether the store took all, part or
  none of it (`src/fm/event_undo.rs`). A delete has no inverse, the store
  keeps no trash, so it empties the stack and its status says so.
- Back goes up one folder; Forward goes back down into the folders Back
  left, and is dim once the person has gone anywhere else
  (`src/fm/nav_trail.rs`).
- Tag (`t`, the info pane or the selection's band) tags every selected
  entry, or untags them when all carry it, and says when a name is refused
  (`src/fm/tags_toggle.rs`).
- A Home figure from a folder walk the store cut short reads "over", since
  it counts only part of the folder (`src/fm/home_stor_text.rs`).

## Verification

- Build: `make nonos-mk-file-manager`; sign: `make nonos-mk-file-manager-sign`.
- `userland/fs_proofs` includes the manager's logic, formatting and tags
  (`fm_tests`, `listing_tests`, `tags_tests`, `favorites_tests`,
  `recents_tests`, `open_with_tests`, `prefs_tests`, and `fm_trail_tests`
  for Forward and the selection's tag) and runs them on the host.
