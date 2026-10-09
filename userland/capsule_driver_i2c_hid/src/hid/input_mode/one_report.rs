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

use super::command::{command, REPORT_MAX};
use super::fallback::from_descriptor;
use super::get::get;
use super::set::set;
use super::set_bits::set_bits;
use crate::hid::report_desc::Field;

// Read-modify-write one feature report: apply every target field that lives
// in it, write back only when a bit actually changed.
pub(super) fn configure_one_report(
    port: u32,
    addr: u8,
    desc: &[u8; 30],
    id: u8,
    targets: &[(Field, u8, u32); 3],
) -> bool {
    let Some(cmd) = command(desc, id) else { return false };
    let mut report = [0u8; REPORT_MAX];
    let mut report_len = get(port, addr, &cmd, id, &mut report);
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
    set(port, addr, &cmd, &report[..report_len])
}
