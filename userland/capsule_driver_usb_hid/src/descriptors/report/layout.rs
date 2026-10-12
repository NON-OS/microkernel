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

//! What a report descriptor says about the reports this driver reads: the
//! first keyboard, media key and pointer collection it declares. Field
//! positions are in bits from the start of a report's body, after its
//! report ID byte when the interface uses report IDs.

use super::keys::Keys;
use super::media::Media;
use super::pointer::Pointer;

/// No field there.
pub const NONE: u16 = u16::MAX;

/// One variable field of 1 to 32 bits: where it sits, its logical range,
/// and whether it reports a change rather than a position.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Field {
    pub at: u16,
    pub size: u8,
    pub min: i64,
    pub max: i64,
    pub relative: bool,
}

/// The slots of an array item, 1 to 6 of 1 to 16 bits: each holds a value
/// from `min` to `max` naming a usage, or one outside it naming none.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slots {
    pub at: u16,
    pub size: u8,
    pub count: u8,
    pub min: i64,
    pub max: i64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Layout {
    /// Every report starts with its report ID byte.
    pub ids: bool,
    /// The longest input report in bytes, report ID byte included.
    pub longest: u16,
    pub keys: Option<Keys>,
    /// Consumer control keys, then system control keys.
    pub media: [Option<Media>; 2],
    pub pointer: Option<Pointer>,
}

impl Layout {
    /// Whether the interface is driven by this layout: it names a report
    /// this driver reads, and every input report the interface can send
    /// fits one read of `read_max` bytes, so none is cut short or babbles.
    pub fn bindable(&self, read_max: u16) -> bool {
        let media = self.media.iter().any(Option::is_some);
        let any = self.keys.is_some() || media || self.pointer.is_some();
        any && self.longest <= read_max
    }
}
