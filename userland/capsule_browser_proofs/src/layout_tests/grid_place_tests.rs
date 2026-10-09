// NONOS Operating System (AGPL-3.0-or-later)
//! Grid items go where their lines say, numbered lines included (-1 is
//! the last line), the rest auto-place behind a cursor that only moves
//! forward, and row and column gaps and self-alignment are honoured.

use crate::probe::Page;

const VP: (u32, u32) = (1336, 800);

fn grid(cols: &str, extra: &str, items: &str) -> Page {
    let html = format!(
        "<body style=margin:0><div style='display:grid;width:800px;\
         grid-template-columns:{cols};{extra}'>{items}</div>"
    );
    Page::at(&html, VP)
}

/* 1 / -1 spans both columns of 1fr 1fr; grid-column: 2 is the second. */
#[test]
fn numbered_lines_place_items_without_named_lines() {
    let p = grid("1fr 1fr", "", "<h2 id=h style='grid-column:1/-1;margin:0'>T</h2><p id=p style='grid-column:2;margin:0'>P</p>");
    assert_eq!(p.rect("h")[0..3], [0, 0, 800], "1 / -1 spans the grid");
    let r = p.rect("p");
    assert_eq!((r[0], r[2]), (400, 400), "grid-column: 2 is the second track");
    assert!(r[1] > p.rect("h")[1], "and on the next row");
}

/* justify-self: end puts a 100px item at the right of its column. */
#[test]
fn justify_self_end_right_aligns() {
    let p = grid("1fr", "", "<div id=i style='justify-self:end;width:100px'>x</div>");
    assert_eq!(p.rect("i")[0..3], [700, 0, 100]);
}

/* gap: 6px 14px is 6 between rows and 14 between columns. */
#[test]
fn a_two_value_gap_sets_rows_then_columns() {
    let p =
        grid("100px 100px", "gap:6px 14px", "<div id=a>a</div><div id=b>b</div><div id=c>c</div>");
    let (a, b, c) = (p.rect("a"), p.rect("b"), p.rect("c"));
    assert_eq!(b[0] - (a[0] + a[2]), 14, "column gap");
    assert_eq!(c[1] - (a[1] + a[3]), 6, "row gap");
}

/* Sparse auto-placement never goes back: after an item locked to column 1
 * starts a new row, the next auto item follows it there, not into the
 * hole left behind in the first row. */
#[test]
fn auto_placement_cursor_only_moves_forward() {
    let p = grid(
        "repeat(3, 100px)",
        "",
        "<div id=a>a</div><div id=b style='grid-column:1'>b</div><div id=c>c</div>",
    );
    let (a, b, c) = (p.rect("a"), p.rect("b"), p.rect("c"));
    assert_eq!((a[0], b[0], c[0]), (0, 0, 100));
    assert!(b[1] > a[1], "b starts the second row");
    assert_eq!(c[1], b[1], "c follows b, not back into the first row");
}
