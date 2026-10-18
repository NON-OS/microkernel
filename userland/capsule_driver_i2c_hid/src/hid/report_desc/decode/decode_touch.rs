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
use super::body::{body, holds};
use super::position::position;
use super::types::TouchSample;
use crate::hid::report_desc::layout::TouchLayout;
use crate::hid::report_desc::read_bits::read_bits;
use crate::hid::report_desc::Field;

/// `report` is the input report after the two-byte i2c-hid length prefix (what
/// the poll loop already slices out). None when it is another report than
/// the touch layout's, or too short to hold every field the layout reads.
pub fn decode_touch(report: &[u8], layout: &TouchLayout) -> Option<TouchSample> {
    let l = layout;
    let body = body(report, l.report_id)?;
    let fields = [l.x, l.y, l.tip, l.contact_count, l.contact_id, l.button, l.confidence];
    if !holds(body, &fields) {
        return None;
    }
    let value = |f: &Field| read_bits(body, f.bit_offset, f.bit_size);
    let (x, x_max) = position(body, &l.x);
    let (y, y_max) = position(body, &l.y);
    let tip = value(&l.tip) != 0;
    let contacts = if l.contact_count.present() { value(&l.contact_count) } else { u32::from(tip) };
    Some(TouchSample {
        x,
        y,
        x_max,
        y_max,
        tip,
        contacts,
        contact_id: l.contact_id.present().then(|| value(&l.contact_id)),
        button: value(&l.button) != 0,
        confidence: !l.confidence.present() || value(&l.confidence) != 0,
    })
}
