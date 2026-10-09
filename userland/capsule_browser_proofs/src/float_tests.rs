// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Float placement. CSS 2.1 section 9.5.1 rule 5: the outer top of a float
//! may not be higher than the outer top of any float earlier in the source.

use crate::render::{render, texts};

fn y_of(html: &str, word: &str) -> i32 {
    let doc = render(html, 300);
    let t = texts(&doc);
    t.iter().find(|f| f.3 == word).map(|f| f.1).unwrap_or_else(|| panic!("{word} not laid out"))
}

const PAGE: &str = "<body style=\"margin:0\">\
<div style=\"float:left;width:200px;height:50px\">a</div>\
<div style=\"float:left;width:200px;height:20px\">b</div>\
<div style=\"float:left;width:50px;height:20px\">c</div></body>";

#[test]
fn a_float_that_does_not_fit_drops_below_the_one_before_it() {
    assert!(y_of(PAGE, "b") >= y_of(PAGE, "a") + 50);
}

#[test]
fn a_later_float_never_climbs_above_an_earlier_one() {
    /* c is narrow enough to fit beside a, in the gap above b, but b came
     * first in the source, so c starts no higher than b. */
    assert!(
        y_of(PAGE, "c") >= y_of(PAGE, "b"),
        "c at {} above b at {}",
        y_of(PAGE, "c"),
        y_of(PAGE, "b")
    );
}
