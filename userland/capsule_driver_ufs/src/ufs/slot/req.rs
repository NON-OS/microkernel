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

//! A request as the protocol layers hand it to the slot.

use super::descriptor::Dir;

/// What to send: the UTRD command type, the data direction, the UPIU, how
/// many bytes of the data region the PRDT hands the host, and the bound.
pub struct Req<'a> {
    pub cmd_type: u32,
    pub dir: Dir,
    pub upiu: &'a [u8],
    pub data_len: usize,
    pub ms: u64,
}
