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

use super::decode_commerce;
use super::error::DecodeError;
use super::reader::Reader;
use crate::types::Commerce;

/// A commerce block on its own, as `OP_GET_COMMERCE` carries it: exactly
/// one block, nothing before it or after.
pub fn commerce_from_bytes(bytes: &[u8]) -> Result<Commerce, DecodeError> {
    let mut r = Reader::new(bytes);
    let c = decode_commerce::read(&mut r)?;
    if r.remaining() != 0 {
        return Err(DecodeError::Trailing);
    }
    Ok(c)
}
