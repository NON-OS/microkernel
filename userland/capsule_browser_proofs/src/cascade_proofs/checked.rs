// NONOS Operating System (AGPL-3.0-or-later)
//! A checked checkbox or radio button is filled by the UA sheet, so a click
//! that checks one shows on the page without the page styling it.

use super::probe::Styles;

const FILL: u32 = 0xff00_75ff;

#[test]
fn a_checked_box_is_filled_and_an_unchecked_one_is_not() {
    let s = Styles::of(
        "<input type=checkbox id=on checked><input type=checkbox id=off>\
         <input type=radio id=r checked><input id=text value=x checked>",
    );
    assert_eq!(s.get("on").bg, FILL);
    assert_eq!(s.get("r").bg, FILL);
    assert_ne!(s.get("off").bg, FILL);
    assert_ne!(s.get("text").bg, FILL, "a text field is no checkbox");
}
