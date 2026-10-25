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
//! A whole receipt verifies, and says what it carries.
//! The writer's clear, by layout alone (napr/doc_clear.rs), gives the bytes
//! the verifier's reader (napr/doc_form.rs) does, for docs with and without
//! a log, and nothing for a doc that is not v5.

use agentproof_receipt::napr::doc_clear::{clear, LOG_AT, PCRS};
use agentproof_receipt::napr::doc_form::receipt_doc;
use nsh_attest::consts::PCR_COUNT;

use crate::fixture::Fixture;

#[test]
fn the_layout_clear_is_the_readers_clear() {
    assert_eq!(PCRS, PCR_COUNT, "the writer's count is nsh_attest's");
    let mut f = Fixture::new(b"strict");
    for log in [0usize, 1, 37, 4096] {
        f.spec.log.bytes = vec![0x5a; log];
        let doc = f.spec.doc_bytes();
        assert_eq!(clear(&doc), receipt_doc(&doc), "a log of {log} bytes");
        assert!(clear(&doc).is_some());
    }
    let doc = f.spec.doc_bytes();
    assert!(clear(&doc[..LOG_AT + 2]).is_none(), "a cut log length");
    let mut v4 = doc.clone();
    v4[11] = 4;
    assert!(clear(&v4).is_none(), "another version");
}
