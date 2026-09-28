// NONOS Operating System (AGPL-3.0-or-later)
//! Path data after a closepath. `Z` reads nothing, so a number after it used
//! to leave the parser on `Z` forever: one SVG on a page hung the browser.

use std::sync::mpsc;
use std::time::Duration;

use super::svg::{decode_svg, is_svg};

fn decodes_in_time(svg: &'static str) -> bool {
    assert!(is_svg(svg.as_bytes()));
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let _ = tx.send(decode_svg(svg.as_bytes(), (64, 64)).is_some());
    });
    rx.recv_timeout(Duration::from_secs(10)).expect("decode_svg did not return in 10 s")
}

#[test]
fn a_number_after_closepath_ends_the_path() {
    assert!(decodes_in_time(concat!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 10 10\">",
        "<path d=\"M1 1 L9 1 L9 9 Z 5\"/></svg>"
    )));
}

#[test]
fn a_new_subpath_after_closepath_still_draws() {
    assert!(decodes_in_time(concat!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 10 10\">",
        "<path d=\"M1 1 L9 1 L9 9 Z M2 2 L8 2 L8 8 z\"/></svg>"
    )));
}
