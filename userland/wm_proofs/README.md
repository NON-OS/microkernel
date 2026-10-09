# wm_proofs

Host-runnable proofs for the window manager, on the real source included
through `#[path]`: geometry, the z stack and `raise`, focus and
`press_focus`, the protocol, the window table (`find`, `insert`,
`held_by`, `remove`), window kinds and the window-open decode
(`../capsule_wm/src/`).

| Tests | What they hold |
|---|---|
| `geometry_tests` | hit testing and clamp-to-display, which placement and click to raise depend on |
| `parse_tests` | the NWMP header decode, and that a refused frame is answered with the op and request id it named |
| `window_open_tests` | the open request decode |
| `window_share_tests` | one client holds at most `PER_OWNER`, half the table |
| `press_tests` | a press raises and focuses the window in one step; a tooltip takes no focus |
| `hand_off_tests` | focus moves to the top window still showing when the focused one closes, minimises or dies, never to a popup |
| `restack_tests` | a late compositor answer is a done raise; a lost one owes a restack, sent bottom first at a doubling interval |
| `notify_send_tests` | a full subscriber inbox is waited on and the subscriber kept; a gone one is dropped |
| `reopen_tests` | a window opened again takes the asked size, on screen, and is hit |
| `full_screen_tests` | the green button's rect reaches the bottom edge at every scale; restore, minimise, reopen and close end full screen; an old client's zero flag never hides the dock; the notification keeps the envelope |
| `geometry_tests` (resize) | a resize keeps the origin and takes the room left right of and below it |

`window.rs`, `window_table.rs`, `server.rs` and `server_handlers.rs` give
the included files the module tree they expect. `userland/desktop_proofs`
reuses `window.rs`.

Run: `cargo test` in this directory. The window manager is described in
[Applications, window manager and desktop shell](../../docs/handbook/desktop/app-model.md).
