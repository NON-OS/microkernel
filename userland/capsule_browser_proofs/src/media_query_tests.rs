// NONOS Operating System (AGPL-3.0-or-later)
//! A script's matchMedia is judged by the stylesheet's own @media parser at
//! the document's viewport (qjs_bridge/media.rs), so a page's breakpoints
//! answer as its CSS does rather than false to everything, as they used to.

use crate::browser::css::color::media_query_matches;

#[test]
fn breakpoints_follow_the_viewport() {
    assert!(media_query_matches("(min-width: 768px)", 1280, 720));
    assert!(!media_query_matches("(min-width: 768px)", 600, 720));
    assert!(media_query_matches("(max-width: 767px)", 600, 720));
    assert!(!media_query_matches("(max-width: 767px)", 1280, 720));
    assert!(media_query_matches("screen and (min-width: 1024px)", 1280, 720));
    assert!(!media_query_matches("print", 1280, 720), "the page is on a screen");
    assert!(media_query_matches("", 1280, 720), "an empty list matches");
}
