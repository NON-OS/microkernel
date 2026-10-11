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

//! Which guest paths reach the data volume, and as which name.

use crate::model_name::{owns, volume_name};

#[test]
fn a_file_under_models_is_the_same_name_on_the_volume() {
    let p = b"/models/qwen2.5-0.5b-instruct-q4_k_m.gguf";
    assert_eq!(volume_name(p), Some(&b"/qwen2.5-0.5b-instruct-q4_k_m.gguf"[..]));
    assert!(owns(b"/models") && owns(b"/models/x"));
    assert!(!owns(b"/modelsx") && !owns(b"/model") && !owns(b"/linux/models"));
}

#[test]
fn no_guest_path_names_anything_but_one_flat_file() {
    for bad in [
        &b"/models"[..],
        b"/models/",
        b"/models/a/b",
        b"/models/..",
        b"/models/.hidden",
        b"/models/a b",
        b"/models/a\0b",
        b"/models/\xc3\xa9",
    ] {
        assert_eq!(volume_name(bad), None, "{:?}", core::str::from_utf8(bad));
    }
    let long = [b'a'; 64];
    let mut p = b"/models/".to_vec();
    p.extend_from_slice(&long);
    assert_eq!(volume_name(&p), None);
    assert!(volume_name(&p[..p.len() - 1]).is_some());
}
