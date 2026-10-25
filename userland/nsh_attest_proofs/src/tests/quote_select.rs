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

//! Step 3, the PCRs: the quote selects exactly PCRs 0, 1, 2, 4, 7, 8, 9
//! and 14 in the SHA-256 bank, and its digest is over the document's
//! values, every one of them, in that order.

use nsh_attest::Refusal;

use super::refused;
use crate::build::attest::Quote;
use crate::build::Spec;

fn select(f: fn(&mut Quote)) -> Refusal {
    refused(Spec { quote: f, ..Spec::default() })
}

#[test]
fn the_version_4_bitmap_is_exact() {
    assert_eq!(nsh_attest::quote::bitmap(&nsh_attest::consts::QUOTED_PCRS_V4), [0x97, 0x43, 0]);
}

#[test]
fn a_quote_over_the_version_3_pcrs_is_refused() {
    assert_eq!(select(|q| q.selections[0].1 = vec![0x87, 0, 0]), Refusal::Selection);
}

#[test]
fn a_selection_with_one_pcr_more_or_less_is_refused() {
    assert_eq!(select(|q| q.selections[0].1 = vec![0x97, 0x43, 0x01]), Refusal::Selection);
    assert_eq!(select(|q| q.selections[0].1 = vec![0x97, 0x03, 0]), Refusal::Selection);
}

#[test]
fn a_selection_in_another_bank_or_shape_is_refused() {
    assert_eq!(select(|q| q.selections[0].0 = 0x0004), Refusal::Selection);
    assert_eq!(select(|q| q.selections[0].1.push(0)), Refusal::Selection);
    assert_eq!(
        select(|q| {
            q.selections[0].1.pop();
        }),
        Refusal::Selection
    );
    let two = |q: &mut Quote| q.selections.push((0x0004, vec![0x97, 0x43, 0]));
    assert_eq!(select(two), Refusal::Selection);
}
