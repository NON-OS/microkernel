// NONOS Operating System (AGPL-3.0-or-later)
//! A script's el.style: a set replaces its property in the style attribute,
//! an empty value removes it, and a read answers what the attribute holds.

use crate::browser::css::parse::{parse_decls, style_get, style_set};

/* The CSS name a script's spelling is written under. */
fn css_name(prop: &str) -> String {
    let written = style_set("", prop, "x");
    written.strip_suffix(": x;").expect("one declaration").to_string()
}

#[test]
fn script_names_become_css_names() {
    assert_eq!(css_name("display"), "display");
    assert_eq!(css_name("fontSize"), "font-size");
    assert_eq!(css_name("borderTopLeftRadius"), "border-top-left-radius");
    assert_eq!(css_name("cssFloat"), "float");
    assert_eq!(css_name("webkitTransform"), "-webkit-transform");
    assert_eq!(css_name("WebkitTransform"), "-webkit-transform");
    assert_eq!(css_name("MozAppearance"), "-moz-appearance");
    assert_eq!(css_name("msOverflowStyle"), "-ms-overflow-style");
    assert_eq!(css_name("overflowX"), "overflow-x", "an o that starts a word is no vendor");
    assert_eq!(css_name("opacity"), "opacity");
    assert_eq!(css_name("margin-top"), "margin-top", "setProperty's spelling is kept");
    assert_eq!(css_name("--Brand-Color"), "--Brand-Color", "a custom property is kept");
}

#[test]
fn a_menu_toggle_opens_again() {
    // The toggle reads the property before it writes it.
    let mut style = String::from("color: red");
    for want in ["none", "block", "none", "block"] {
        let now = style_get(&style, "display");
        let next = if now == "none" { "block" } else { "none" };
        assert_eq!(next, want);
        style = style_set(&style, "display", next);
        assert_eq!(style_get(&style, "display"), want);
    }
    assert_eq!(style, "color: red; display: block;");
}

#[test]
fn a_property_set_every_frame_does_not_grow_the_attribute() {
    let mut style = String::from("position: absolute");
    for x in 0..10_000 {
        style = style_set(&style, "left", &format!("{x}px"));
    }
    assert_eq!(style, "position: absolute; left: 9999px;");
}

#[test]
fn an_empty_value_removes_the_property_and_others_keep_their_text() {
    let style = "background: url(data:image/png;base64,AAAA); display: none; DISPLAY: flex";
    assert_eq!(style_get(style, "display"), "flex", "the last declaration wins");
    let left = style_set(style, "display", "");
    assert_eq!(left, "background: url(data:image/png;base64,AAAA);");
    assert_eq!(style_get(&left, "display"), "");
    assert_eq!(style_get(&left, "backgroundImage"), "");
    assert_eq!(style_set("display: none", "display", "  "), "", "nothing is left");
}

#[test]
fn a_read_leaves_out_the_priority_and_the_page_still_parses_the_result() {
    let style = "color: red !important; font-size:12px";
    assert_eq!(style_get(style, "color"), "red");
    assert_eq!(style_get(style, "fontSize"), "12px");
    let written = style_set(style, "marginTop", "4px");
    assert_eq!(written, "color: red !important; font-size:12px; margin-top: 4px;");
    let decls = parse_decls(&written);
    assert_eq!(decls.len(), 3, "every declaration still reaches the cascade");
}

#[test]
fn custom_properties_are_case_sensitive() {
    let style = style_set("--gap: 2px", "--Gap", "8px");
    assert_eq!(style, "--gap: 2px; --Gap: 8px;");
    assert_eq!(style_get(&style, "--gap"), "2px");
    assert_eq!(style_get(&style, "--Gap"), "8px");
}
