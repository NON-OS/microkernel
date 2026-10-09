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

//! The body of an EXTENDED2, or why there is none.

use crate::cell::{body, unpack, RELAY_EXTENDED2, RELAY_TRUNCATED};

use super::error::BuildError;

pub(super) fn extended2_body(
    payload: &[u8; crate::cell::PAYLOAD_BYTES],
) -> Result<&[u8], BuildError> {
    let header = unpack(payload);
    if header.command == RELAY_TRUNCATED {
        return Err(BuildError::Destroyed);
    }
    if header.command != RELAY_EXTENDED2 || header.stream != 0 {
        return Err(BuildError::Protocol);
    }
    body(payload).ok_or(BuildError::Protocol)
}
