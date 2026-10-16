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

//! One request in, one reply out. A header that does not decode is
//! answered E_INVAL under an empty header, as the SATA driver answers it.

use super::answer::answer;
use super::fault::Fault;
use super::medium::Medium;
use super::reply::send;
use crate::protocol::{decode_request, Request, E_INVAL, HDR_LEN, STATUS_LEN};
use crate::setup::Driver;

/// Answer `rx` from `sender` through `tx`; the bytes the reply used.
pub fn serve(
    drv: &mut Driver,
    medium: &mut Medium,
    sender: u32,
    rx: &[u8],
    tx: &mut [u8],
) -> usize {
    let Some(req) = decode_request(rx) else {
        return send(sender, &Request::default(), Err(E_INVAL), tx);
    };
    let body = rx.get(HDR_LEN..).unwrap_or(&[]);
    let out = tx.get_mut(HDR_LEN + STATUS_LEN..).unwrap_or(&mut []);
    let answer = answer(drv, medium, sender, &req, body, out).map_err(Fault::status);
    send(sender, &req, answer, tx)
}
