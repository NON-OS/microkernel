// NONOS Operating System (AGPL-3.0-or-later)
//! The interaction pseudo-classes read the matcher's element state. Nothing
//! on device sets it yet, so they match nothing there; set here, they follow
//! it: :hover and :active climb to every ancestor, :focus-within too, and
//! :focus-visible needs a keyboard focus. One test, since the state is
//! process-wide and this binary is its only writer.

use capsule_browser_proofs::browser::css::matching::{
    element_state, set_active, set_focused, set_hovered, set_target, ElementState,
};
use capsule_browser_proofs::browser::{css, dom};

const PAGE: &str = "<!DOCTYPE html><html><head></head><body><nav><ul><li><a href=/x id=a>x</a>\
</li></ul></nav><form><input id=i></form><p id=t>t</p></body></html>";

#[test]
fn interaction_states_follow_the_element_state() {
    let d = dom::parse(PAGE.as_bytes());
    let n = |sel: &str| css::select(&d, sel, usize::MAX).len();
    let id = |sel: &str| css::select(&d, sel, 1)[0];
    assert_eq!(element_state(), ElementState::default(), "nothing is hovered at rest");
    assert_eq!(n(":hover") + n(":focus") + n(":active") + n(":target"), 0);

    set_hovered(Some(id("#a")));
    assert_eq!(n(":hover"), 6, "a, li, ul, nav, body and html");
    assert_eq!(n("nav:hover a"), 1);
    set_active(Some(id("#a")));
    assert_eq!(n("a:active"), 1);

    set_focused(Some(id("#i")), false);
    assert_eq!(n(":focus"), 1);
    assert_eq!(n(":focus-visible"), 0, "a pointer focus shows no ring");
    assert_eq!(n(":focus-within"), 4, "input, form, body and html");
    set_focused(Some(id("#i")), true);
    assert_eq!(n("input:focus-visible"), 1);

    set_target(Some(id("#t")));
    assert_eq!(n(":target"), 1);

    let st = css::compute(&d, "a:hover{font-size:40px}").styles;
    assert_eq!(st[id("#a")].font_size_px, 40, "the cascade sees the state too");

    set_hovered(None);
    set_active(None);
    set_focused(None, false);
    set_target(None);
    assert_eq!(n(":hover") + n(":focus") + n(":active") + n(":target"), 0);
}
