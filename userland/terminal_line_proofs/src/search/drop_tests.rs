// NONOS Operating System (AGPL-3.0-or-later)
//! F12's drop-down: F12 drops and raises it again; Esc raises it only on an
//! empty line with nothing else open; no other key, and no modified F12, is
//! the drop-down's; the rect is the full width and 45% of the work area.

use super::drop::{key, rect, DROP};
use nonos_app_skeleton::{WorkRect, KEY_ENTER, KEY_ESC, KEY_F12, MOD_CTRL};

#[test]
fn f12_drops_and_raises() {
    assert_eq!(key(false, KEY_F12, 0, false), Some(true), "busy or not, F12 drops");
    assert_eq!(key(true, KEY_F12, 0, false), Some(false));
    assert_eq!(key(false, KEY_F12, MOD_CTRL, true), None, "Ctrl+F12 is not it");
}

#[test]
fn esc_raises_only_an_idle_dropped_window() {
    assert_eq!(key(true, KEY_ESC, 0, true), Some(false));
    assert_eq!(key(true, KEY_ESC, 0, false), None, "Esc belongs to the search or the line");
    assert_eq!(key(false, KEY_ESC, 0, true), None, "not dropped: Esc is the line's");
    assert_eq!(key(true, KEY_ENTER, 0, true), None);
}

#[test]
fn the_drop_is_full_width_under_the_menubar() {
    assert_eq!(DROP, WorkRect { x: 0, y: 0, w: 1000, h: 450 });
    assert_eq!(rect(true), Some(DROP));
    assert_eq!(rect(false), None, "not dropped: the window is left where it is");
}
