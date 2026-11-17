// NONOS Operating System (AGPL-3.0-or-later)
//! The ribbon's square controls drawn as the shared icon button: a square
//! whose value holds is filled with the accent and its icon inked on it, one
//! the model does not back draws its icon dimmed, each of the ribbon's icons
//! draws apart from every other on, off and disabled, and a square the model
//! does not back takes no hover and no press.

use nonos_toolkit::icons::IconId;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::Pointer;

use super::square_tests::{centre, lock, paint, row};
use crate::editor::square::Square;

const ICONS: [IconId; 11] = [
    IconId::TextBold,
    IconId::TextItalic,
    IconId::TextUnderline,
    IconId::TextStrike,
    IconId::TextColor,
    IconId::AlignLeft,
    IconId::AlignCenter,
    IconId::AlignRight,
    IconId::ListBullet,
    IconId::ListNumber,
    IconId::Table,
];

#[test]
fn a_square_in_force_is_filled_with_the_accent_and_inked_on_it() {
    let _g = lock();
    let r = roles();
    let has = |on, live, c: u32| paint(&row(4, on, live), &Pointer::new()).contains(&c);
    assert!(has(true, true, r.accent.argb()) && !has(false, true, r.accent.argb()));
    assert!(has(true, true, r.on_accent.argb()) && has(false, true, r.accent_text.argb()));
    assert!(has(false, false, r.disabled.argb()) && !has(false, false, r.accent_text.argb()));
}

#[test]
fn every_square_draws_its_own_icon() {
    let _g = lock();
    for (on, live) in [(false, true), (true, true), (false, false)] {
        let as_icon = |icon: IconId| {
            let squares: Vec<Square> =
                row(4, on, live).into_iter().map(|s| Square { icon, ..s }).collect();
            paint(&squares, &Pointer::new())
        };
        let drawn: Vec<Vec<u32>> = ICONS.into_iter().map(as_icon).collect();
        for (i, a) in drawn.iter().enumerate() {
            assert!(drawn[i + 1..].iter().all(|b| a != b), "{:?} {on} {live}", ICONS[i]);
        }
    }
}

#[test]
fn a_square_the_model_does_not_back_takes_no_hover_or_press() {
    let _g = lock();
    for on in [false, true] {
        let squares = row(4, on, false);
        let unlit = paint(&squares, &Pointer::new());
        let (mut ptr, (x, y)) = (Pointer::new(), centre(squares[1].band));
        ptr.pressed(x, y);
        let st = squares[1].state(&ptr);
        assert!(st.disabled && !st.hovered && !st.pressed);
        assert_eq!(paint(&squares, &ptr), unlit, "a disabled square shows no press");
    }
}
