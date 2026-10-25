// NONOS Operating System (AGPL-3.0-or-later)

//! A document of n lines, its PDF, and what to read out of it.

use capsule_text_editor_layout_tests::doc::block::Block;
use capsule_text_editor_layout_tests::doc::document::Doc;
use capsule_text_editor_layout_tests::doc::export::pdf::to_pdf;
use capsule_text_editor_layout_tests::doc::kind::BlockKind;
use capsule_text_editor_layout_tests::doc::measure::FixedMeasurer;
use capsule_text_editor_layout_tests::doc::page::PageMetrics;
use capsule_text_editor_layout_tests::doc::style::RunStyle;

pub(super) fn metrics() -> PageMetrics {
    PageMetrics { width: 816.0, height: 200.0, margin: 48.0 }
}

pub(super) fn doc_of(n: usize) -> Doc {
    let mut d = Doc::new();
    for _ in 0..n {
        d.blocks.push(Block::plain(BlockKind::Paragraph, "line", RunStyle::body()));
    }
    d
}

pub(super) fn pdf_of(doc: &Doc) -> String {
    String::from_utf8_lossy(&to_pdf(doc, &metrics(), &FixedMeasurer)).into_owned()
}

pub(super) fn find(hay: &[u8], needle: &[u8]) -> usize {
    hay.windows(needle.len()).position(|w| w == needle).expect("needle not found")
}

pub(super) fn tm_ys(src: &str) -> Vec<f32> {
    src.lines()
        .filter(|l| l.ends_with(" Tm"))
        .map(|l| l.split(' ').nth(5).unwrap().parse::<f32>().unwrap())
        .collect()
}
