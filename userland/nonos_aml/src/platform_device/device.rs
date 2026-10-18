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

//! What the walk finds: the devices to publish, and the ones it passed over
//! with why, so the capsule's log names every listed device it saw.

use super::known::Class;
use crate::legacy::Range;
use crate::namespace::Id;
use crate::platform::Interrupt;
use alloc::vec::Vec;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlatformDevice {
    pub id: Id,
    pub class: Class,
    /// The id that matched, which may be a `_CID`.
    pub matched: Vec<u8>,
    pub uid: u64,
    pub windows: Vec<Range>,
    /// The first interrupt the template names: these controllers have one.
    pub irq: Option<Interrupt>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Skip {
    /// `_CRS` failed or gave no buffer.
    NoTemplate,
    /// No memory window a driver could map.
    NoWindow,
}

#[derive(Default)]
pub struct Found {
    pub devices: Vec<PlatformDevice>,
    pub skipped: Vec<(Id, Skip)>,
}
