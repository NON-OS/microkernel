// NONOS Operating System (AGPL-3.0-or-later)
//! Custom properties: scoped to the subtree that declares them,
//! inherited, with fallbacks, and a reference cycle made invalid.

use super::probe::Styles;

const RED: u32 = 0xffff_0000;
const BLUE: u32 = 0xff00_00ff;

#[test]
fn a_custom_property_scopes_to_its_subtree() {
    let html = "<style>:root{--bg:#ffffff;--fg:#111111}.dark{--bg:#000000;--fg:#eeeeee}\
                .btn-a{--c:#ff0000}.btn-b{--c:#0000ff}.btn{background:var(--c)}\
                body{background:var(--bg);color:var(--fg)}</style>\
                <html><body id=b><p>t</p><div id=a class=\"btn btn-a\">r</div><div id=z class=\"btn btn-b\">b</div>";
    let s = Styles::of(html);
    assert_eq!(s.get("b").color, 0xff11_1111, "no .dark element: the root value");
    assert_eq!(s.get("b").bg, 0xffff_ffff);
    assert_eq!(s.get("a").bg, RED);
    assert_eq!(s.get("z").bg, BLUE);
    let s = Styles::of(
        "<style>.p:hover{--r:4px}.q{width:var(--r,9px)}</style><div id=q class=q>q</div>",
    );
    assert!(
        s.get("q").width == crate::browser::css::Size::Px(9),
        "an unmatched rule defines nothing"
    );
}

#[test]
fn custom_properties_inherit_resolve_fallbacks_and_break_cycles() {
    let html = "<style>#o{--a:#0000ff;--x:var(--y);--y:var(--x)}#i{color:var(--a)}\
                #j{color:var(--x,#ff0000)}</style><div id=o><p id=i>i</p><p id=j>j</p></div>";
    let s = Styles::of(html);
    assert_eq!(s.get("i").color, BLUE, "inherited from the parent's scope");
    assert_eq!(s.get("j").color, RED, "a cycle is invalid, so the fallback applies");
}
