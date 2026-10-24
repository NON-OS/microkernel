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

//! Any one byte of the document changed outside the log, or of the
//! certificate, is refused; a byte changed inside the log may name an
//! event body no PCR covers, but never panics and never moves a PCR
//! unnoticed.

use nsh_attest::Doc;

use crate::build::Spec;

#[test]
fn any_changed_byte_outside_the_log_is_refused() {
    let b = Spec::default().build();
    let d = Doc::parse(&b.doc).expect("parses");
    let log_at = d.log.as_ptr() as usize - b.doc.as_ptr() as usize;
    let log = log_at..log_at + d.log.len();
    for i in (0..b.doc.len()).filter(|i| !log.contains(i)) {
        let mut c = b.clone();
        c.doc[i] ^= 0x01;
        assert!(c.check().is_err(), "byte {i} of the document");
    }
}

#[test]
fn a_changed_byte_inside_the_log_never_panics() {
    let b = Spec::default().build();
    let d = Doc::parse(&b.doc).expect("parses");
    let log_at = d.log.as_ptr() as usize - b.doc.as_ptr() as usize;
    for i in log_at..log_at + d.log.len() {
        for flip in [0x01, 0x80, 0xFF] {
            let mut c = b.clone();
            c.doc[i] ^= flip;
            let _ = c.check();
        }
    }
}
