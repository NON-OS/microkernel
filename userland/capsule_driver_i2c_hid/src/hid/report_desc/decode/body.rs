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

//! What every decode does first: find the report's body, and make sure it
//! holds whole each field the layout reads.

use crate::hid::report_desc::Field;

/// `report` is the input report after the two-byte length prefix. Its body
/// after the report ID byte when it is report `id`; the whole report when
/// the device numbers none (`id` 0).
pub(super) fn body(report: &[u8], id: u8) -> Option<&[u8]> {
    if id == 0 {
        return Some(report);
    }
    let (&first, body) = report.split_first()?;
    (first == id).then_some(body)
}

/// Whether `body` holds every present field whole. A torn or short polled
/// read is dropped whole: read_bits reads zeros past the end, which would
/// turn a cut coordinate into a jump toward the origin instead of nothing.
pub(crate) fn holds(body: &[u8], fields: &[Field]) -> bool {
    let bits = (body.len() as u64).saturating_mul(8);
    fields.iter().filter(|f| f.present()).all(|f| f.end().is_some_and(|e| u64::from(e) <= bits))
}
