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

//! A setup that fails on a DMA grant gives the device back before any ring
//! of this attempt was programmed. An I219 is stopped but not reset: the
//! rings firmware left may still hold descriptors, a reset on them is a unit
//! hang only a PCI reset clears, and the flush that clears them needs rings
//! this attempt never programmed. An older part has new master requests
//! blocked and both units stopped, is taken off the bus, then reset.

use std::sync::Arc;

use nonos_devmodel::FakeBar;
use nonos_libc::{seen, set_command, watch, MmioMapOut};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER as BM, MK_PCI_CMD_MEMORY_SPACE as MEM};

use super::flush_master_tests::pending;
use super::order::{no_reset_while_mastering, released_quiet};
use super::probe::probe;
use crate::constants::ctrl::{CTRL_GIO_MASTER_DISABLE, CTRL_RST};
use crate::constants::regs::{REG_CTRL, REG_RCTL};
use crate::constants::rxtx::RCTL_EN;
use crate::constants::Family;
use crate::model::live::live;
use crate::model::part::Behaviour;
use crate::model::window::{i82574, pch, phy_at};
use crate::setup::rollback::after;

pub fn grant(bar: &FakeBar) -> MmioMapOut {
    MmioMapOut { user_va: bar.base(), length: bar.len() as u64, grant_id: 20 }
}

#[test]
fn an_i219_whose_setup_failed_is_stopped_and_not_reset() {
    let bar = pch();
    pending(&bar);
    watch(probe(&bar, &Arc::new(Behaviour::default())));
    after(9, 1, &grant(&bar), Family::PchCnp);
    assert_eq!(bar.wrote32(REG_CTRL) & CTRL_RST, 0, "no reset over pending rings");
    assert_eq!(bar.wrote32(REG_RCTL) & RCTL_EN, 0, "the receiver stopped");
    released_quiet(&seen(), 0);
}

#[test]
fn an_older_part_whose_setup_failed_is_stopped_and_reset() {
    let bar = i82574();
    bar.present32(REG_RCTL, RCTL_EN);
    let part = live(&bar, phy_at(1), Behaviour::default());
    set_command(MEM | BM);
    watch(probe(&bar, &part.how));
    after(9, 1, &grant(&bar), Family::I82574);
    let found = part.how.resets.lock().unwrap().clone();
    assert_eq!(found.len(), 1, "the part was reset");
    assert_ne!(found[0] & CTRL_GIO_MASTER_DISABLE, 0, "the reset found the master stopped");
    let seen = seen();
    no_reset_while_mastering(&seen, 1);
    released_quiet(&seen, 1);
}
