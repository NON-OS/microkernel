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

//! driver.bt_usb0's desk: the board, and the inbox NBTC requests land in.
//! The capsule's only way to sleep is a receive on that inbox with a
//! timeout, so every wait answers what arrives meanwhile, at once: the
//! kernel pairs a reply with the request its server took last, and the
//! caller waits in its call until then.

use alloc::rc::Rc;
use core::cell::{RefCell, RefMut};

use super::kept::Kept;
use super::serve::{now, serve};
use crate::nbtc::Board;
use nonos_libc::mk_ipc_recv_from;

const INBOX: u64 = 0;
/// Room for any NBTC request: the header and five octets at most; a longer
/// message is cut here and refused as malformed.
const RX_MAX: usize = 64;

/// Shared by the link's environment and the USB pipes, whose waits answer
/// requests in the middle of an HCI exchange.
#[derive(Clone)]
pub struct Desk(Rc<RefCell<Board<Kept>>>);

impl Desk {
    pub fn new(board: Board<Kept>) -> Self {
        Self(Rc::new(RefCell::new(board)))
    }

    /// The board, for the link's environment. No wait runs while it is
    /// held: the environment's calls make no HCI exchange.
    pub fn board(&self) -> RefMut<'_, Board<Kept>> {
        self.0.borrow_mut()
    }

    /// About `ms` milliseconds, answering each request that arrives.
    pub fn wait(&self, ms: u64) {
        let until = now().saturating_add(ms);
        let mut rx = [0u8; RX_MAX];
        loop {
            let left = until.saturating_sub(now()).max(1);
            let mut from = 0u32;
            let n = mk_ipc_recv_from(INBOX, rx.as_mut_ptr(), rx.len(), left, &mut from);
            if n <= 0 {
                return;
            }
            // The kernel's own messages carry pid 0 and wait on no reply.
            if from != 0 {
                serve(&self.0, &rx[..(n as usize).min(RX_MAX)], from);
            }
            if now() >= until {
                return;
            }
        }
    }
}
