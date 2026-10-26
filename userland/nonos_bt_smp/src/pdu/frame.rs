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

//! One SMP PDU on the SMP fixed channel. Its length must match the code's
//! exactly: SMP has no optional fields, so a longer or shorter PDU is
//! malformed (Vol 3, Part H, 3.3).

use super::code::params_len;
use alloc::vec::Vec;
use nonos_bt_wire::WireError;

/// The code and its parameters; `Reserved` for an unknown code (answered
/// with Pairing Failed, reason 07h (COMMAND_NOT_SUPPORTED), by the caller).
pub fn parse(pdu: &[u8]) -> Result<(u8, &[u8]), WireError> {
    let (&code, params) = pdu.split_first().ok_or(WireError::Short)?;
    match params_len(code) {
        None => Err(WireError::Reserved),
        Some(n) if params.len() < n => Err(WireError::Short),
        Some(n) if params.len() > n => Err(WireError::Length),
        Some(_) => Ok((code, params)),
    }
}

/// The PDU for `code` with `params`, refused unless the length fits.
pub fn frame(code: u8, params: &[u8]) -> Result<Vec<u8>, WireError> {
    match params_len(code) {
        None => Err(WireError::Reserved),
        Some(n) if n != params.len() => Err(WireError::Length),
        Some(_) => Ok([&[code][..], params].concat()),
    }
}
