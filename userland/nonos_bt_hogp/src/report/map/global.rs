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

//! The global items (HID 1.11, 6.2.2.7): state that holds from one main
//! item to the next until changed, with Push and Pop to save and restore
//! it.

use super::error::MapError;
use super::item::Item;

/// The longest field element this host reads, in bits.
pub const SIZE_MAX: u32 = 32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Globals {
    pub page: u16,
    pub lmin: i32,
    /// Logical Maximum as written, with its size: whether it is signed
    /// depends on the minimum, which may come after it.
    pub lmax: (u32, u8),
    pub size: u32,
    pub id: u8,
    pub count: u32,
}

impl Globals {
    /// One global item other than Push and Pop. Physical extents and units
    /// do not change which keys are held, so they are passed over.
    pub fn set(&mut self, it: &Item) -> Result<(), MapError> {
        match it.tag {
            0 => self.page = u16::try_from(it.data).map_err(|_| MapError::Value)?,
            1 => self.lmin = it.signed(),
            2 => self.lmax = (it.data, it.size),
            3..=6 => {}
            7 if it.data <= SIZE_MAX => self.size = it.data,
            // Report ID 0 is reserved (HID 1.11, 6.2.2.7).
            8 if (1..=255).contains(&it.data) => self.id = it.data as u8,
            9 => self.count = it.data,
            7..=9 => return Err(MapError::Value),
            _ => return Err(MapError::Unknown),
        }
        Ok(())
    }

    /// Logical Maximum: signed, unless the minimum is not negative and the
    /// signed reading would fall below it, which is how a maximum such as
    /// 255 in one octet is meant.
    pub fn max(&self) -> i64 {
        let (data, size) = self.lmax;
        let it = Item { kind: super::item::Type::Global, tag: 2, size, data };
        let signed = i64::from(it.signed());
        if self.lmin >= 0 && signed < i64::from(self.lmin) {
            i64::from(data)
        } else {
            signed
        }
    }
}
