// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Inline SVG takes the page's cascade: rules that match SVG elements, and
//! var() in presentation attributes, reach the serialized document the
//! rasterizer reads, resolved; the element's color stands for currentColor.

use crate::browser::layout::boxmodel::Content;
use crate::probe::Page;

const VP: (u32, u32) = (1000, 600);

/* The serialized SVG of the image box painted for element `id`. */
fn svg_of(html: &str, id: &str) -> String {
    let p = Page::at(html, VP);
    match &p.frag(id).content {
        Content::Image { src, .. } => src.clone(),
        _ => panic!("#{id} is not an image box"),
    }
}

#[test]
fn a_page_rule_styles_an_inline_path_with_its_variables_resolved() {
    let html = "<style>:root{--accent:#12e0ff}.s path{fill:none;stroke:var(--accent);\
                stroke-width:clamp(14px, 1.9vw, 36px)}</style>\
                <svg id=s class=s viewBox=\"0 0 10 10\"><path d=\"M0 0L9 9\"/></svg>";
    let svg = svg_of(html, "s");
    assert!(svg.contains("fill:none;stroke:#12e0ff;stroke-width:19;"), "{svg}");
}

#[test]
fn a_variable_in_a_presentation_attribute_is_resolved() {
    let html = "<style>:root{--a:red}</style><svg id=s viewBox=\"0 0 1 1\">\
                <stop offset=\"0\" stop-color=\"var(--a-x, var(--a))\"/></svg>";
    let svg = svg_of(html, "s");
    assert!(svg.contains(";stop-color:red;\""), "{svg}");
}

#[test]
fn a_page_rule_outranks_a_presentation_attribute() {
    let html = "<style>svg rect{fill:blue}</style>\
                <svg id=s viewBox=\"0 0 1 1\"><rect fill=\"red\" width=\"1\" height=\"1\"/></svg>";
    let svg = svg_of(html, "s");
    assert!(svg.contains(
        "fill=\"red\" width=\"1\" height=\"1\" style=\"color:rgba(26,26,26,1.000);fill:blue;\""
    ));
}

#[test]
fn current_color_follows_the_css_color_of_the_icon() {
    let html =
        "<p style=\"color:rgb(10,20,30)\"><svg id=s viewBox=\"0 0 1 1\" fill=\"currentColor\">\
                <rect width=\"1\" height=\"1\"/></svg></p>";
    assert!(svg_of(html, "s").contains("color:rgba(10,20,30,1.000)"));
}

#[test]
fn the_outer_svg_leaves_its_own_opacity_to_its_box() {
    let html = "<style>svg{opacity:.5}</style><svg id=s viewBox=\"0 0 1 1\"></svg>";
    assert!(!svg_of(html, "s").contains("opacity"));
}
