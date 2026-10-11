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

//! A disk the driver serves: where it sits and what IDENTIFY DEVICE said.

mod identity;

pub use identity::{parse, Flush, Identity};

#[derive(Clone, Copy)]
pub struct Disk {
    pub channel: usize,
    pub drive: u8,
    pub id: Identity,
}

/// The wire's unit for a slot: the primary master 0, its slave 1, the
/// secondary master 2, its slave 3.
pub const fn unit_of(channel: usize, drive: u8) -> usize {
    channel * 2 + drive as usize
}

pub const UNITS: usize = 4;
