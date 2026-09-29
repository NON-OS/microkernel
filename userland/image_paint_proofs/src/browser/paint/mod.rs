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

#[path = "../../../../capsule_browser/src/browser/paint/grad/mod.rs"]
pub mod grad;
#[path = "../../../../capsule_browser/src/browser/paint/mask_weights.rs"]
mod mask_weights;

/// Shim for the mask proofs: `n` samples of mask coverage along screen
/// row `y` from column `x0`, for mask value `v` over a box at `o`.
pub fn mask_row(v: &str, o: [i32; 4], isect: bool, (y, x0): (i32, i32), n: usize) -> Vec<u32> {
    use crate::browser::layout::fade_table::{fade_id, fade_value};
    let kept = fade_value(fade_id(v)).expect("a gradient mask");
    let layers = grad::mask_layers(&kept, o[2], o[3]).expect("layers this painter draws");
    let (mut m, mut tmp) = (std::vec![0u32; n], std::vec![0u32; n]);
    mask_weights::weights(&layers, o, isect, (y, x0), &mut m, &mut tmp);
    m
}

/// Shim: old and new mixed by coverage m.
pub fn mix(old: u32, new: u32, m: u32) -> u32 {
    mask_weights::mix(old, new, m)
}
