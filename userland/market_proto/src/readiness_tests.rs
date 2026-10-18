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

//! OP_INSTALL_READY's reply is exactly READINESS_LEN bytes: one short or
//! one past is no readiness.

use alloc::vec::Vec;

use super::{parse_readiness, BondCode, READINESS_LEN};

fn row() -> Vec<u8> {
    let mut r = alloc::vec![1u8; READINESS_LEN];
    if let Some(code) = r.last_mut() {
        *code = BondCode::Active as u8;
    }
    r
}

#[test]
fn a_readiness_with_bytes_past_its_bond_code_is_refused() {
    assert!(parse_readiness(&row()).is_some(), "exactly READINESS_LEN reads");
    for extra in [&[0u8][..], &[1], &[2, 3], &[0; 64]] {
        let long = [row().as_slice(), extra].concat();
        assert_eq!(parse_readiness(&long), None, "{} bytes past", extra.len());
    }
}

#[test]
fn a_readiness_one_byte_short_is_refused() {
    let r = row();
    let short = r.get(..READINESS_LEN.saturating_sub(1)).unwrap_or_default();
    assert_eq!(parse_readiness(short), None);
}
