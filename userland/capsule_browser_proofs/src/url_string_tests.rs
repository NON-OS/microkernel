// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! The address of the page on screen, kept apart from the text being
//! edited. It prints in the form `join` builds, so links compare with it.

use crate::browser::url::{join, parse, to_string};

#[test]
fn an_address_prints_back_as_the_browser_shows_it() {
    let u = parse("https://nonos.software/docs?x=1#top").expect("parses");
    assert_eq!(to_string(&u), "https://nonos.software/docs?x=1#top");
    let u = parse("http://example.com:8080").expect("parses");
    assert_eq!(to_string(&u), "http://example.com:8080/");
    let u = parse("https://example.com:443/a").expect("parses");
    assert_eq!(to_string(&u), "https://example.com/a");
}

#[test]
fn a_fragment_link_joins_to_the_same_document() {
    let u = parse("https://nonos.software/docs").expect("parses");
    let shown = to_string(&u);
    assert_eq!(join(&u, "#install"), alloc::format!("{shown}#install"));
}
