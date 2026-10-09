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

use nonos_libc::{mk_ipc_recv_from, mk_ipc_send};

use super::dispatch::dispatch;
use super::reap::Reaper;
use super::wipe::wipe;
use crate::protocol::{decode_request, encode_response, EINVAL, KERNEL_REPLY_ENDPOINT};
use crate::store::Store;

/// Room for the largest request: a signed settlement's calldata and its header.
const MAX_MSG: usize = super::handlers::SIGN_TX_MAX_DATA + 4096;

pub fn run() -> ! {
    let mut buf = vec![0u8; MAX_MSG];
    let mut store = Store::new();
    let mut reaper = Reaper::new();
    loop {
        let mut sender_pid: u32 = 0;
        let n = mk_ipc_recv_from(0, buf.as_mut_ptr(), MAX_MSG, 0, &mut sender_pid);
        if n <= 0 {
            continue;
        }
        reaper.reap_if_due(&mut store);
        let used = n as usize;
        let resp = match decode_request(&buf[..used]) {
            Some(req) => dispatch(&mut store, req, sender_pid),
            /*
             * Too short to carry a sequence number: answered under zero. The
             * caller is blocked in its call until a reply comes, and the
             * kernel keeps its place in this service's reply queue until then.
             */
            None => encode_response(0, EINVAL, &[]),
        };
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, resp.as_ptr(), resp.len());
        wipe(&mut buf[..used]);
    }
}
