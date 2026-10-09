// NONOS Operating System (AGPL-3.0-or-later)
//! Cascade layers and sheet limits: later layers win, unlayered rules
//! beat layered ones and importance reverses the order; a sheet past
//! 4,096 rules keeps its last; an invalid selector list drops its rule.

use alloc::format;
use alloc::string::String;

use super::probe::Styles;

const RED: u32 = 0xffff_0000;
const GREEN: u32 = 0xff00_ff00;
const BLUE: u32 = 0xff00_00ff;

#[test]
fn layer_order_decides_before_specificity() {
    let css = "@layer a, b;@layer b{p{color:#0000ff}}@layer a{#l{color:#ff0000}}\
               @layer x{#u{color:#ff0000}}p.u{color:#00ff00}";
    let s = Styles::of(&format!("<style>{css}</style><p id=l>l</p><p id=u class=u>u</p>"));
    assert_eq!(s.get("l").color, BLUE, "layer b is declared after a");
    assert_eq!(s.get("u").color, GREEN, "unlayered beats layered");
    let css = "@layer a{#i{color:#ff0000!important}}#i{color:#0000ff!important}";
    let s = Styles::of(&format!("<style>{css}</style><p id=i>i</p>"));
    assert_eq!(s.get("i").color, RED, "important declarations reverse the layer order");
}

#[test]
fn a_twenty_thousand_rule_sheet_keeps_its_last_rule() {
    let mut css = String::new();
    for i in 0..20_000 {
        css.push_str(&format!(".r{i}{{margin-left:{}px}}", i % 7));
    }
    css.push_str("#last{color:#0000ff}");
    let s = Styles::of(&format!("<style>{css}</style><p id=last>x</p>"));
    assert_eq!(s.get("last").color, BLUE);
}

#[test]
fn an_invalid_selector_list_drops_the_rule() {
    /* An empty selector in the list is invalid; which other selectors
     * are is the selector parser's to say. */
    let css = "#j, , p{color:#ff0000}#m,{color:#ff0000}#k, p{color:#0000ff}";
    let s = Styles::of(&format!("<style>{css}</style><p id=j>j</p><p id=m>m</p><p id=k>k</p>"));
    assert_ne!(s.get("j").color, RED, "an empty selector between commas");
    assert_ne!(s.get("m").color, RED, "a trailing comma");
    assert_eq!(s.get("k").color, BLUE, "a valid list applies");
}
