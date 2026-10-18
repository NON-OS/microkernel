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

//! The service loop: the slot looked at, then a request waited for, for
//! at most the time between two looks, so a card put in or taken out is
//! seen within half a second while nobody asks. Neither the data a request
//! sent nor the data it was answered with stays in the buffers (C8).

use alloc::vec;

use nonos_libc::{mk_ipc_recv_from, recv_ready};

use super::medium::Medium;
use super::request::serve;
use super::slot::look;
use crate::protocol::{BLOCK_BYTES, HDR_LEN, MAX_RW_BLOCKS, RW_HEADER_LEN, STATUS_LEN};
use crate::setup::Driver;

const LOOK_MS: u64 = 500;
const DATA: usize = MAX_RW_BLOCKS as usize * BLOCK_BYTES;
const RX_LEN: usize = HDR_LEN + RW_HEADER_LEN + DATA;
const TX_LEN: usize = HDR_LEN + STATUS_LEN + DATA;

pub fn run(drv: &mut Driver) -> ! {
    let mut medium = Medium::Empty;
    let (mut rx, mut tx) = (vec![0u8; RX_LEN], vec![0u8; TX_LEN]);
    loop {
        look(drv, &mut medium);
        serve_one(drv, &mut medium, &mut rx, &mut tx);
    }
}

/// A request waited for until the next look, answered, and wiped from
/// both buffers once the reply is out.
pub fn serve_one(drv: &mut Driver, medium: &mut Medium, rx: &mut [u8], tx: &mut [u8]) {
    let mut sender = 0u32;
    let n = mk_ipc_recv_from(0, rx.as_mut_ptr(), rx.len(), LOOK_MS, &mut sender);
    if !recv_ready(n) {
        return;
    }
    let got = (n as usize).min(rx.len());
    let sent = serve(drv, medium, sender, rx.get(..got).unwrap_or(&[]), tx);
    rx.iter_mut().take(got).for_each(|b| *b = 0);
    tx.iter_mut().take(sent).for_each(|b| *b = 0);
}
