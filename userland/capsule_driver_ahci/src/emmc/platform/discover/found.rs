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

//! One host discovery found.

use nonos_libc::Bar;

use super::super::super::pci::Kind;

#[derive(Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub vendor: u16,
    pub device: u16,
    pub kind: Kind,
    pub bars: [Bar; 6],
    pub bar_count: u8,
}
