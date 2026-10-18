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

//! A part that completes a software reset on the next look and counts the
//! resets it has done, and the probe the Bus Master tests read it through:
//! its resets, both engines, and whether both rings name this driver's.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_libc::Part;

use super::memory::{RX_DESC_DA, TX_DESC_DA};
use crate::constants::regs::{CMD_RESET, CMD_RX_ENABLE, CMD_TX_ENABLE, REG_CMD};
use crate::constants::regs::{REG_RXDESC_ADDR_HI, REG_RXDESC_ADDR_LO};
use crate::constants::regs::{REG_TXDESC_ADDR_HI, REG_TXDESC_ADDR_LO};

/// The reset, as the chip does it: CmdReset clears itself with both engines
/// off. Counted before it shows, so a driver that saw it end never reads a
/// count that misses it.
pub fn counting(resets: &Arc<AtomicU32>) -> impl Fn(&FakeBar) + Send + 'static {
    let n = Arc::clone(resets);
    move |b| {
        let cmd = b.wrote8(REG_CMD);
        if cmd & CMD_RESET != 0 {
            n.fetch_add(1, Ordering::SeqCst);
            b.present8(REG_CMD, cmd & !(CMD_RESET | CMD_RX_ENABLE | CMD_TX_ENABLE));
        }
    }
}

pub fn probe(bar: &Arc<FakeBar>, resets: &Arc<AtomicU32>) -> impl Fn() -> Part + 'static {
    let (b, n) = (Arc::clone(bar), Arc::clone(resets));
    move || {
        let at = |lo, hi| ((b.wrote32(hi) as u64) << 32) | b.wrote32(lo) as u64;
        let cmd = b.wrote8(REG_CMD);
        Part {
            resets: n.load(Ordering::SeqCst),
            rx_on: cmd & CMD_RX_ENABLE != 0,
            tx_on: cmd & CMD_TX_ENABLE != 0,
            rings_own: at(REG_RXDESC_ADDR_LO, REG_RXDESC_ADDR_HI) == RX_DESC_DA
                && at(REG_TXDESC_ADDR_LO, REG_TXDESC_ADDR_HI) == TX_DESC_DA,
        }
    }
}
