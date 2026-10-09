// NONOS Operating System (AGPL-3.0-or-later)
//! At-rules: @supports judged against the engine's own property
//! appliers, @container against the viewport width, deep @media, and
//! nesting with & and the implicit descendant.

use alloc::format;

use super::probe::Styles;

const RED: u32 = 0xffff_0000;
const GREEN: u32 = 0xff00_ff00;
const BLUE: u32 = 0xff00_00ff;

#[test]
fn supports_applies_what_the_engine_styles() {
    let css =
        "@supports (display:grid){#a{color:#0000ff}}@supports (frobnicate:1){#b{color:#ff0000}}\
               @supports not (frobnicate:1){#c{color:#00ff00}}\
               @supports (display:flex) and ((color:red) or (x:y)){#d{color:#0000ff}}\
               @supports selector(a > b){#e{color:#0000ff}}";
    let s = Styles::of(&format!(
        "<style>{css}</style><p id=a>a</p><p id=b>b</p><p id=c>c</p><p id=d>d</p><p id=e>e</p>"
    ));
    assert_eq!(s.get("a").color, BLUE);
    assert_ne!(s.get("b").color, RED, "an unknown property is unsupported");
    assert_eq!(s.get("c").color, GREEN);
    assert_eq!(s.get("d").color, BLUE);
    assert_eq!(s.get("e").color, BLUE);
}

#[test]
fn container_queries_and_deep_media_apply() {
    let css = "@container (min-width:10px){#c{height:50px}}\
               @media screen{@media (min-width:1px){@media all{@media all{@media all{#m{height:40px}}}}}}";
    let s = Styles::of(&format!("<style>{css}</style><div id=c>c</div><div id=m>m</div>"));
    assert!(s.get("c").height == crate::browser::css::Size::Px(50));
    assert!(s.get("m").height == crate::browser::css::Size::Px(40), "five levels of @media");
}

#[test]
fn nesting_parses_both_rules() {
    let css = ".card{color:#ff0000; & .t{color:#0000ff} .u{color:#00ff00} &:hover{color:#000000}}";
    let s = Styles::of(&format!("<style>{css}</style><div id=c class=card><p id=t class=t>t</p><p id=u class=u>u</p></div><p id=o class=t>o</p>"));
    assert_eq!(s.get("c").color, RED);
    assert_eq!(s.get("t").color, BLUE);
    assert_eq!(s.get("u").color, GREEN);
    assert_ne!(s.get("o").color, BLUE, "& .t needs a .card ancestor");
}
