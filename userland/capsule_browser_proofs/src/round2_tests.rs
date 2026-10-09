// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! rem against the root's font size, font-size keywords and percentages,
//! per-axis overflow, shrink-to-fit floats, image natural size and
//! attribute hints, and italic text.

use crate::browser::layout::boxmodel::Content;
use crate::probe::Page;
use crate::render::render_with;

const VP: (u32, u32) = (800, 600);

fn px_of(p: &Page, word: &str) -> f32 {
    match &p.word(word).content {
        Content::Text { px, .. } => *px,
        _ => 0.0,
    }
}

#[test]
fn rem_follows_the_root_font_size() {
    let html = "<html><style>html{font-size:62.5%}.r{font-size:1.6rem}.p{font-size:150%}</style>\
                <p class=r>rem</p><p class=p>pct</p><p style=\"font-size:small\">kw</p>";
    let p = Page::at(html, VP);
    assert_eq!(px_of(&p, "rem"), 16.0);
    assert_eq!(px_of(&p, "pct"), 15.0, "150% of the 10px body");
    assert_eq!(px_of(&p, "kw"), 13.0);
}

#[test]
fn overflow_x_alone_does_not_clip_vertically() {
    let html = "<div style=\"height:50px;overflow-x:hidden\"><div id=c style=\"height:200px\">c</div></div>";
    let clip = Page::at(html, VP).frag("c").clip.expect("clipped across");
    assert!(clip[1] < -1_000_000 && clip[3] > 1_000_000, "vertical stays open: {clip:?}");
}

#[test]
fn an_auto_width_float_shrinks_to_its_content() {
    let html = "<body style=\"margin:0\"><nav id=n style=\"float:left\"><a>Stories</a> <a>All</a></nav></body>";
    let r = Page::at(html, VP).rect("n");
    assert!(r[2] > 60 && r[2] < 200 && r[3] < 40, "one short line, not the container: {r:?}");
}

#[test]
fn an_image_takes_its_natural_ratio_under_a_width_attribute() {
    let html = "<body style=\"margin:0\"><img src=\"a.gif\" width=180><img src=\"b.gif\"></body>";
    let doc = render_with(html, VP, &|_| Some((100, 100)));
    let imgs: Vec<[i32; 2]> = doc
        .frags
        .iter()
        .filter(|f| matches!(f.content, Content::Image { .. }))
        .map(|f| [f.w, f.h])
        .collect();
    assert_eq!(imgs, [[180, 180], [100, 100]]);
}

#[test]
fn italic_text_is_flagged_for_a_slanted_draw() {
    let p = Page::at("<style>em{font-style:italic}</style><p>a <em>b</em></p>", VP);
    let italic = |w: &str| matches!(p.word(w).content, Content::Text { italic: true, .. });
    assert!(italic("b") && !italic("a"));
}
