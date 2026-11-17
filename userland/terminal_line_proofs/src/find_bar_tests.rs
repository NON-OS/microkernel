// NONOS Operating System (AGPL-3.0-or-later)
//! The search bar: the field holds the query with the caret at its end, the
//! hint says what Enter does, and each part lights only its own box, at 1x
//! and 2x, with the rail open and shut.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::widgets::rect::contains;
use nonos_toolkit::widgets::{Field, State};

use crate::find_boxes::{changed, each, find, paint, H, W};
use crate::find_view::{hint, view, PLACEHOLDER};
use crate::strip_boxes::serial;

#[test]
fn the_hint_says_what_enter_does_with_the_query() {
    assert_eq!(hint(&find("", false, false)), "type to search history");
    assert_eq!(hint(&find("ls", true, false)), "Enter older, Shift+Enter newer, Esc close");
    assert_eq!(hint(&find("ls", false, true)), "no match");
}

#[test]
fn typing_lights_only_the_field_and_a_match_only_the_hint() {
    let _g = serial();
    each(|rows| {
        let v = view(rows.0, rows.1, false);
        let typed = changed(
            &paint(&find("ab", false, false), rows),
            &paint(&find("abc", false, false), rows),
        );
        assert!(!typed.is_empty() && typed.iter().all(|&(x, y)| contains(v.field, x, y)));
        let hit = changed(
            &paint(&find("ab", false, false), rows),
            &paint(&find("ab", true, false), rows),
        );
        assert!(!hit.is_empty() && hit.iter().all(|&(x, y)| contains(v.hint, x, y)));
    });
}

#[test]
fn case_lights_its_mark_and_nothing_paints_outside_the_bar() {
    let _g = serial();
    each(|rows| {
        let v = view(rows.0, rows.1, true);
        let (mark, plain) = (v.case.unwrap(), view(rows.0, rows.1, false));
        assert!(!contains(v.field, mark.x as i32, mark.y as i32) && mark.x >= v.hint.x + v.hint.w);
        let on = paint(&find("ab", true, true), rows);
        let lit = changed(&paint(&find("ab", true, false), rows), &on);
        assert!(lit.iter().any(|&(x, y)| contains(mark, x, y)));
        assert!(lit.iter().all(|&(x, y)| contains(plain.hint, x, y)), "{lit:?}");
        let parts = [v.field, v.hint, mark];
        let blank = vec![0u32; (W * H) as usize];
        let all = changed(&blank, &on);
        assert!(all.iter().all(|&(x, y)| parts.iter().any(|&r| contains(r, x, y))));
    });
}

#[test]
fn the_field_holds_the_query_with_the_caret_at_its_end() {
    let _g = serial();
    each(|rows| {
        let v = view(rows.0, rows.1, false);
        let bar = paint(&find("grep", false, false), rows);
        for (caret, same) in [(Some(4), true), (None, false), (Some(1), false)] {
            let mut px = vec![0u32; (W * H) as usize];
            let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
            let field = Field::new("grep").placeholder(PLACEHOLDER).caret(caret);
            field.state(State { focused: true, ..State::default() }).draw(&mut fb, v.field);
            let lit = changed(&bar, &px).into_iter().filter(|&(x, y)| contains(v.field, x, y));
            assert_eq!(lit.count() == 0, same, "{caret:?}");
        }
    });
}
