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

//! A document read back from a boot's serial log: the last one whole, and
//! nothing taken that is cut, padded, over a document's size or not hex.

use nsh_enrol::{doc_from_serial, Error};

fn log(lines: &[&str]) -> String {
    lines.iter().map(|l| format!("noise [ATTEST-DOC] {l}\r\n")).collect()
}

#[test]
fn the_last_whole_document_is_read() {
    let two = [log(&["begin 2", "0a0b", "end"]), log(&["begin 3", "010203", "end"])].concat();
    assert_eq!(doc_from_serial(&two), Ok(vec![1, 2, 3]));
    assert_eq!(
        doc_from_serial(&log(&["begin 4", "0a0b", "0c0d", "end "])),
        Ok(vec![10, 11, 12, 13])
    );
}

#[test]
fn a_cut_padded_oversized_or_unread_document_is_refused() {
    let cases: [(&[&str], &str); 6] = [
        (&["begin 3", "0a0b", "end"], "serial: document cut"),
        (&["begin 1", "0a0b", "end"], "serial: hex"),
        (&["begin 1", "0g", "end"], "serial: hex"),
        (&["begin 2", "0a0b"], "serial: no end line"),
        (&["begin 18446744073709551615", "end"], "serial: longer than a document"),
        (&["begin x", "end"], "serial: length"),
    ];
    for (lines, why) in cases {
        assert_eq!(doc_from_serial(&log(lines)), Err(Error::Value(why)), "{lines:?}");
    }
    assert_eq!(doc_from_serial("no document here"), Err(Error::Value("serial: no document")));
}
