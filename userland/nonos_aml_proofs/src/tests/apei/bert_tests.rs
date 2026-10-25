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

//! A HEST that claims more than it holds, and BERT.

use super::build::{ghes, header};
use crate::apei::*;

#[test]
fn a_count_past_the_table_and_bert() {
    let t =
        header(b"HEST", &[9u32.to_le_bytes().to_vec(), ghes(1, 0x1000, POLLED, 50, None)].concat());
    assert_eq!(hest(&t).len(), 1, "the count says 9; the table holds one");
    let mut cut = t.clone();
    cut.truncate(cut.len() - 1);
    assert!(hest(&cut).is_empty());
    let b = header(
        b"BERT",
        &[0x200u32.to_le_bytes().to_vec(), 0x7FE0_0000u64.to_le_bytes().to_vec()].concat(),
    );
    assert_eq!(bert(&b), Some((0x7FE0_0000, 0x200)));
    let empty = header(b"BERT", &[0u8; 12]);
    assert_eq!(bert(&empty), None);
}
