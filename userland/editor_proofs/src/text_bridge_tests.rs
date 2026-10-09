// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Proofs for the plain-text side of the document view: a file is read into
//! one block per line with headings and page breaks recognised, the line scan
//! a list edit walks stays inside the range and the buffer it is given, and a
//! page with no font face still lays out on the fixed advances.

use crate::doc::block::Block;
use crate::doc::document::Doc;
use crate::doc::kind::BlockKind;
use crate::doc::list::scan::line_starts;
use crate::doc::measure::{FixedMeasurer, Measurer};
use crate::doc::page::PageMetrics;
use crate::doc::paginate::paginate;
use crate::doc::style::RunStyle;
use crate::doc::text_bridge::doc_from_text;

#[test]
fn headings_paragraphs_and_page_breaks_are_read_line_by_line() {
    let src = b"# Title\nbody text\n\x0c\n###### Six\n####### seven is prose\n#no space\n- item";
    let d = doc_from_text(src);
    let got: Vec<(BlockKind, &str)> = d.blocks.iter().map(|b| (b.kind, b.as_str())).collect();
    assert_eq!(
        got,
        [
            (BlockKind::Heading(1), "Title"),
            (BlockKind::Paragraph, "body text"),
            (BlockKind::PageBreak, ""),
            (BlockKind::Heading(6), "Six"),
            (BlockKind::Paragraph, "####### seven is prose"),
            (BlockKind::Paragraph, "#no space"),
            (BlockKind::Paragraph, "- item"),
        ]
    );
    for b in &d.blocks {
        let styled: usize = b.runs.iter().map(|r| r.len).sum();
        assert_eq!(styled, b.text.len(), "every byte of {:?} carries a style", b.as_str());
    }
}

#[test]
fn a_page_with_no_font_face_still_wraps_on_the_fixed_advances() {
    // TtfMeasurer answers with FixedMeasurer when the built-in face is
    // missing; a zero advance would put a whole paragraph on one line.
    let src = include_str!("../../capsule_text_editor/src/doc/ttf_measure.rs");
    for arm in [
        "None => FixedMeasurer.advance(text, style)",
        "None => FixedMeasurer.line_height(style)",
        "None => FixedMeasurer.ascent(style)",
    ] {
        assert!(src.contains(arm), "TtfMeasurer falls back with `{arm}`");
    }
    let body = RunStyle::body();
    assert!(FixedMeasurer.advance("a", &body) > 0.0);
    let mut d = Doc::new();
    let long = "word ".repeat(200);
    d.blocks.push(Block::plain(BlockKind::Paragraph, &long, body));
    let pm = PageMetrics { width: 760.0, height: 980.0, margin: 56.0 };
    let lines: Vec<_> =
        paginate(&d, &pm, &FixedMeasurer).into_iter().flat_map(|p| p.lines).collect();
    assert!(lines.len() > 1, "a long paragraph wraps");
    for l in &lines {
        assert!(l.width <= pm.content_width() + 0.5, "line of {} px overruns", l.width);
    }
}

#[test]
fn line_starts_stay_inside_the_range_and_the_buffer() {
    let buf = b"a\nb\nc";
    assert_eq!(line_starts(buf, 0, buf.len()), [0, 2, 4]);
    assert_eq!(line_starts(buf, 2, 100), [2, 4], "an end past the buffer stops at it");
    assert_eq!(line_starts(buf, 0, 2), [0, 2], "a newline just inside the end counts");
    assert_eq!(line_starts(buf, 0, 1), [0], "one just past it does not");
    assert_eq!(line_starts(buf, 9, 3), [5], "a start past the buffer is its end");
}
