// NONOS Operating System (AGPL-3.0-or-later)
//! The theme picker: Up and Down show each theme, wrapping; Enter keeps the
//! one on show; Esc goes back to the one it opened on; other keys do nothing.

use super::profiles::{by_name, COUNT};
use super::theme_pick::{Key, Pick, Then, NAMES};

#[test]
fn every_name_is_its_themes_name() {
    assert_eq!(NAMES.len(), usize::from(COUNT));
    for (i, n) in NAMES.iter().enumerate() {
        assert_eq!(by_name(n), Some(i as u16), "{}", String::from_utf8_lossy(n));
    }
}

#[test]
fn up_and_down_show_each_theme_and_wrap() {
    let mut p = Pick::open(1, COUNT);
    assert_eq!(p.key(Key::Down, COUNT), Then::Show(2));
    assert_eq!(p.key(Key::Down, COUNT), Then::Show(3));
    assert_eq!(p.key(Key::Down, COUNT), Then::Show(0), "past the last, the first");
    assert_eq!(p.key(Key::Up, COUNT), Then::Show(3), "before the first, the last");
    assert_eq!(p.key(Key::Other, COUNT), Then::Stay);
    assert_eq!(p.at, 3);
}

#[test]
fn enter_keeps_and_esc_goes_back() {
    let mut p = Pick::open(1, COUNT);
    p.key(Key::Down, COUNT);
    assert_eq!(p.key(Key::Enter, COUNT), Then::Close(2));
    let mut q = Pick::open(1, COUNT);
    q.key(Key::Down, COUNT);
    q.key(Key::Down, COUNT);
    assert_eq!(q.key(Key::Esc, COUNT), Then::Close(1), "the theme it opened on");
}

#[test]
fn a_theme_number_out_of_range_opens_on_the_first() {
    assert_eq!(Pick::open(9, COUNT), Pick { from: 0, at: 0 });
}
