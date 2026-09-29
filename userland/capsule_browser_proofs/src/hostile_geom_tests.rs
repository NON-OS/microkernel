// NONOS Operating System (AGPL-3.0-or-later)
//! Hostile geometry: deep nests of positioned, transformed and clipped
//! boxes, absurd lengths and math, lay out without a panic or a hang.

use crate::render::render_at;

fn nest(open: &str, n: usize) -> String {
    let mut s = String::from("<body>");
    for _ in 0..n {
        s.push_str(open);
    }
    s.push('x');
    for _ in 0..n {
        s.push_str("</div>");
    }
    s
}

#[test]
fn deep_absolute_and_fixed_nests_lay_out() {
    for open in [
        "<div style=\"position:absolute;bottom:1px;right:-3px\">",
        "<div style=\"position:fixed;inset:0;transform:rotate(3deg)\">",
        "<div style=\"transform:scale(1.01) translate(-50%);clip-path:circle(40%)\">",
        "<div style=\"float:left;margin-left:-100%;aspect-ratio:3/1\">",
    ] {
        let doc = render_at(&nest(open, 450), (800, 600));
        assert!(!doc.frags.is_empty());
    }
}

#[test]
fn absurd_lengths_and_math_stay_bounded() {
    let css = [
        "width:calc(1e30px * 1e30)",
        "margin:-99999999px",
        "padding:clamp(1px, 999999vw, 1e9px)",
        "font-size:1e9px",
        "width:min(100%, 1px, 2px, 3px, 4px, 5px, 6%)",
        "height:max(0px, calc((100vh - 100vw / 0) / 2))",
        "transform:matrix(1e30, 0, 0, 1e30, 1e30, -1e30)",
        "border-radius:1e9%",
        "inset:-1e9px;position:absolute",
        "aspect-ratio:1e-30 / 1e30",
        "line-height:1e9",
    ];
    for c in css {
        let html = format!("<div style=\"{c}\">word <span style=\"{c}\">more</span></div>");
        let doc = render_at(&html, (800, 600));
        assert!(doc.content_h < 1 << 30, "{c}: page height {}", doc.content_h);
    }
}
