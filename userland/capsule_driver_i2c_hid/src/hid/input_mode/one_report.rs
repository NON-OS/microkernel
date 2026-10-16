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

use super::fallback::from_descriptor;
use super::port::{FeaturePort, REPORT_MAX};
use super::set_bits::set_bits;
use crate::hid::Field;

// Read-modify-write one feature report: apply every target field that lives
// in it, write back only when a bit actually changed.
pub(super) fn configure_one_report(
    port: &impl FeaturePort,
    id: u8,
    targets: &[(Field, u8, u32); 3],
) -> bool {
    let mut report = [0u8; REPORT_MAX];
    let mut report_len = port.get(id, &mut report);
    let mut changed = false;
    if report_len < 2 {
        let Some(len) = from_descriptor(id, targets, &mut report) else { return false };
        report_len = len;
        changed = true;
    }
    for (field, field_id, value) in targets.iter() {
        if *field_id == id && field.present() {
            changed |=
                set_bits(&mut report[1..report_len], field.bit_offset, field.bit_size, *value);
        }
    }
    if !changed {
        return true;
    }
    port.set(&report[..report_len])
}
