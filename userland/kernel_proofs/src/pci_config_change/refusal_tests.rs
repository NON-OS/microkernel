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

use super::change::{merge, split};
use super::command_bits::{BUS_MASTER, MEMORY_SPACE};

#[test]
fn a_value_bit_outside_the_mask_is_refused_not_dropped() {
    assert_eq!(merge(BUS_MASTER, MEMORY_SPACE, 0), None);
    assert_eq!(merge(MEMORY_SPACE | BUS_MASTER, MEMORY_SPACE, 0), None);
    assert_eq!(merge(0, 0, 0x1234), Some(0x1234));
}

#[test]
fn offsets_past_config_space_are_refused_and_the_plain_form_reads_unchanged() {
    assert_eq!(split(0x04), Some((0x04, 0)));
    assert_eq!(split(0x0FFF), Some((0x0FFF, 0)));
    assert_eq!(split(0x1000), None);
    assert_eq!(split(0x8004), None);
    assert_eq!(split(0xFFFF_0004), Some((0x04, 0xFFFF)));
}
