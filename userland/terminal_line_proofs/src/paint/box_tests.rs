// NONOS Operating System (AGPL-3.0-or-later)
//! The box characters the terminal strokes instead of drawing from the font.

use crate::box_arms::{arms, DOUBLE, NONE};

#[test]
fn every_line_character_has_arms_of_one_weight() {
    for ch in '\u{2500}'..='\u{257F}' {
        let Some(a) = arms(ch) else { continue };
        assert!(a.iter().any(|&w| w != NONE), "{ch} has no arm");
        let w: Vec<u8> = a.iter().copied().filter(|&w| w != NONE).collect();
        assert!(w.iter().all(|&x| x == w[0]), "{ch} mixes weights");
    }
}

#[test]
fn the_characters_csview_draws_are_all_stroked() {
    for ch in "─│┌┐└┘├┤┬┴┼═║╔╗╚╝╭╮╯╰".chars() {
        assert!(arms(ch).is_some(), "{ch} falls back to the font");
    }
    assert_eq!(arms('╬'), Some([DOUBLE; 4]));
}

#[test]
fn ordinary_text_is_left_to_the_font() {
    for ch in "aZ|-+=#".chars() {
        assert_eq!(arms(ch), None);
    }
}
