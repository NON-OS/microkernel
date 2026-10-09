// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Paint order, CSS 2.1 Appendix E: within a stacking context the root's
//! background (step 1), negative z-index children (2), in-flow blocks and
//! inline content (3 to 7), positioned descendants with z-index auto or 0
//! in tree order (8), then positive z-index children (9). A context paints
//! as a unit inside its parent's, so z-index values compare only among
//! siblings of one context.
use crate::browser::layout::boxmodel::{BoxDocument, Content, Fragment};
use crate::shim::render;

fn doc(body: &str) -> BoxDocument {
    render(&std::format!("<!doctype html><html><body>{body}</body></html>"), 1336)
}

/* Where in paint order the first fragment `hit` picks is. */
fn at(d: &BoxDocument, hit: impl Fn(&Fragment) -> bool) -> usize {
    d.frags.iter().position(hit).expect("fragment present")
}

fn text(t: &'static str) -> impl Fn(&Fragment) -> bool {
    move |f| matches!(&f.content, Content::Text { text, .. } if text == t)
}

fn image(f: &Fragment) -> bool {
    matches!(f.content, Content::Image { .. })
}

const ARC: &str = "<svg style=\"position:absolute;left:0;top:0;width:300px;height:300px;\
                   z-index:0\" viewBox=\"0 0 10 10\"><rect width=\"10\" height=\"10\"/></svg>";

#[test]
fn the_hero_headline_in_a_z_index_1_context_paints_over_the_z_index_0_arc() {
    let d = doc(&std::format!(
        "<div style=\"position:relative;z-index:10\"><section style=\"position:relative\">\
         {ARC}<div style=\"position:relative;z-index:1\"><h1>Head</h1></div></section></div>"
    ));
    assert!(at(&d, image) < at(&d, text("Head")), "z-index 1 over 0 in #page's context");
}

#[test]
fn positioned_boxes_paint_over_later_non_positioned_content() {
    let d = doc(&std::format!("<section style=\"position:relative\">{ARC}<h1>Head</h1></section>"));
    assert!(at(&d, text("Head")) < at(&d, image), "step 8 follows step 7");
    let d = doc("<div style=\"position:relative\"><p>Lifted</p></div>\
         <div style=\"margin-top:-9px;background:#ff0000\"><p>Flat</p></div>");
    let red = at(&d, |f| f.bg == 0xFFFF_0000);
    assert!(red < at(&d, text("Lifted")), "a later in-flow background (step 4) lies under it");
}

#[test]
fn a_negative_child_paints_between_its_context_background_and_content() {
    let d = doc("<div style=\"position:relative;z-index:0;background:#0000ff\">\
         <div style=\"position:absolute;z-index:-1;width:9px;height:9px;background:#ff0000\">\
         </div><p>Body</p></div>");
    let (own, neg) = (at(&d, |f| f.bg == 0xFF00_00FF), at(&d, |f| f.bg == 0xFFFF_0000));
    assert!(own < neg && neg < at(&d, text("Body")), "steps 1, 2, then 7");
}
