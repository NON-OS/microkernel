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

//! Programming both legacy virtqueues, receive first, all or nothing.

use super::program_queue::program_queue;
use crate::constants::{Q_RX, Q_TX};
use crate::queue::{RxQueue, TxQueue};
use crate::regs::Regs;
use crate::transport::Transport;

/// Point the receive and transmit queues at the rings at `rx_phys` and
/// `tx_phys`; the number of buffers to prime on each. When either is
/// refused the receive queue may already hold its page number, in a grant
/// the failed attempt releases next: the device is reset first, so it has
/// let go of every ring before the grants go (plan/50 C2).
pub fn program_queues(regs: Regs, rx_phys: u64, tx_phys: u64) -> Result<(u16, u16), &'static str> {
    let both = program_queue(regs, Q_RX, rx_phys, RxQueue::queue_size()).and_then(|rx| {
        let tx = program_queue(regs, Q_TX, tx_phys, TxQueue::queue_size())?;
        Ok((rx, tx))
    });
    if both.is_err() {
        let _ = Transport::Legacy(regs).reset();
    }
    both
}
