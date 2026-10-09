// NONOS Operating System (AGPL-3.0-or-later)
//! Flex columns and grid areas: free height in a column of definite
//! height goes to the items that grow, factors summing under 1 handing out
//! only that share; auto margins keep an item from stretching across.

use crate::probe::Page;

const VP: (u32, u32) = (1336, 800);

/* flex-grow: .5 alone in a 200px column with 20px of content takes half
 * of the 180px left over. */
#[test]
fn grow_factors_under_one_take_their_share() {
    let p = Page::at(
        "<body style=margin:0><div style='display:flex;flex-direction:column;height:200px'>\
         <div id=a style='flex-grow:.5;line-height:20px'>a</div></div>",
        VP,
    );
    assert_eq!(p.rect("a")[3], 20 + 90);
}

/* A whole factor takes all of it. */
#[test]
fn a_grow_factor_of_one_takes_all_free_height() {
    let p = Page::at(
        "<body style=margin:0><div style='display:flex;flex-direction:column;height:200px'>\
         <div id=a style='flex-grow:1;line-height:20px'>a</div></div>",
        VP,
    );
    assert_eq!(p.rect("a")[3], 200);
}

/* A grid item with auto margins is not stretched: it fits its content and
 * the margins centre it in its column. */
#[test]
fn auto_margins_centre_a_grid_item() {
    let p = Page::at(
        "<body style=margin:0><div style='display:grid;grid-template-columns:400px'>\
         <div id=i style='margin:0 auto'>x</div></div>",
        VP,
    );
    let r = p.rect("i");
    assert!(r[2] > 0 && r[2] < 40, "fits its content: {r:?}");
    assert!((r[0] * 2 + r[2] - 400).abs() <= 1, "centred: {r:?}");
}

/* In a flex column the same holds across: auto margins centre the item. */
#[test]
fn auto_margins_centre_a_flex_column_item() {
    let p = Page::at(
        "<body style=margin:0><div style='display:flex;flex-direction:column;width:400px'>\
         <div id=i style='margin:0 auto'>x</div></div>",
        VP,
    );
    let r = p.rect("i");
    assert!(r[2] > 0 && r[2] < 40, "fits its content: {r:?}");
    assert!((r[0] * 2 + r[2] - 400).abs() <= 1, "centred: {r:?}");
}
