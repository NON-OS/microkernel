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

use alloc::vec;

use nonos_libc::{mk_ipc_recv_from_at, mk_ipc_send};

use super::dispatch::dispatch;
use super::gate::Sender;
use super::outcomes::Outcomes;
use crate::protocol::{decode_request, encode_response, EINVAL, KERNEL_REPLY_ENDPOINT};

const MAX_MSG: usize = 8 * 1024 * 1024;

pub fn run() -> ! {
    let mut buf = vec![0u8; MAX_MSG];
    let mut kept = Outcomes::new();
    loop {
        let (mut pid, mut sent_ms) = (0u32, 0u64);
        let n = mk_ipc_recv_from_at(0, buf.as_mut_ptr(), MAX_MSG, 0, &mut pid, &mut sent_ms);
        if n <= 0 {
            continue;
        }
        let resp = match decode_request(buf.get(..n as usize).unwrap_or(&[])) {
            Some(req) => dispatch(req, Sender { pid, sent_ms }, &mut kept),
            /*
             * Refused by the decode: answered under a zero sequence number.
             * The caller is blocked in its call until a reply comes, and the
             * kernel keeps its place in this service's reply queue until then.
             */
            None => encode_response(0, EINVAL, &[]),
        };
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, resp.as_ptr(), resp.len());
    }
}
