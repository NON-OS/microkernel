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

//! The ribbon's formatting outlives the next keystroke and reaches Export.
//! It was written into the document model alone, which every edit rebuilds
//! from the text, so bold vanished as soon as anything was typed and no
//! exported file ever carried it.

use crate::doc::align::Align;
use crate::doc::export::md::to_markdown;
use crate::edit_tests::doc;
use crate::mode::Mode;
use crate::state::State;
use crate::style_marks::Mark;

fn page(text: &str) -> State {
    let mut s = doc(text);
    s.mode = Mode::Document;
    s.reset_styles();
    s.reflow();
    s
}

fn bold(m: &mut Mark) {
    m.bold = Some(true);
}

#[test]
fn bold_survives_typing_elsewhere() {
    let mut s = page("hello world");
    assert!(s.mark_range(0, 5, &bold));
    s.caret = s.len;
    assert!(s.insert(b"!"));
    let block = &s.doc.blocks[0];
    assert!(block.style_at(0).bold && block.style_at(4).bold);
    assert!(!block.style_at(6).bold);
}

#[test]
fn text_typed_inside_bold_is_bold() {
    let mut s = page("hello world");
    s.mark_range(0, 5, &bold);
    s.caret = 3;
    assert!(s.insert(b"XY"));
    assert!(s.doc.blocks[0].style_at(4).bold, "the Y typed inside the bold run");
    assert!(!s.doc.blocks[0].style_at(8).bold);
}

#[test]
fn bold_goes_out_with_the_markdown_export() {
    let mut s = page("hello world");
    s.mark_range(0, 5, &bold);
    s.caret = s.len;
    s.insert(b".");
    s.reflow();
    assert_eq!(to_markdown(&s.doc).trim_end(), "**hello** world.");
}

#[test]
fn deleting_formatted_text_takes_its_marks_with_it() {
    let mut s = page("abc def");
    s.mark_range(4, 7, &bold);
    assert!(s.apply_edit(0, 4, b""));
    assert_eq!(s.marks.len(), s.len);
    assert!(s.doc.blocks[0].style_at(0).bold, "def moved to the front, still bold");
}

/// Alignment was copied block by block by index, so a line inserted above a
/// centred paragraph moved the centring onto the paragraph before it.
#[test]
fn alignment_stays_with_its_paragraph_when_lines_go_in_above() {
    let mut s = page("a\nb\nc");
    s.align_lines(4, 4, Align::Center);
    assert!(s.apply_edit(0, 0, b"x\n"));
    let aligns: Vec<Align> = s.doc.blocks.iter().map(|b| b.align).collect();
    assert_eq!(aligns, vec![Align::Left, Align::Left, Align::Left, Align::Center]);
    // And joining a line onto the one above drops the line's own entry.
    assert!(s.apply_edit(1, 1, b""));
    assert_eq!(s.doc.blocks.len(), 3);
    assert_eq!(s.doc.blocks[2].align, Align::Center);
}

#[test]
fn a_heading_keeps_its_size_under_a_mark_that_does_not_set_one() {
    let mut s = page("# Title");
    s.mark_range(2, 7, &|m: &mut Mark| m.italic = Some(true));
    let run = s.doc.blocks[0].style_at(0);
    assert!(run.italic);
    assert_eq!(run.size_px, 34.0);
}

#[test]
fn the_document_knows_when_save_would_leave_formatting_out() {
    let mut s = page("plain");
    assert!(!s.has_formatting());
    s.mark_range(0, 5, &bold);
    assert!(s.has_formatting());
}

#[test]
fn a_new_text_starts_without_the_last_ones_formatting() {
    let mut s = page("hello");
    s.mark_range(0, 5, &bold);
    s.buf[..3].copy_from_slice(b"new");
    s.len = 3;
    s.reset_styles();
    s.reflow();
    assert!(!s.has_formatting());
    assert!(!s.doc.blocks[0].style_at(0).bold);
}
