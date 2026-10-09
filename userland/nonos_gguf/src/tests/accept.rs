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

//! Headers a real writer produces, accepted with what they say.

use super::build::Gguf;
use super::model::small_model;
use crate::{parse, DEFAULT};

#[test]
fn a_well_formed_model_is_accepted_with_its_layout_and_identity() {
    let file = small_model();
    let s = parse(&mut file.as_slice(), file.len() as u64, &DEFAULT).unwrap();
    assert_eq!((s.version, s.tensors, s.keys), (3, 2, 2));
    assert_eq!(s.meta.architecture.unwrap().as_bytes(), b"qwen2");
    assert!(s.meta.architecture.unwrap().is_whole());
    assert_eq!(s.meta.alignment, 32);
    assert_eq!(s.data_start % 32, 0);
    assert_eq!(s.data_bytes, 1152 + 2048);
    assert_eq!((s.by_type[12], s.by_type[0]), (1, 1));
}

#[test]
fn a_long_name_is_kept_in_part_and_marked_as_part() {
    let long = "x".repeat(200);
    let mut g = Gguf::header(3, 0, 1);
    g.kv_str("general.name", &long);
    let s = parse(&mut g.out.as_slice(), g.out.len() as u64, &DEFAULT).unwrap();
    let name = s.meta.name.unwrap();
    assert_eq!((name.as_bytes().len(), name.full_len, name.is_whole()), (64, 200, false));
}

#[test]
fn keys_of_every_type_are_stepped_over_including_string_arrays() {
    let mut g = Gguf::header(3, 0, 4);
    g.str("a.u8").u32(0).out.push(7);
    g.str("a.f64").u32(12).u64(0);
    g.str("tokenizer.ggml.tokens").u32(9).u32(8).u64(3).str("a").str("bb").str("");
    g.str("tokenizer.ggml.scores").u32(9).u32(6).u64(2).u32(0).u32(0);
    assert!(parse(&mut g.out.as_slice(), g.out.len() as u64, &DEFAULT).is_ok());
}
