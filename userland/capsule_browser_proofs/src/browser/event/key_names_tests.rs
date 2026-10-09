// NONOS Operating System (AGPL-3.0-or-later)
//! A key as a page's keydown reads it: key, code and keyCode, for the keys
//! pages answer most (Enter, Esc, the arrows, letters, digits).

use super::key_names::key_names;
use nonos_app_skeleton::{KEY_DOWN, KEY_ENTER, KEY_ESC, KEY_F1, KEY_F12, KEY_TAB};

fn names(code: u32) -> (String, String, i32) {
    let k = key_names(code);
    (k.key, k.code, k.key_code)
}

fn of(key: &str, code: &str, n: i32) -> (String, String, i32) {
    (key.into(), code.into(), n)
}

#[test]
fn named_keys() {
    assert_eq!(names(KEY_ENTER), of("Enter", "Enter", 13));
    assert_eq!(names(KEY_ESC), of("Escape", "Escape", 27));
    assert_eq!(names(KEY_TAB), of("Tab", "Tab", 9));
    assert_eq!(names(KEY_DOWN), of("ArrowDown", "ArrowDown", 40));
    assert_eq!(names(0x20), of(" ", "Space", 32));
    assert_eq!(names(KEY_F1), of("F1", "F1", 112));
    assert_eq!(names(KEY_F12), of("F12", "F12", 123));
}

#[test]
fn characters_keep_their_case_and_name_their_key() {
    assert_eq!(names('a' as u32), of("a", "KeyA", 65));
    assert_eq!(names('A' as u32), of("A", "KeyA", 65));
    assert_eq!(names('7' as u32), of("7", "Digit7", 55));
    assert_eq!(names('&' as u32), of("&", "Digit7", 55), "Shift+7 on a US layout");
    assert_eq!(names('/' as u32), of("/", "Slash", 191));
    assert_eq!(names('\u{e9}' as u32), of("\u{e9}", "", 0), "no US key types it");
}

#[test]
fn what_is_not_a_key_is_unidentified() {
    assert_eq!(names(0x01), of("Unidentified", "", 0));
    assert_eq!(names(0x1FFF), of("Unidentified", "", 0));
}
