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

//! The size `stat` gives a model under /models: a pinned file not brought
//! onto the volume yet is its pin's length, so qwenchat sizes a model before
//! its first open; every other answer of the volume stands.

use crate::models::pinned::pin_of;
use crate::stat_size::stat_size;

const ENOENT: i64 = 2;
const ENODEV: i64 = 19;
const EIO: i64 = 5;

#[test]
fn a_pinned_model_not_yet_imported_is_its_pinned_size() {
    for name in ["/qwen2.5-0.5b-instruct-q4_k_m.gguf", "/Qwen3-0.6B-Q8_0.gguf", "/Qwen3-4B-Q4_K_M.gguf"] {
        let pin = pin_of(name.as_bytes()).unwrap();
        assert_eq!(stat_size(Some(pin.bytes), -ENOENT), Ok(pin.bytes), "{name}");
    }
    let part = pin_of(b"/qwen2.5-coder-32b-q4_k_m-00002-of-00003.gguf").unwrap();
    assert_eq!(stat_size(Some(part.bytes), -ENOENT), Ok(7_943_826_304));
}

#[test]
fn what_the_volume_holds_and_its_other_refusals_stand() {
    assert_eq!(stat_size(Some(491_400_032), 491_400_032), Ok(491_400_032));
    assert_eq!(stat_size(Some(491_400_032), -ENODEV), Err(ENODEV));
    assert_eq!(stat_size(Some(491_400_032), -EIO), Err(EIO));
    assert_eq!(stat_size(None, -ENOENT), Err(ENOENT));
    assert_eq!(stat_size(None, 12), Ok(12));
}
