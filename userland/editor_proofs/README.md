# editor_proofs

Host-runnable proofs for the text editor's document engine, with no
compositor or vfs present. The real editing source from
`../capsule_text_editor/src/doc/` and `src/editor/` is included through
`#[path]`, flat at the crate root so the files' `super::` imports resolve.

| Tests | What they hold |
|---|---|
| `edit_tests` | every mutation goes through `apply_edit`; undo restores the exact prior bytes and redo reapplies them |
| `feature_tests` | word motion, block indent, comment toggling, line duplication and deletion, smart Home and bracket auto-closing, each undone in one step |
| `open_tests` | an open reads one byte past capacity and refuses a cut-off or non-UTF-8 file |
| `save_tests`, `save_history_tests` | the document is clean exactly when its text is the text last read or written, across undo, dropped history and a reload |
| `tree_note_tests` | the explorer's note over a tree whose listing failed |

Run: `cargo test` in this directory. The editor is described in
[System apps and services](../../docs/handbook/apps/system-apps.md).
