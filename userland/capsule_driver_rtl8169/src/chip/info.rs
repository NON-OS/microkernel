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

use super::MacVersion;

/// The chip found, with what identified it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Chip {
    pub ver: MacVersion,
    pub name: &'static str,
    /// The XID read from TxConfig, or the extended id from TX_CONFIG_V2 when
    /// `extended` is set.
    pub xid: u32,
    pub extended: bool,
}

impl Chip {
    pub const fn new(ver: u8, name: &'static str, xid: u32) -> Self {
        Self { ver: MacVersion(ver), name, xid, extended: false }
    }
}
