// NONOS Operating System (AGPL-3.0-or-later)

//! Pages: one object each, and the y axis flipped.

use super::pdf::{doc_of, metrics, pdf_of, tm_ys};

#[test]
fn a_two_page_document_emits_two_page_objects() {
    let src = pdf_of(&doc_of(5));
    assert_eq!(src.matches("/Type /Page /Parent").count(), 2);
    assert_eq!(src.matches("/Type /Pages ").count(), 1);
    assert_eq!(src.matches("/Count 2").count(), 1);
}

#[test]
fn the_y_axis_is_flipped_so_the_first_line_sits_higher() {
    let ys = tm_ys(&pdf_of(&doc_of(3)));
    assert_eq!(ys.len(), 3);
    assert!(ys[0] > ys[1], "first line {} must be above second {}", ys[0], ys[1]);
    assert!(ys[1] > ys[2]);
    let page = metrics();
    assert!(ys[0] < page.height - page.margin, "baseline must sit inside the top margin");
    assert!(ys[2] > 0.0, "last baseline must stay on the page");
}
