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
 * A named purchase's terms, read from market.index itself (B8, v1.4.36):
 * GET_COMMERCE, the release's escrow fields, and INSTALL_READY on the
 * chain the names give. The store's names choose the listing, nothing else.
 */

use alloc::vec;
use alloc::vec::Vec;

use nonos_libc::{mk_ipc_call_timeout, mk_service_lookup, mk_time_millis};
use nonos_market_proto::{pair_body, parse_commerce, parse_readiness, parse_release_escrow};
use nonos_market_proto::{ready_body, reply_body, request, SERVICE};
use nonos_market_proto::{OP_GET_COMMERCE, OP_GET_RELEASE, OP_INSTALL_READY};
use nonos_payment_proto::Names;

use super::read::{read_named, Named};

const TIMEOUT_MS: u64 = 2_000;
const REPLY_MAX: usize = 16 << 10;
const SILENT: &str = "The market did not answer, so nothing can be shown to confirm.";
const UNREAD: &str = "The market's answer could not be read, so nothing can be confirmed.";

pub fn read(names: &Names) -> Result<Named, &'static str> {
    let (l, r) = (&names.listing[..], &names.release[..]);
    let c = parse_commerce(&ask(OP_GET_COMMERCE, &pair_body(l, r))?).ok_or(UNREAD)?;
    let e = parse_release_escrow(&ask(OP_GET_RELEASE, &pair_body(l, r))?).ok_or(UNREAD)?;
    let body = ready_body(l, r, names.chain_id);
    let ready = parse_readiness(&ask(OP_INSTALL_READY, &body)?).ok_or(UNREAD)?;
    read_named(names.chain_id, &c, &e, &ready)
}

fn ask(op: u16, body: &[u8]) -> Result<Vec<u8>, &'static str> {
    let (mut port, mut pid) = (0u32, 0u32);
    if mk_service_lookup(SERVICE.as_ptr(), SERVICE.len(), &mut port, &mut pid) != 0 || pid == 0 {
        return Err(SILENT);
    }
    let id = mk_time_millis() as u32;
    let tx = request(op, id, body);
    let mut rx = vec![0u8; REPLY_MAX];
    let (out, cap) = (rx.as_mut_ptr(), rx.len());
    let n = mk_ipc_call_timeout(port as u64, tx.as_ptr(), tx.len(), out, cap, TIMEOUT_MS);
    if n <= 0 {
        return Err(SILENT);
    }
    reply_body(&rx[..n as usize], op, id).map(<[u8]>::to_vec).map_err(|_| UNREAD)
}
