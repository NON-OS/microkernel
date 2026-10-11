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

use super::kinds::every;
use crate::intent::{hex, id, review_rows};

fn nox(v: u128) -> String {
    format!("{v}")
}

#[test]
fn every_kind_reads_as_an_action() {
    for p in every() {
        let rows = review_rows(&p, nox);
        assert_eq!(rows[0].0, "action", "{p:?}");
        assert!(rows.len() >= 2 && rows.iter().all(|(_, v)| !v.is_empty()), "{p:?}");
    }
    let approve = review_rows(&every()[0], nox);
    assert_eq!(approve[1], ("taker", "NONOS Bond".into()));
    assert_eq!(approve[2].1, "5 NOX, exactly this, never unlimited");
    let publish = review_rows(&every()[5], nox);
    assert_eq!(publish.last().unwrap().1, "https://example.org/r/qwen-small@1");
}

#[test]
fn hex_as_a_person_reads_it() {
    assert_eq!(hex(&[0xab, 0x01]), "0xab01");
    assert_eq!(hex(&[0x0f; 20]).len(), 42);
    let mut w = [0x11u8; 32];
    w[31] = 0xfe;
    assert_eq!(id(&w), "0x11111111...1111fe");
}
