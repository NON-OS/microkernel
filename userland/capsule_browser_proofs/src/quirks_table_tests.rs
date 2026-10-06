// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Quirks mode: a table does not inherit text alignment, so cells inside a
//! <center> keep their text at the start; with a standards doctype they
//! inherit the centring as any box does.

use crate::probe::Page;

const VP: (u32, u32) = (800, 600);

fn title_x(doctype: &str) -> i32 {
    let html = format!(
        "{doctype}<body style=\"margin:0\"><center><table width=\"100%\">\
         <tr><td>Title</td></tr></table></center></body>"
    );
    Page::at(&html, VP).word("Title").x
}

#[test]
fn a_quirks_mode_table_keeps_its_cells_at_the_start() {
    assert!(title_x("") < 40, "left in quirks mode: {}", title_x(""));
}

#[test]
fn a_standards_mode_table_inherits_the_centring() {
    assert!(title_x("<!DOCTYPE html>") > 300);
}

#[test]
fn a_table_nested_in_a_quirks_table_is_not_centred() {
    let html = "<body style=\"margin:0\"><center><table width=\"100%\"><tr><td>\
                <table><tr><td>Item</td></tr></table></td></tr></table></center></body>";
    assert!(Page::at(html, VP).word("Item").x < 40);
}
