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

//! When the device refuses GET_REPORT, construct the report from the
//! descriptor instead: every field whose position we parsed is set to its
//! correct value, and the remainder is zero (latency/off-state defaults).
//! This is descriptor-accurate, not a blind guess, and it is the only way
//! to reconfigure a pad whose firmware does not answer GET_REPORT.

use super::command::REPORT_MAX;
use crate::hid::report_desc::Field;

/// The length of the zeroed report `id` needs to hold every target field
/// in it, id byte included; None when no field lives there or it is too long.
pub(super) fn from_descriptor(
    id: u8,
    targets: &[(Field, u8, u32); 3],
    report: &mut [u8; REPORT_MAX],
) -> Option<usize> {
    let mut need_bits = 0u32;
    for (field, field_id, _) in targets.iter() {
        if *field_id == id && field.present() {
            need_bits = need_bits.max(field.bit_offset + field.bit_size);
        }
    }
    if need_bits == 0 {
        return None;
    }
    let body_len = (need_bits as usize).div_ceil(8);
    if 1 + body_len > REPORT_MAX {
        return None;
    }
    *report = [0u8; REPORT_MAX];
    report[0] = id;
    Some(1 + body_len)
}
