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

//! One I2C-over-AUX transaction (DisplayPort 1.4a section 2.7.7.1.6),
//! retried on DEFER up to RETRIES times and, when a deadline is given, not
//! begun once it has passed (the EDID read on it: `i2c_edid.rs`).

use super::aux::{i2c_reply, native_reply, Aux, AuxError, Reply};
use super::native::RETRIES;
use crate::engine::access::Expiry;

/// The EDID's I2C address (VESA E-DDC).
pub const EDID_ADDR: u32 = 0x50;

/// No deadline but the retries' count.
struct Unbounded;

impl Expiry for Unbounded {
    fn expired(&self) -> bool {
        false
    }
}

pub fn once(aux: &mut dyn Aux, req: &[u8]) -> Result<([u8; 20], usize), AuxError> {
    within(aux, req, &Unbounded)
}

/// `once`, with no attempt begun after `until`.
pub fn within(
    aux: &mut dyn Aux,
    req: &[u8],
    until: &dyn Expiry,
) -> Result<([u8; 20], usize), AuxError> {
    let mut last = AuxError::Timeout;
    for _ in 0..RETRIES {
        if until.expired() {
            return Err(AuxError::Timeout);
        }
        let mut r = [0u8; 20];
        match aux.transfer(req, &mut r) {
            Ok(n) if n >= 1 => match (native_reply(r[0]), i2c_reply(r[0])) {
                (Some(Reply::Ack), Some(Reply::Ack)) => return Ok((r, n)),
                (Some(Reply::Defer), _) | (Some(Reply::Ack), Some(Reply::Defer)) => {
                    last = AuxError::Refused
                }
                _ => return Err(AuxError::Refused),
            },
            Ok(_) => last = AuxError::Short,
            Err(e) => last = e,
        }
    }
    Err(last)
}
