// NONOS Operating System (AGPL-3.0-or-later)
//! The <noscript> policy: its content renders only when the page's
//! scripts cannot stand in for it; and a linked sheet whose media query
//! cannot apply here is not fetched.

use crate::browser::css::CssCache;
use crate::browser::dom;
use crate::probe::Page;

const VP: (u32, u32) = (800, 600);

#[test]
fn noscript_policy_table() {
    let with_text =
        dom::parse(b"<body><noscript><p>fallback</p></noscript><p>main text</p></body>");
    let only_ns = dom::parse(b"<body><noscript><p>fallback</p></noscript></body>");
    for (d, text_outside) in [(&with_text, true), (&only_ns, false)] {
        assert!(CssCache::noscript_shows(d, false, None), "QuickJS off");
        assert!(CssCache::noscript_shows(d, true, Some(false)), "the scripts failed");
        assert_eq!(CssCache::noscript_shows(d, true, None), !text_outside, "scripts ran");
    }
    let p = Page::at("<body><noscript><p>fallback</p></noscript><p>main text</p></body>", VP);
    assert!(
        crate::render::texts(&p.doc).iter().all(|t| t.3 != "fallback"),
        "hidden beside real content"
    );
    let p = Page::at("<body><noscript><p>fallback</p></noscript></body>", VP);
    p.word("fallback");
}

#[test]
fn a_link_media_query_that_does_not_apply_here_is_not_fetched() {
    let applies = |m: &str| {
        let attr = |k: &str| match k {
            "rel" => Some("stylesheet"),
            "media" => Some(m),
            _ => None,
        };
        crate::browser::sheet::sheet_applies(attr, (1336, 760))
    };
    for m in ["print", "(prefers-color-scheme: dark)", "not all", "(min-width: 5000px)"] {
        assert!(!applies(m), "{m}");
    }
    for m in ["", "all", "screen", "(prefers-color-scheme: light)", "(min-width: 600px)"] {
        assert!(applies(m), "{m}");
    }
}
