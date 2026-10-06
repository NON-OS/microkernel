// NONOS Operating System (AGPL-3.0-or-later)
//! Form controls are atomic inlines with an intrinsic size: a text field
//! is about twenty characters wide and one line tall, a select fits its
//! widest option, and a button fits its label.

use crate::browser::fonts::ch_px;
use crate::probe::Page;

const VP: (u32, u32) = (1336, 800);
/* The engine's control font, 13.333px as in Chromium's user agent sheet. */
const CONTROL_PX: f32 = 13.333;
/* The control box the user agent gives once form controls are inline. */
const UA: &str = "<style>input,select,button{display:inline-block;margin:0;\
                  padding:1px 2px;border:2px solid #767676}</style>";

/* A search form reads on one line: label, field, button. The field is
 * 20ch plus its padding and border wide and a line tall. */
#[test]
fn a_text_input_is_an_inline_field_twenty_characters_wide() {
    let p = Page::at(&[UA, "<body style=margin:0><form><label>Search</label> <input id=q type=text> <button id=go>Go</button></form>"].concat(), VP);
    let (q, go, label) = (p.rect("q"), p.rect("go"), p.word("Search"));
    let want = (20.0 * ch_px(CONTROL_PX) + 0.5) as i32 + 8;
    assert_eq!(q[2], want, "20ch + 4 padding + 4 border");
    assert!(q[0] > label.x + label.w && go[0] > q[0] + q[2], "one line, in order");
    assert!(q[1] < label.y + label.h && go[1] < q[1] + q[3], "same line: {q:?} {go:?}");
    assert!(p.word("Go").x > go[0], "the button fits its label");
}

/* size= sets the width in characters. */
#[test]
fn the_size_attribute_sets_the_field_width() {
    let p = Page::at(&[UA, "<body style=margin:0><input id=q size=5>"].concat(), VP);
    assert_eq!(p.rect("q")[2], (5.0 * ch_px(CONTROL_PX) + 0.5) as i32 + 8);
}

/* A select is as wide as its longest option plus its arrow. */
#[test]
fn a_select_fits_its_widest_option() {
    let html = "<body style=margin:0><select id=s><option>a</option><option>a much longer one</option></select>";
    let p = Page::at(&[UA, html].concat(), VP);
    let long =
        crate::browser::fonts::measure_text(0, false, false, "a much longer one", CONTROL_PX, 0.0);
    assert_eq!(p.rect("s")[2], long + 20 + 8, "option text, arrow, padding and border");
}

/* Even as the old block default, a field is not the whole line wide. */
#[test]
fn a_block_field_keeps_its_intrinsic_width() {
    let p = Page::at("<body style=margin:0><input id=q style='display:block'>", VP);
    assert!(p.rect("q")[2] < 400, "field {:?} spans the page", p.rect("q"));
}
