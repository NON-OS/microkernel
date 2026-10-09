// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! getComputedStyle reads the cascade the page is drawn with and the boxes
//! it was laid out in (dom::style_facts), as the capsule records them after
//! a layout. It answered empty for every property.

use crate::browser::css::style_facts::facts_of;
use crate::browser::css::{collect_css, compute_cached};
use crate::browser::dom::{self, Dom};
use crate::browser::layout::boxmodel::{build, layout};

/* The capsule's relayout, recording rects and facts as it does. */
fn styled(html: &str) -> Dom {
    let vp = (800, 600);
    let mut d = dom::parse(html.as_bytes());
    let css = collect_css(&d);
    let s = compute_cached(&d, &css, vp, &mut None);
    let root = build(&d, &s.styles, &s.bg_images, &s.svg_paint, &s.grids, &s.pseudos, &|_| None);
    let doc = layout(&root, vp);
    drop(root);
    d.record_rects(doc.frags.iter().map(|f| (f.node, f.x, f.y, f.w, f.h)));
    d.record_facts(|id| facts_of(&s.styles[id]));
    d
}

fn id(d: &Dom, want: &str) -> usize {
    d.nodes.iter().position(|n| n.attr("id") == Some(want)).expect("id present")
}

fn get(d: &Dom, el: &str, prop: &str) -> String {
    d.computed_value(id(d, el), prop).unwrap_or_else(|| format!("<not kept: {prop}>"))
}

const PAGE: &str = "<style>
  body { margin: 0 }
  #menu { display: none; color: #102030 }
  .panel { width: 300px; padding: 10px 20px; border: 2px solid; margin: 5px auto;
           background-color: rgba(0, 0, 255, 0.5); font-size: 20px; position: relative }
  .bb { box-sizing: border-box; width: 300px; padding: 10px; height: 50px }
  .ghost { visibility: hidden; opacity: 0.5 }
  .row { display: inline-flex }
</style>
<body><nav id=menu>m</nav><div id=panel class=panel>p</div><div id=bb class=bb>b</div>
<div class=ghost><span id=inner>i</span></div><span id=word>w</span><div id=row class=row>r</div>
<div id=shown style='--gap: 4px; cursor: pointer'>s</div></body>";

#[test]
fn display_colour_and_position_come_from_the_cascade() {
    let d = styled(PAGE);
    assert_eq!(get(&d, "menu", "display"), "none");
    assert_eq!(get(&d, "menu", "color"), "rgb(16, 32, 48)");
    assert_eq!(get(&d, "panel", "display"), "block");
    assert_eq!(get(&d, "panel", "position"), "relative");
    assert_eq!(get(&d, "panel", "background-color"), "rgba(0, 0, 255, 0.5)");
    assert_eq!(get(&d, "panel", "font-size"), "20px");
    assert_eq!(get(&d, "word", "display"), "inline");
    assert_eq!(get(&d, "row", "display"), "inline-flex");
    assert_eq!(get(&d, "word", "background-color"), "rgba(0, 0, 0, 0)");
}

#[test]
fn sizes_and_spacing_are_as_laid_out() {
    let d = styled(PAGE);
    assert_eq!(get(&d, "panel", "width"), "300px", "the content box");
    assert_eq!(get(&d, "panel", "padding"), "10px 20px");
    assert_eq!(get(&d, "panel", "padding-left"), "20px");
    assert_eq!(get(&d, "panel", "border-top-width"), "2px");
    assert_eq!(get(&d, "panel", "margin-top"), "5px");
    /* auto margins are what the layout left either side: 800 - 344 */
    assert_eq!(get(&d, "panel", "margin-left"), "228px");
    assert_eq!(get(&d, "panel", "margin-right"), "228px");
    assert_eq!(get(&d, "bb", "width"), "300px", "border-box reads the border box");
    assert_eq!(get(&d, "bb", "height"), "50px");
    assert_eq!(get(&d, "word", "width"), "auto", "no width applies to an inline box");
    assert_eq!(get(&d, "menu", "height"), "auto", "nor to one not displayed");
}

#[test]
fn visibility_and_opacity_are_each_their_own() {
    let d = styled(PAGE);
    let ghost = d.nodes[id(&d, "inner")].parent;
    assert_eq!(d.computed_value(ghost, "visibility").as_deref(), Some("hidden"));
    assert_eq!(d.computed_value(ghost, "opacity").as_deref(), Some("0.5"));
    assert_eq!(get(&d, "inner", "visibility"), "hidden", "inherited");
    assert_eq!(get(&d, "panel", "visibility"), "visible");
    assert_eq!(get(&d, "panel", "opacity"), "1");
}

#[test]
fn what_is_not_kept_is_said_to_be_not_kept() {
    let d = styled(PAGE);
    assert_eq!(d.computed_value(id(&d, "shown"), "cursor"), None);
    assert_eq!(d.computed_value(id(&d, "shown"), "--gap"), None);
    assert_eq!(Dom::new().computed_value(3, "display"), None, "no layout yet");
}

/* visibility no longer writes opacity: a hidden box still paints nothing,
 * and visibility:visible after an opacity no longer paints it at full. */
#[test]
fn a_hidden_box_paints_nothing_and_visible_keeps_its_opacity() {
    let html = "<div id=g style='opacity:.5;visibility:hidden'>gone</div>\
                <div id=h style='visibility:hidden;opacity:1'>still gone</div>\
                <div id=v style='opacity:.5;visibility:visible'>half</div>";
    let (d, doc) = crate::render::render_full(html, (800, 600), &|_| None);
    let alpha_of = |el: &str| {
        let n = id(&d, el);
        let kid = d.nodes[n].children[0];
        doc.frags.iter().find(|f| f.node == kid || f.node == n).map(|f| f.alpha)
    };
    assert_eq!(alpha_of("g"), Some(0));
    assert_eq!(alpha_of("h"), Some(0), "an opacity after visibility does not show it");
    assert_eq!(alpha_of("v"), Some(127), "visible keeps the opacity set before it");
}
