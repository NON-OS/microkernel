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

//! The two feature reads bring-up makes: whether the controller does LE at
//! all, and which LE features it has.

use crate::error::HciError;
use nonos_bt_wire::cursor::Reader;

/// LE Supported (Controller) is bit 38 of the LMP features, page 0
/// (Vol 2, Part C, 3.3), returned by Read Local Supported Features
/// (Vol 4, Part E, 7.4.3) as eight octets, bit 0 first.
pub fn le_supported(ret: &[u8]) -> Result<bool, HciError> {
    let features: [u8; 8] = Reader::new(ret).array()?;
    Ok(features[38 / 8] & 1 << (38 % 8) != 0)
}

/// LE Read Local Supported Features (Vol 4, Part E, 7.8.3): eight octets.
pub fn le_features(ret: &[u8]) -> Result<u64, HciError> {
    Ok(u64::from_le_bytes(Reader::new(ret).array()?))
}
