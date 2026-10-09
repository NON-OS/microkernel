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

//! The doorbell's answer from one register read: which level is "a report
//! waits" on Intel PADCFG0 and AMD pin registers, for both polarities a
//! GpioInt declares. The bit values come from nonos_pinctrl, the same
//! constants the driver puts in its Doorbell.

use nonos_pinctrl::{AMD_PIN_STS, INTEL_RXSTATE};

use crate::driver::Doorbell;
use crate::regs::Regs;

fn bell(level_bit: u32, active_high: bool) -> Doorbell {
    Doorbell { regs: Regs::new(0), cfg_offset: 0, level_bit, active_high }
}

#[test]
fn an_active_low_intel_pad_rings_while_its_rx_state_is_low() {
    let db = bell(INTEL_RXSTATE, false);
    // PADCFG0 with GPIO mode and RX disabled bits set, RX state low.
    assert!(db.asserted(0x4400_0100));
    assert!(!db.asserted(0x4400_0102));
}

#[test]
fn an_active_low_amd_pin_rings_while_pin_sts_is_low() {
    let db = bell(AMD_PIN_STS, false);
    // Level trigger, active low, interrupt enabled bits; PIN_STS clear.
    assert!(db.asserted(0x0000_1A00));
    assert!(!db.asserted(0x0001_1A00));
}

#[test]
fn an_active_high_line_rings_while_high() {
    assert!(bell(INTEL_RXSTATE, true).asserted(0x2));
    assert!(!bell(AMD_PIN_STS, true).asserted(0x0));
}
