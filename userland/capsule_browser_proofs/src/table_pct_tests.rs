// NONOS Operating System (AGPL-3.0-or-later)
#![cfg(test)]
//! Tables: a percentage is the width a column prefers, so columns still
//! shrink to fit the table, and an absolutely positioned part leaves it.

use crate::probe::Page;

const VP: (u32, u32) = (800, 600);

#[test]
fn percentage_columns_leave_an_auto_column_its_minimum() {
    let html = "<body style=\"margin:0\"><table id=t style=\"width:400px;border-spacing:0\"><tr>\
                <td style=\"width:33%\">a</td><td style=\"width:66%\">b</td>\
                <td id=c style=\"white-space:nowrap\">2026-09-28 03:12:49</td></tr></table></body>";
    let p = Page::at(html, VP);
    let (t, c) = (p.rect("t"), p.rect("c"));
    assert!(c[0] + c[2] <= t[0] + t[2], "cell {c:?} spills out of table {t:?}");
}

#[test]
fn a_percentage_column_takes_its_share_when_there_is_room() {
    let html = "<body style=\"margin:0\"><table style=\"width:400px;border-spacing:0\"><tr>\
                <td id=a style=\"width:25%;padding:0\">a</td><td>b</td></tr></table></body>";
    assert_eq!(Page::at(html, VP).rect("a")[2], 100);
}

#[test]
fn an_absolutely_positioned_header_is_not_a_table_row() {
    let html = "<body style=\"margin:0\"><table style=\"border-spacing:0\">\
                <thead style=\"position:absolute;width:1px;height:1px;overflow:hidden\">\
                <tr><th>Filename</th></tr></thead><tbody><tr><td id=c>x</td></tr></tbody></table></body>";
    assert_eq!(Page::at(html, VP).rect("c")[1], 0, "the body row is the first row");
}
