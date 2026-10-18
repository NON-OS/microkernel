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

//! One feature report: read, changed where a target field is wrong, and
//! written back only when a bit changed.

use super::configure::MODE;
use super::configured::Configured;
use super::fallback::from_descriptor;
use super::port::{FeaturePort, REPORT_MAX};
use super::set_bits::set_bits;
use crate::hid::Field;

// Read-modify-write one feature report: apply every target field that lives
// in it, write back only when a bit actually changed, and say in `c` what
// its Input Mode field read back as and whether the write took. None when
// there is nothing to write (no command register, a report too long to
// build).
pub(super) fn configure_one_report(
    port: &impl FeaturePort,
    id: u8,
    targets: &[(Field, u8, u32); 3],
    c: &mut Configured,
) -> Option<()> {
    if !port.reaches(id) {
        return None;
    }
    let mut report = [0u8; REPORT_MAX];
    let mut report_len = port.get(id, &mut report);
    let read_back = report_len >= 2;
    let mut changed = !read_back;
    if !read_back {
        report_len = from_descriptor(id, targets, &mut report)?;
    }
    for (i, (field, field_id, value)) in targets.iter().enumerate() {
        if *field_id == id && field.present() {
            let body = &mut report[1..report_len];
            let flipped = set_bits(body, field.bit_offset, field.bit_size, *value);
            changed |= flipped;
            c.mode_lost |= i == MODE && read_back && flipped;
            c.mode_unread |= i == MODE && !read_back;
        }
    }
    c.refused |= changed && !port.set(&report[..report_len]);
    Some(())
}
