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

//! Interrupts masked and the board on its main supply, before anything else
//! (tg3_disable_ints, tg3_pwrsrc_switch_to_vmain).

use crate::constants::grc::GRC_LOCAL_CTRL;
use crate::constants::hostcc::IRQ_MAX_VECS;
use crate::constants::mailbox::int_mbox;
use crate::regs::Regs;
use crate::setup::Driver;

use super::wait;

/// TG3_GRC_LCLCTL_PWRSW_DELAY is 100 us; the shortest hold is 1 ms.
const PWRSW_HOLD_MS: u64 = 1;

/// A 1 in each vector's interrupt mailbox masks it. Host control already
/// carries MASK_PCI_INT from identify, so it is not rewritten here. The
/// driver polls and never unmasks.
pub fn disable_ints(regs: &Regs) {
    for n in 0..IRQ_MAX_VECS {
        // SAFETY: `regs` carries the broker MmioMap base for BAR0; each
        // interrupt mailbox is a 32-bit register inside its first page.
        unsafe { regs.w32f(int_mbox(n), 1) };
    }
}

/// An add-in card (IS_NIC) is moved off auxiliary power. A LOM's GPIOs are
/// the board's and are left alone.
pub fn to_vmain(d: &Driver) {
    if d.chip.hw.eeprom_write_protect {
        return;
    }
    // SAFETY: as above; GRC_LOCAL_CTRL is a register inside BAR0.
    unsafe { d.regs.w32f(GRC_LOCAL_CTRL, d.modes.grc_local_ctrl) };
    wait::hold(PWRSW_HOLD_MS);
}
