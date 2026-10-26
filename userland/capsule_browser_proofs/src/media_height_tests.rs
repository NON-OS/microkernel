// NONOS Operating System (AGPL-3.0-or-later)
//! Height features are judged at the page's own viewport height, as width
//! is, not at the browser window's opening size (manifest HEIGHT 760), so a
//! page's height breakpoints follow a resized or smaller window.

use crate::browser::css::color::media_query_matches;

#[test]
fn height_features_follow_the_viewport_height() {
    for (h, tall) in [(500, false), (700, true), (900, true)] {
        assert_eq!(media_query_matches("(min-height: 600px)", 1280, h), tall, "min at {h}");
        assert_eq!(media_query_matches("(max-height: 599px)", 1280, h), !tall, "max at {h}");
        assert_eq!(media_query_matches("(height >= 600px)", 1280, h), tall, "range at {h}");
    }
    assert!(media_query_matches("(400px <= height <= 550px)", 1280, 500));
    assert!(!media_query_matches("(400px <= height <= 550px)", 1280, 760));
}
