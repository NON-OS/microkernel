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

//! One native AUX transaction: retried on a DEFER or no reply up to the
//! retry count, each try bounded by the transmitter's own wait; a NACK or
//! a reply code the standard does not define ends it at once.

use super::aux::{native_reply, Aux, AuxError, Reply};
use super::native::RETRIES;

pub(super) fn once(aux: &mut dyn Aux, req: &[u8]) -> Result<([u8; 20], usize), AuxError> {
    let mut last = AuxError::Timeout;
    for _ in 0..RETRIES {
        let mut reply = [0u8; 20];
        match aux.transfer(req, &mut reply) {
            Ok(0) => last = AuxError::Short,
            Ok(n) => match native_reply(reply[0]) {
                Some(Reply::Ack) => return Ok((reply, n)),
                Some(Reply::Defer) => last = AuxError::Refused,
                _ => return Err(AuxError::Refused),
            },
            Err(e) => last = e,
        }
    }
    Err(last)
}
