// NONOS Operating System (AGPL-3.0-or-later)
//! Grid rows: an explicit row template sizes the rows it lists, implicit
//! rows take grid-auto-rows, and an item spanning rows covers them all.

use crate::probe::Page;

const VP: (u32, u32) = (1336, 800);

fn grid(cols: &str, extra: &str, items: &str) -> Page {
    let html = format!(
        "<body style=margin:0><div style='display:grid;width:800px;\
         grid-template-columns:{cols};{extra}'>{items}</div>"
    );
    Page::at(&html, VP)
}

/* An explicit row template sizes the rows; implicit rows use auto-rows. */
#[test]
fn template_rows_and_auto_rows_size_the_rows() {
    let p = grid(
        "1fr",
        "grid-template-rows:50px;grid-auto-rows:30px",
        "<div id=a>a</div><div id=b>b</div><div id=c>c</div>",
    );
    let (a, b, c) = (p.rect("a"), p.rect("b"), p.rect("c"));
    assert_eq!((a[3], b[1], b[3], c[1]), (50, 50, 30, 80));
}

/* A row span covers the rows it names; a later auto item flows beside it. */
#[test]
fn a_row_span_holds_its_column() {
    let p = grid(
        "100px 100px",
        "grid-auto-rows:40px",
        "<div id=a style='grid-row:span 2'>a</div><div id=b>b</div><div id=c>c</div>",
    );
    let (a, b, c) = (p.rect("a"), p.rect("b"), p.rect("c"));
    assert_eq!(a[3], 80, "two 40px rows");
    assert_eq!((b[0], b[1], c[0], c[1]), (100, 0, 100, 40), "b and c stack beside a");
}
