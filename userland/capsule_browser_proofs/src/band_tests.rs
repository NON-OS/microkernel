// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! A band repaint moves the scroll down by the band's top. Boxes that
//! scroll with the page land right under that shift; a fixed or sticky box
//! does not, so a page holding one must take the full repaint instead.

use crate::browser::omnibox::band_safe;
use crate::render::render;

const FIXED: &str = "<html><head><style>body{margin:0}\
    .mast{position:fixed;inset:0 0 auto 0;height:40px;background:#123}\
    .mark{position:fixed;bottom:8px;left:8px}\
    </style></head><body><div class=mast>nav</div>\
    <p>text</p><div style=height:3000px></div>\
    <div class=mark>build</div></body></html>";

const STICKY: &str = "<html><head><style>body{margin:0}\
    h2{position:sticky;top:0}</style></head><body>\
    <h2>head</h2><div style=height:3000px></div></body></html>";

const FLOW: &str = "<html><head><style>body{margin:0}</style></head>\
    <body><p>one</p><div style=height:3000px></div><p>two</p></body></html>";

fn safe(page: &str) -> bool {
    let doc = render(page, 800);
    band_safe(doc.frags.iter().map(|f| (f.fixed, f.sticky.is_some())))
}

#[test]
fn a_page_with_a_fixed_or_sticky_box_is_not_band_safe() {
    assert!(!safe(FIXED));
    assert!(!safe(STICKY));
    assert!(safe(FLOW));
}

#[test]
fn band_safe_needs_every_box_to_scroll() {
    assert!(band_safe([]));
    assert!(band_safe([(false, false), (false, false)]));
    assert!(!band_safe([(false, false), (true, false)]));
    assert!(!band_safe([(false, true)]));
}
