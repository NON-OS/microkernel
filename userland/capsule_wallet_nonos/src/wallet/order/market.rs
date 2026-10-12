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

/*
 * One call to market.index for a named purchase (B8, v1.4.36): the terms
 * are the market's; the store's names choose the listing, nothing else.
 */

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup, mk_time_millis};
use nonos_market_proto::{reply_body, request, SERVICE};

const TIMEOUT_MS: u64 = 2_000;
const REPLY_MAX: usize = 16 << 10;
const SILENT: &str = "The market did not answer, so nothing can be shown to confirm.";
pub(super) const TOO_LONG: &str = "That request is too long to send.";
pub(super) const UNREAD: &str =
    "The market's answer could not be read, so nothing can be confirmed.";

pub(super) fn ask(op: u16, body: &[u8]) -> Result<Vec<u8>, &'static str> {
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid) != 0 || pid == 0 {
        return Err(SILENT);
    }
    let id = mk_time_millis() as u32;
    let tx = request(op, id, body).ok_or(TOO_LONG)?;
    let mut rx = vec![0u8; REPLY_MAX];
    let (out, cap) = (rx.as_mut_ptr(), rx.len());
    let n = mk_ipc_call_timeout(port as u64, tx.as_ptr(), tx.len(), out, cap, TIMEOUT_MS);
    if n <= 0 {
        return Err(SILENT);
    }
    reply_body(&rx[..n as usize], op, id).map(<[u8]>::to_vec).map_err(|_| UNREAD)
}
