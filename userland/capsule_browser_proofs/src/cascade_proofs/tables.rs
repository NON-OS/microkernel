// NONOS Operating System (AGPL-3.0-or-later)
//! Table layout end to end: colspan and rowspan in the grid, border-spacing,
//! presentational hints (bgcolor, border, cellpadding), no default cell
//! border, and shrink-to-fit auto tables.

use crate::probe::Page;

const VP: (u32, u32) = (800, 600);

#[test]
fn a_colspan_cell_spans_two_columns_and_the_gap() {
    let html = "<body style=\"margin:0\"><table cellspacing=4><tr><td id=a style=\"width:100px\">a</td>\
                <td id=b style=\"width:60px\">b</td></tr><tr><td id=s colspan=2>s</td></tr></table>";
    let p = Page::at(html, VP);
    let (a, b, s) = (p.rect("a"), p.rect("b"), p.rect("s"));
    assert_eq!(s[2], a[2] + b[2] + 4, "two columns plus the spacing between them");
    assert_eq!(s[0], a[0]);
    assert_eq!(b[0], a[0] + a[2] + 4);
}

#[test]
fn a_rowspan_cell_holds_its_column_in_the_next_row() {
    let html =
        "<table><tr><td id=r rowspan=2>r</td><td id=a>a</td></tr><tr><td id=b>b</td></tr></table>";
    let p = Page::at(html, VP);
    let (r, a, b) = (p.rect("r"), p.rect("a"), p.rect("b"));
    assert_eq!(b[0], a[0], "the second row's cell starts in column two");
    assert_eq!(r[1] + r[3], b[1] + b[3], "the spanning cell fills both rows");
}

#[test]
fn bgcolor_paints_and_no_border_attribute_means_no_cell_border() {
    let html =
        "<table><tr><td id=c bgcolor=#ff6600>c</td><td id=d bgcolor=\"f6f6ef\">d</td></tr></table>";
    let p = Page::at(html, VP);
    assert_eq!(p.frag("c").bg, 0xffff_6600);
    assert_eq!(p.frag("d").bg, 0xfff6_f6ef, "bare hex digits as legacy pages write them");
    assert_eq!(p.frag("c").border, [0; 4]);
    let p = Page::at("<table border=1 cellpadding=5><tr><td id=c>c</td></tr></table>", VP);
    assert_eq!(p.frag("c").border, [1; 4], "border=1 gives each cell a 1px border");
}

#[test]
fn an_auto_table_with_short_text_shrinks_to_fit() {
    let p = Page::at("<table id=t><tr><td>short</td><td>text</td></tr></table>", VP);
    let t = p.rect("t");
    assert!(t[2] > 40 && t[2] < 200, "the table is as wide as its cells: {t:?}");
    let p = Page::at("<table id=t width=500><tr><td>short</td></tr></table>", VP);
    assert_eq!(p.rect("t")[2], 500, "a width attribute sets the width");
    let p = Page::at("<center><table id=t><tr><td>mid</td></tr></table></center>", VP);
    let t = p.rect("t");
    assert!((t[0] + t[2] / 2 - 400).abs() <= 2, "<center> centres the table: {t:?}");
}

#[test]
fn cells_align_middle_by_default_and_fill_their_row() {
    let html = "<table><tbody><tr><td id=a style=\"height:60px\">a</td><td id=b>b</td><td id=c valign=top>c</td></tr></tbody></table>";
    let p = Page::at(html, VP);
    let (a, b) = (p.rect("a"), p.rect("b"));
    assert_eq!(b[3], a[3], "the short cell's box fills the row");
    let (wb, wc) = (p.word("b"), p.word("c"));
    assert!(wb.y > wc.y + 15, "b sits in the middle, c at the top: {} {}", wb.y, wc.y);
}
