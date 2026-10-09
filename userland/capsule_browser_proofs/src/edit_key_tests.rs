// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Key bindings of the address bar, as mainstream browsers bind them.

use crate::browser::omnibox::{edit_key, EditKey};
use nonos_app_skeleton::{
    KEY_BACKSPACE, KEY_DELETE, KEY_ENTER, KEY_ESC, KEY_F6, KEY_INSERT, KEY_LEFT, KEY_RIGHT,
    MOD_ALT, MOD_ALTGR, MOD_CTRL, MOD_SHIFT,
};

#[test]
fn shortcuts_map_to_their_actions() {
    assert_eq!(edit_key(b'a' as u32, MOD_CTRL), EditKey::SelectAll);
    assert_eq!(edit_key(b'A' as u32, MOD_CTRL | MOD_SHIFT), EditKey::SelectAll);
    assert_eq!(edit_key(b'l' as u32, MOD_CTRL), EditKey::FocusOmnibox);
    assert_eq!(edit_key(KEY_F6, 0), EditKey::FocusOmnibox);
    assert_eq!(edit_key(b'd' as u32, MOD_ALT), EditKey::FocusOmnibox);
    assert_eq!(edit_key(b'c' as u32, MOD_CTRL), EditKey::Copy);
    assert_eq!(edit_key(b'x' as u32, MOD_CTRL), EditKey::Cut);
    assert_eq!(edit_key(b'v' as u32, MOD_CTRL), EditKey::Paste);
    assert_eq!(edit_key(KEY_INSERT, MOD_SHIFT), EditKey::Paste);
    assert_eq!(edit_key(KEY_DELETE, MOD_SHIFT), EditKey::Cut);
}

#[test]
fn editing_keys_map_with_their_modifiers() {
    assert_eq!(edit_key(KEY_BACKSPACE, MOD_CTRL), EditKey::WordBackspace);
    assert_eq!(edit_key(KEY_BACKSPACE, 0), EditKey::Backspace);
    assert_eq!(edit_key(KEY_DELETE, MOD_CTRL), EditKey::WordDelete);
    assert_eq!(edit_key(KEY_ESC, 0), EditKey::Cancel);
    assert_eq!(edit_key(KEY_ENTER, 0), EditKey::Commit);
    let left = EditKey::Left { word: true, extend: true };
    assert_eq!(edit_key(KEY_LEFT, MOD_CTRL | MOD_SHIFT), left);
    assert_eq!(edit_key(KEY_RIGHT, 0), EditKey::Right { word: false, extend: false });
}

#[test]
fn ctrl_letters_never_type_and_altgr_does() {
    assert_eq!(edit_key(b'q' as u32, MOD_CTRL), EditKey::Ignore);
    assert_eq!(edit_key(b'/' as u32, MOD_SHIFT), EditKey::Insert('/'));
    assert_eq!(edit_key(0x40, MOD_ALTGR), EditKey::Insert('@'));
    assert_eq!(edit_key(0x20AC, MOD_ALTGR), EditKey::Insert('\u{20ac}'));
}
