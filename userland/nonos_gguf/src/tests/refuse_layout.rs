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

//! Where tensor data may lie, broken once each and refused by name.

use super::build::Gguf;
use super::model::small_model;
use crate::{parse, GgufError as E, TensorError as T, DEFAULT};

fn two(first: u64, second: u64, data: usize) -> Result<crate::Summary, E> {
    let mut g = Gguf::header(3, 2, 0);
    g.tensor("a", &[64], 0, first).tensor("b", &[64], 0, second).data(32, data);
    parse(&mut g.out.as_slice(), g.out.len() as u64, &DEFAULT)
}

#[test]
fn tensor_data_must_be_aligned_inside_the_file_and_unshared() {
    assert!(two(0, 256, 512).is_ok());
    assert_eq!(two(0, 250, 512), Err(E::Tensor { tensor: 1, why: T::Misaligned(250) }));
    assert_eq!(two(0, 320, 512), Err(E::Tensor { tensor: 1, why: T::PastEnd }));
    assert_eq!(two(0, 128, 512), Err(E::Tensor { tensor: 1, why: T::Overlap }));
    assert_eq!(two(256, 0, 512), Ok(two(256, 0, 512).unwrap()));
    /* An offset whose sum with the data start overflows u64 is past the end. */
    assert_eq!(two(0, u64::MAX & !31, 512), Err(E::Tensor { tensor: 1, why: T::PastEnd }));
}

#[test]
fn a_file_cut_short_anywhere_is_refused_and_never_read_past_its_length() {
    let file = small_model();
    for len in 0..file.len() {
        let got = parse(&mut &file[..len], len as u64, &DEFAULT);
        assert!(got.is_err(), "accepted at {len} of {}", file.len());
    }
}
