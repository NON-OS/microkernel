// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! The drivers deliver the final character, layout and shift applied. The
//! browser used to shift it again through a US table and drop everything
//! past ASCII: German Shift+7 gave '?' for '/', and umlauts vanished.

use crate::browser::omnibox::text_char;
use nonos_app_skeleton::{MOD_ALTGR, MOD_CAPS, MOD_CTRL, MOD_SHIFT};
use nonos_keymap::{resolve, Layout};

const KEYS: &[u8] = b"abcdefghijklmnopqrstuvwxyz0123456789-=[]\\;',./` ";

fn typed(c: u32) -> Option<char> {
    char::from_u32(c).filter(|ch| !ch.is_control() && !(0x1000..=0x1FFF).contains(&c))
}

#[test]
fn every_layout_types_exactly_what_the_driver_resolved() {
    for li in 0..Layout::COUNT {
        let l = Layout::from_index(li);
        let mut wrong = 0;
        for &k in KEYS {
            for (shift, caps, altgr) in (0..8).map(|m| (m & 1 != 0, m & 2 != 0, m & 4 != 0)) {
                let want = resolve(k as u32, shift, caps, altgr, l);
                let mut flags = 0;
                flags |= if shift { MOD_SHIFT } else { 0 };
                flags |= if caps { MOD_CAPS } else { 0 };
                flags |= if altgr { MOD_ALTGR } else { 0 };
                if text_char(want, flags) != typed(want) {
                    wrong += 1;
                }
            }
        }
        assert_eq!(wrong, 0, "{l:?}: {wrong} keys type the wrong character");
    }
}

#[test]
fn shortcuts_and_non_ascii() {
    assert_eq!(text_char(b'v' as u32, MOD_CTRL), None);
    assert_eq!(text_char(0xE4, 0), Some('\u{e4}'));
    assert_eq!(text_char(b'/' as u32, MOD_SHIFT), Some('/'));
    assert_eq!(text_char(0x1203, 0), None);
    assert_eq!(text_char(0x7F, 0), None);
    let de_shift_7 = resolve(b'7' as u32, true, false, false, Layout::De);
    assert_eq!(text_char(de_shift_7, MOD_SHIFT), Some('/'));
}
