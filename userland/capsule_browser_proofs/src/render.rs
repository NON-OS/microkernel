// NONOS Operating System (AGPL-3.0-or-later)
//! Drive the real render pipeline end to end: parse, cascade, box tree,
//! layout. Same call order the capsule uses, so a geometry proof here is a
//! statement about what the device paints.

use crate::browser::css::{collect_css, compute_cached};
use crate::browser::dom;
use crate::browser::layout::boxmodel::{build, layout, BoxDocument, Content, Fragment};

/* The page height a proof lays out against unless it names one: the
 * manifest window's, which vh units resolved against before the viewport
 * was threaded through. */
const PAGE_H: u32 = crate::browser::manifest::HEIGHT;

/* An image's natural size by its src, as the capsule's image store answers. */
type Natural<'a> = &'a dyn Fn(&str) -> Option<(u32, u32)>;

pub fn render(html: &str, viewport_w: u32) -> BoxDocument {
    render_at(html, (viewport_w, PAGE_H))
}

/* The pipeline at a viewport of (width, height) px, no image sizes known. */
pub fn render_at(html: &str, viewport: (u32, u32)) -> BoxDocument {
    render_with(html, viewport, &|_| None)
}

/* The pipeline with image natural sizes looked up by src. */
pub fn render_with(html: &str, viewport: (u32, u32), natural: Natural<'_>) -> BoxDocument {
    render_full(html, viewport, natural).1
}

/* The parsed document alongside its layout, for proofs that find a box by
 * its element. */
pub fn render_full(html: &str, vp: (u32, u32), natural: Natural<'_>) -> (dom::Dom, BoxDocument) {
    let d = dom::parse(html.as_bytes());
    let css = collect_css(&d);
    let s = compute_cached(&d, &css, vp, &mut None);
    let root = build(&d, &s.styles, &s.bg_images, &s.svg_paint, &s.grids, &s.pseudos, natural);
    let doc = layout(&root, vp);
    (d, doc)
}

/* Text a fragment paints, for tests that care about where a word landed. */
pub fn text_of(f: &Fragment) -> Option<&str> {
    match &f.content {
        Content::Text { text, .. } => Some(text.as_str()),
        _ => None,
    }
}

/* Every text fragment as (x, y, w, text), in paint order. */
pub fn texts(doc: &BoxDocument) -> alloc::vec::Vec<(i32, i32, i32, &str)> {
    doc.frags.iter().filter_map(|f| text_of(f).map(|t| (f.x, f.y, f.w, t))).collect()
}
