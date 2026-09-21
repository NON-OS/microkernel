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
//! What the layout tolerates at either end of the range.

use crate::text_scale::{scale_of, ONE};
use nonos_policy_proto::font_size_labels::FONT_SIZE_LABELS;

const STORED_DEFAULT: u8 = 1;

#[test]
fn the_steps_are_a_usable_size_apart() {
    for i in 1..FONT_SIZE_LABELS.len() {
        let previous = scale_of(i as u8 - 1) as f64;
        let scale = scale_of(i as u8) as f64;
        let ratio = scale / previous;
        assert!(ratio > 1.07, "step {i} is too small to notice: {ratio:.3}");
        assert!(ratio < 1.25, "step {i} is too large a jump: {ratio:.3}");
    }
}
#[test]
fn the_range_stays_inside_what_the_layout_tolerates() {
    let smallest = scale_of(0) as f64 / ONE as f64;
    let largest = scale_of(FONT_SIZE_LABELS.len() as u8 - 1) as f64 / ONE as f64;
    assert!(smallest >= 0.85, "the smallest is {smallest:.3}");
    assert!(largest <= 1.50, "the largest is {largest:.3}");
}
