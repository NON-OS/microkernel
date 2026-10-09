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

//! Each pinned Qwen tier and the summed length of its files, written by
//! build.rs from the Linux personality's pin tables. The check below stops
//! the build when it and the fetcher's memory table (`need.rs`) name
//! different tiers, so no tier's card is left without a size.

use crate::need::SHAPES;

include!(concat!(env!("OUT_DIR"), "/model_weights.rs"));

const fn same(a: &str, b: &str) -> bool {
    let (a, b) = (a.as_bytes(), b.as_bytes());
    if a.len() != b.len() {
        return false;
    }
    let mut i = 0;
    while i < a.len() {
        if a[i] != b[i] {
            return false;
        }
        i += 1;
    }
    true
}

const fn agree() -> bool {
    if MODEL_WEIGHTS.len() != SHAPES.len() {
        return false;
    }
    let mut i = 0;
    while i < MODEL_WEIGHTS.len() {
        let mut found = false;
        let mut j = 0;
        while j < SHAPES.len() {
            found = found || same(MODEL_WEIGHTS[i].0, SHAPES[j].0);
            j += 1;
        }
        if !found || MODEL_WEIGHTS[i].1 == 0 {
            return false;
        }
        i += 1;
    }
    true
}

const _: () = assert!(agree(), "the pinned tiers and need.rs's shapes name different tiers");
