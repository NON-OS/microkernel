// NONOS Operating System (AGPL-3.0-or-later)
//! Cascade order: !important over specificity, a later display
//! replacing display:none, a unitless line-height inherited as a number,
//! and var() resolved before background-image reads its url.

use super::probe::Styles;

const RED: u32 = 0xffff_0000;
const BLUE: u32 = 0xff00_00ff;

#[test]
fn important_beats_an_id_selector() {
    let s = Styles::of("<style>#x{color:#0000ff} p{color:#ff0000 !important}</style><p id=x>w</p>");
    assert_eq!(s.get("x").color, RED);
    let s = Styles::of("<style>#x{color:red !important}#x{color:blue}</style><p id=x>w</p>");
    assert_eq!(s.get("x").color, RED, "a later normal declaration loses to !important");
    let s = Styles::of("<style>p{color:#ff0000 !important}</style><p id=x style=\"color:#0000ff !important\">w</p>");
    assert_eq!(s.get("x").color, BLUE, "inline !important beats author !important");
}

#[test]
fn a_later_display_replaces_display_none() {
    let s =
        Styles::of("<style>#d{display:none}#d.x{display:block}</style><div id=d class=x>d</div>");
    assert!(!s.get("d").display_none, "#d.x shows the box");
    let s = Styles::of(
        "<style>.h{display:none}html .h{display:flex}</style><html><div id=d class=h>d</div>",
    );
    assert!(!s.get("d").display_none && s.get("d").is_flex);
    let s = Styles::of("<div id=d hidden style=\"display:block\">d</div>");
    assert!(!s.get("d").display_none, "an author display overrides [hidden]");
}

#[test]
fn a_numeric_line_height_is_inherited_as_a_number() {
    let s = Styles::of("<div id=l style=\"line-height:1.5;font-size:10px\">l<span id=s style=\"font-size:30px\">big</span></div>");
    assert_eq!(s.get("l").line_height(), 15);
    assert_eq!(s.get("s").line_height(), 45, "1.5 times the span's own 30px");
    let s = Styles::of("<style>.d{font-size:3em;line-height:1.05}</style><h1 id=h class=d style=\"font-size:2em\">x</h1>");
    assert_eq!(s.get("h").line_height(), 34, "line-height follows the winning 32px font size");
}

#[test]
fn var_resolves_before_background_image_reads_its_url() {
    let s = Styles::of(
        "<style>#p{--g:url(dot.png);background-image:var(--g)}</style><div id=p>p</div>",
    );
    assert_eq!(s.bg_image("p"), Some("dot.png"));
    let html = "<style>#q{--mx:40%;background:radial-gradient(circle at var(--mx) 50%,#fff,#000)}</style><div id=q>q</div>";
    let s = Styles::of(html);
    let g = s.bg_image("q").expect("gradient kept");
    assert!(g.starts_with("radial-gradient(circle at 40% 50%"), "substituted: {g}");
}
