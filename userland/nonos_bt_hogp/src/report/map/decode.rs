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

//! An input report into the keys it says are held (HID 1.11, 6.2.2.5): a
//! variable field's bit set holds its usage; an array element holds the
//! usage at its index, counted from the Logical Minimum, and a value
//! outside the logical extent holds none.

use super::bits;
use super::field::Field;
use super::keys::{key_of, phantom, readable};
use super::layout::Layout;
use super::span::Dir;
use alloc::vec::Vec;

/// Keys one report holds at most; more is no hand on a keyboard.
pub const HELD_MAX: usize = 32;

fn usage_at(f: &Field, i: u32, value: &[u8]) -> Option<Option<(u16, u16)>> {
    if !f.array {
        let raw = bits::read(value, f.bit + i * f.size, f.size)?;
        return Some(if raw != 0 { f.usage_of(i) } else { None });
    }
    let Some(v) = bits::element(f, i, value)? else { return Some(None) };
    Some(u32::try_from(v - i64::from(f.lmin)).ok().and_then(|n| f.usage(n)))
}

/// The keys held, in report order, or `None` when the report cannot say:
/// shorter than the map lays out, a phantom state, or too many keys.
/// Octets past the laid-out length are the device's own and not read.
pub fn held(layout: &Layout, id: u8, value: &[u8]) -> Option<Vec<u8>> {
    let value = value.get(..layout.octets(Dir::Input, id)?)?;
    let mut keys = Vec::new();
    for f in layout.fields(Dir::Input, id).filter(|f| readable(f)) {
        for i in 0..f.count {
            let Some(usage) = usage_at(f, i, value)? else { continue };
            if phantom(usage) {
                return None;
            }
            match key_of(usage) {
                Some(k) if keys.contains(&k) => {}
                Some(_) if keys.len() == HELD_MAX => return None,
                Some(k) => keys.push(k),
                None => {}
            }
        }
    }
    Some(keys)
}
