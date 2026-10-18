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

//! What the Bus Master tests read the part through: the resets the model has
//! done, both engines, and whether both rings name this driver's frames.

use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_libc::Part;

use crate::constants::regs::{REG_RCTL, REG_RDBAH, REG_RDBAL, REG_TCTL, REG_TDBAH, REG_TDBAL};
use crate::constants::rxtx::{RCTL_EN, TCTL_EN};
use crate::model::memory::{RX_RING_PHYS, TX_RING_PHYS};
use crate::model::part::Behaviour;

pub fn probe(bar: &Arc<FakeBar>, how: &Arc<Behaviour>) -> impl Fn() -> Part + 'static {
    let (b, how) = (Arc::clone(bar), Arc::clone(how));
    move || {
        let at = |lo, hi| ((b.wrote32(hi) as u64) << 32) | b.wrote32(lo) as u64;
        Part {
            resets: how.resets.lock().unwrap().len() as u32,
            rx_on: b.wrote32(REG_RCTL) & RCTL_EN != 0,
            tx_on: b.wrote32(REG_TCTL) & TCTL_EN != 0,
            rings_own: at(REG_RDBAL, REG_RDBAH) == RX_RING_PHYS
                && at(REG_TDBAL, REG_TDBAH) == TX_RING_PHYS,
        }
    }
}

pub fn resets(how: &Behaviour) -> u32 {
    how.resets.lock().unwrap().len() as u32
}
