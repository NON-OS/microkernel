// NONOS Operating System (AGPL-3.0-or-later)
//! Right-to-left text: a paragraph's direction sets its line order and its
//! start edge, and runs of the other direction keep their own order.

use crate::probe::Page;

const VP: (u32, u32) = (1336, 800);
/* The user agent maps the dir attribute to direction this way. */
const DIR: &str = "<style>[dir=rtl]{direction:rtl}</style><body style=margin:0>";

fn at(html: &str) -> Page {
    Page::at(&[DIR, html].concat(), VP)
}

/* Hebrew words in an rtl paragraph run right to left and the line ends at
 * the container's right edge; a number stays readable left to right. */
#[test]
fn an_rtl_line_runs_right_to_left_from_the_right_edge() {
    let p = at("<p dir=rtl style='width:600px;margin:0'>\u{5e9}\u{5dc}\u{5d5}\u{5dd} \u{5e2}\u{5d5}\u{5dc}\u{5dd} 123</p>");
    /* Drawn in visual order, so each Hebrew word's letters are reversed. */
    let (w1, w2, n) = (
        p.word("\u{5dd}\u{5d5}\u{5dc}\u{5e9}"),
        p.word("\u{5dd}\u{5dc}\u{5d5}\u{5e2}"),
        p.word("123"),
    );
    assert!(w1.x > w2.x && w2.x > n.x, "logical order runs leftward: {} {} {}", w1.x, w2.x, n.x);
    assert_eq!(w1.x + w1.w, 600, "start edge is the right edge");
}

/* A left-to-right run inside an rtl paragraph keeps its own order. */
#[test]
fn an_ltr_run_in_rtl_text_keeps_its_order() {
    let p = at(
        "<p dir=rtl style='width:600px;margin:0'>abc <b>123</b> \u{5e9}\u{5dc}\u{5d5}\u{5dd}</p>",
    );
    let (abc, n, sh) = (p.word("abc"), p.word("123"), p.word("\u{5dd}\u{5d5}\u{5dc}\u{5e9}"));
    assert!(abc.x < n.x, "abc then 123, left to right");
    assert!(sh.x < abc.x, "the Hebrew word, logically last, is leftmost");
    assert_eq!(n.x + n.w, 600, "the line ends at the right edge");
}

/* In a left-to-right paragraph a Hebrew phrase reads right to left. */
#[test]
fn an_rtl_run_in_ltr_text_is_reversed() {
    let p = at(
        "<p style=margin:0>go \u{5e9}\u{5dc}\u{5d5}\u{5dd} \u{5e2}\u{5d5}\u{5dc}\u{5dd} now</p>",
    );
    let (go, w1, w2, now) = (
        p.word("go"),
        p.word("\u{5dd}\u{5d5}\u{5dc}\u{5e9}"),
        p.word("\u{5dd}\u{5dc}\u{5d5}\u{5e2}"),
        p.word("now"),
    );
    assert!(go.x < w2.x && w2.x < w1.x && w1.x < now.x, "go, second, first, now");
    assert_eq!(go.x, 0, "an ltr line still starts at the left");
}

/* text-align: start follows the direction; end is the other side. */
#[test]
fn text_align_start_and_end_follow_the_direction() {
    let p = at("<p dir=rtl style='width:600px;margin:0'>abc</p><p style='width:600px;margin:0;text-align:end'>xyz</p>");
    let (abc, xyz) = (p.word("abc"), p.word("xyz"));
    assert_eq!(abc.x + abc.w, 600);
    assert_eq!(xyz.x + xyz.w, 600);
}
