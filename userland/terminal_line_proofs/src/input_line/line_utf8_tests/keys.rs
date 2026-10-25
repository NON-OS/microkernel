// NONOS Operating System (AGPL-3.0-or-later)
//! Typing, the arrows, Backspace and Delete each take a whole character.

use super::{text, typed};

#[test]
fn typed_characters_go_in_whole() {
    let l = typed("café €5 日本");
    assert_eq!(text(&l), "café €5 日本");
    assert_eq!(l.cursor, "café €5 日本".len());
}

#[test]
fn backspace_takes_a_whole_character() {
    let mut l = typed("naïve€");
    assert!(l.backspace());
    assert_eq!(text(&l), "naïve");
    l.move_left();
    l.move_left();
    assert!(l.backspace(), "the two byte i");
    assert_eq!(text(&l), "nave");
}

#[test]
fn delete_takes_the_whole_character_under_the_cursor() {
    let mut l = typed("a日b");
    l.move_home();
    l.move_right();
    assert!(l.delete());
    assert_eq!(text(&l), "ab");
    assert_eq!(l.cursor, 1);
}

#[test]
fn the_arrows_step_one_character_of_any_length() {
    let mut l = typed("é€😀");
    let mut stops = vec![l.cursor];
    while l.move_left() {
        stops.push(l.cursor);
    }
    assert_eq!(stops, vec![9, 5, 2, 0]);
    while l.move_right() {}
    assert_eq!(l.cursor, 9);
}

#[test]
fn typing_in_the_middle_keeps_the_rest_intact() {
    let mut l = typed("über");
    l.move_home();
    l.move_right();
    assert!(l.insert_char('ß'));
    assert_eq!(text(&l), "üßber");
    assert_eq!(l.cursor, 4);
}
