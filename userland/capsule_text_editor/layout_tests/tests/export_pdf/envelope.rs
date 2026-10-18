// NONOS Operating System (AGPL-3.0-or-later)

//! The PDF's envelope: its header, its xref and its trailer.

use capsule_text_editor_layout_tests::doc::export::pdf::to_pdf;
use capsule_text_editor_layout_tests::doc::measure::FixedMeasurer;

use super::pdf::{doc_of, find, metrics, pdf_of};

#[test]
fn output_is_a_pdf_envelope() {
    let out = to_pdf(&doc_of(2), &metrics(), &FixedMeasurer);
    assert!(out.starts_with(b"%PDF-"), "missing header");
    assert!(out.ends_with(b"%%EOF"), "missing trailer marker");
}

#[test]
fn startxref_points_at_the_xref_keyword() {
    let out = to_pdf(&doc_of(2), &metrics(), &FixedMeasurer);
    let at = find(&out, b"startxref\n") + 10;
    let digits = &out[at..out.len() - 6];
    let off: usize = core::str::from_utf8(digits).unwrap().parse().unwrap();
    assert_eq!(&out[off..off + 5], b"xref\n");
}

#[test]
fn size_counts_every_object_plus_the_free_head() {
    let src = pdf_of(&doc_of(2));
    let declared: usize =
        src.split("/Size ").nth(1).unwrap().split(' ').next().unwrap().parse().unwrap();
    let objects = src.matches(" 0 obj\n").count();
    assert_eq!(declared, objects + 1);
    assert_eq!(src.matches(" 00000 n \n").count(), objects);
}
