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

use super::fixture::{family, SUP};
use crate::foreign::seal::trace::{trace_step, Row, VALUE};

fn row(seq: u64) -> Row {
    Row { seq, pid: 7, what: VALUE, nr: 1, ret: 3 }
}

#[test]
fn a_step_is_blake3_of_the_head_and_the_row() {
    let k = [3u8; 32];
    let mut b = k.to_vec();
    b.extend_from_slice(&5u64.to_le_bytes());
    b.extend_from_slice(&7u32.to_le_bytes());
    b.extend_from_slice(&0u32.to_le_bytes());
    b.extend_from_slice(&1u64.to_le_bytes());
    b.extend_from_slice(&3u64.to_le_bytes());
    assert_eq!(trace_step(&k, &row(5)), *blake3::hash(&b).as_bytes());
}

#[test]
fn the_chain_keeps_its_order() {
    let k = [0u8; 32];
    let ab = trace_step(&trace_step(&k, &row(0)), &row(1));
    let ba = trace_step(&trace_step(&k, &row(1)), &row(0));
    assert_ne!(ab, ba);
}

#[test]
fn sequence_numbers_follow_trap_order() {
    let mut f = family(SUP);
    (1..=3).for_each(|pid| f.trapped(pid, 0));
    assert_eq!(f.open.iter().map(|c| c.1).collect::<Vec<_>>(), [0, 1, 2]);
}
