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
//! The factor each offered size maps to.

use crate::text_scale::{scale_of, ONE};
use nonos_policy_proto::font_size_labels::FONT_SIZE_LABELS;

const STORED_DEFAULT: u8 = 1;

#[test]
fn the_stored_default_leaves_text_exactly_as_it_is() {
    assert_eq!(scale_of(STORED_DEFAULT), ONE, "the default must not resize anything");
}
#[test]
fn every_offered_size_has_a_factor_of_its_own() {
    let last = FONT_SIZE_LABELS.len() as u8 - 1;
    for i in 2..=last {
        let label = core::str::from_utf8(FONT_SIZE_LABELS[i as usize]).unwrap_or("?");
        assert!(scale_of(i) > ONE, "{label} fell back to unchanged; the table is short");
    }
    assert!(scale_of(0) < ONE, "the smallest offered size must actually be smaller");
}
#[test]
fn the_sizes_increase_with_the_label_order() {
    let mut previous = 0u32;
    for i in 0..FONT_SIZE_LABELS.len() {
        let scale = scale_of(i as u8);
        assert!(scale > previous, "step {i} must be larger than the one before");
        previous = scale;
    }
}
#[test]
fn an_unknown_size_leaves_text_unchanged() {
    for steps in [FONT_SIZE_LABELS.len() as u8, 200, 255] {
        assert_eq!(scale_of(steps), ONE, "{steps} is not a size this build knows");
    }
}
