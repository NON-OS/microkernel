# desktop_proofs

Host-runnable proofs for the desktop's pointer path end to end. One press
on a window's title bar is driven through the real input router press
grab (`capsule_input_router/src/state/press.rs`), the window manager's
stack, focus and hit test (`capsule_wm/src/`), the compositor's scene and
raise (`compositor/src/state/`), and app_skeleton's chrome, dispatch,
drag and press grab (`app_skeleton/src/runner/`), each included through
`#[path]` under the module paths its files name each other by. `desk.rs`
is the small desktop that wires them together.

| Tests | What they hold |
|---|---|
| `one_press_tests` | one press on a covered window's title bar raises it, focuses it and starts its drag; after every step the window drawn on top is the one a press there would hit |
| `drag_tests` | the drag state machine, the press routing inside a window and the router's press grab, each driven directly |
| `frame_scale_tests` | a window's frame at the shell's display scale: title bar, buttons, border and title grow by the shell's rounding, each button answers where it is painted, the drag still tells the bar from the border, and a window grows by the frame so its content keeps its size |
| `delivery_tests` | app_skeleton's `parse_delivery` takes any frame without a panic and yields an event only from a whole NINP frame with a known kind |
| `tray_share_tests` | the desktop shell's tray holds each client to half and removes the items of clients that ended (`capsule_desktop_shell/src/state/tray/`) |

Run: `cargo test` in this directory. The desktop is described in
[Applications, window manager and desktop shell](../../docs/handbook/desktop/app-model.md)
and [Compositor](../../docs/handbook/desktop/compositor.md).
