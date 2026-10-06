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


//! Alerts (RFC 5246 7.2). Every alert during the handshake ends it; after
//! it, close_notify ends the session cleanly and anything else ends it as
//! broken.

use super::error::Tls12Error;

pub const CLOSE_NOTIFY: u8 = 0;

/// The description byte of an alert record's body.
pub fn description(body: &[u8]) -> Result<u8, Tls12Error> {
    match body {
        [_level, description] => Ok(*description),
        _ => Err(Tls12Error::Malformed),
    }
}
