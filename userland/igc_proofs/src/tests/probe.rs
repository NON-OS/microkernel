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

//! The part counted, and the probe the Bus Master tests read it through:
//! its resets, both queues, and whether both rings name this driver's.

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_libc::Part;

use super::memory::{RX_RING_PHYS, TX_RING_PHYS};
use super::model::stable32;
use crate::constants::ctrl::{CTRL_GIO_MASTER_DISABLE, CTRL_RST, EECD_AUTO_RD, RAH_AV};
use crate::constants::regs::*;
use crate::constants::rx_bits::RXDCTL_QUEUE_ENABLE;
use crate::constants::tx_bits::TXDCTL_QUEUE_ENABLE;

/// What the NVM auto-read puts back in RAR0 on every reset.
pub const FACTORY: (u32, u32) = (0x5634_1200, RAH_AV | 0x9A78);

/// A part that finishes a global reset on the next look, counting it first
/// so a driver that saw it end never reads a count that misses it: queue 0
/// left enabled, as Linux's igc_setup_tctl says a reset may leave it, the
/// factory address back in RAR0, CTRL.RST and the master disable dropped,
/// the NVM auto-read done. `part` is the rest of the part.
pub fn counting(
    resets: &Arc<AtomicU32>,
    part: impl Fn(&FakeBar) + Send + 'static,
) -> impl Fn(&FakeBar) + Send + 'static {
    let n = Arc::clone(resets);
    move |b| {
        let Some(ctrl) = stable32(b, REG_CTRL) else { return };
        if ctrl & CTRL_RST != 0 {
            n.fetch_add(1, Ordering::SeqCst);
            b.present32(REG_RXDCTL, RXDCTL_QUEUE_ENABLE);
            b.present32(REG_TXDCTL, TXDCTL_QUEUE_ENABLE);
            b.present32(REG_RAL0, FACTORY.0);
            b.present32(REG_RAH0, FACTORY.1);
            b.present32(REG_CTRL, ctrl & !(CTRL_RST | CTRL_GIO_MASTER_DISABLE));
            b.present32(REG_EECD, EECD_AUTO_RD);
        }
        part(b);
    }
}

pub fn probe(bar: &Arc<FakeBar>, resets: &Arc<AtomicU32>) -> impl Fn() -> Part + 'static {
    let (b, n) = (Arc::clone(bar), Arc::clone(resets));
    move || {
        let at = |lo, hi| ((b.wrote32(hi) as u64) << 32) | b.wrote32(lo) as u64;
        Part {
            resets: n.load(Ordering::SeqCst),
            rx_on: b.wrote32(REG_RXDCTL) & RXDCTL_QUEUE_ENABLE != 0,
            tx_on: b.wrote32(REG_TXDCTL) & TXDCTL_QUEUE_ENABLE != 0,
            rings_own: at(REG_RDBAL, REG_RDBAH) == RX_RING_PHYS
                && at(REG_TDBAL, REG_TDBAH) == TX_RING_PHYS,
        }
    }
}
