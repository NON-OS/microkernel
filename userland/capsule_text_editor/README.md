# capsule_text_editor

## Role

`capsule_text_editor` is the desktop's Editor window, `app.text_editor`, a
1680 by 1000 app on `nonos_app_skeleton`. It edits text and code with syntax
highlighting, find and replace, go to line, comment toggling, bracket
auto-closing and undo, and it has a document model with pages, lists and
tables that exports to Markdown, DOCX or PDF (`src/doc/export/`). Its file
explorer is built from one `list_paths` call over the whole store. The
handbook page is
[System apps and services](../../docs/handbook/apps/system-apps.md).

```text
text editor (App trait)
    |
    | vfs client: list_paths, read_file, write_file, stat, mkdir, rename, unlink, rmdir
    v
vfs_pool (service:4104)          clipboard (cut, copy, paste)
```

## Microkernel contract

The window, input and frame loop come from `nonos_app_skeleton::run`. Files
are IPC calls to `vfs_pool`, with the editor's own pid from `mk_getpid` as
owner (`src/editor/resolve_owner_pid.rs`); `vfs_pool` refuses a claimed owner
that is not the sender. Cut, copy and paste go to the `clipboard` service.

## Authority

`CAPSULE_REQUIRED_CAPS = 0x1859`:

| Bit | Capability | Purpose |
|---|---|---|
| 0x0001 | CoreExec | run user code |
| 0x0008 | IPC | vfs, clipboard, window services |
| 0x0010 | Memory | heap, the 256 KiB document and window backing |
| 0x0040 | FileSystem | `vfs_pool` serves only a holder of it |
| 0x0800, 0x1000 | GraphicsDisplayQuery, GraphicsSurfaceCreate | its window |

Endpoints: `service:4726:app.text_editor`, reply `4727`, and the instance
windows `app.text_editor.1` (4830) and `app.text_editor.2` (4832). The kernel
mirror is `src/userspace/capsule_text_editor`.

## Open and save

- Open reads one byte past the 256 KiB capacity, and `refuse_open` turns
  away a longer file or one that is not UTF-8, so a file cut off at the
  limit never opens and is never saved back short (`src/editor/open_limit.rs`).
- Save writes with `vfs::write_file` and takes the new name only if the
  write landed (`src/editor/ctrl_save.rs`).
- The explorer says its last failure in words, whether or not the tree has
  rows.

## Formatting

Headings are text (a leading `#`). The ribbon's bold, italic, underline,
strike, colour, font and size, and paragraph alignment, are kept beside the
text in `src/editor/style_marks.rs`: a mark per byte and an alignment per
line, moved with the text by the one edit path (`splice`) and laid back over
the document model each time it is rebuilt, so they survive typing and reach
Export (DOCX and PDF carry all of them, Markdown what it can say). Typed text
takes the formatting of the character before it. Save writes the text alone,
and its status line says so when the document has formatting. Undo restores
text, and restored text takes its neighbour's formatting.

## Screens

The Home screen lists every file in the store and the files this session
opened. Settings has one section, Editing, whose two switches (invisible
characters, current line) are read by `src/editor/settings/live.rs`; they last
as long as the window. Help > Keyboard Shortcuts lists every Ctrl shortcut the
editor answers, and `editor_proofs` checks the list against the handlers.

## Privacy and persistence

The document lives in the editor's memory and in `vfs_pool`, which is RAM.
Save does not call `persist`, so a saved file stays in RAM unless something
else commits it to the store; it is gone at power off.

## Verification

- Build: `make nonos-mk-text-editor`; sign: `make nonos-mk-text-editor-sign`.
- `userland/editor_proofs` runs the real document engine on the host: the
  edit path and undo, the editing features, open limits, the save point, the
  explorer's failure note, formatting kept across edits and into the
  Markdown export (`style_marks_tests`) and the shortcut sheet against the
  key handlers (`shortcut_sheet_tests`).
- `layout_tests` runs pagination, line breaking and the caret hit-test.
