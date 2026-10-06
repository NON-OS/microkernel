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

//! An Intel function the device table does not list, taken by its class: it
//! is brought up only when its registers read as an LPSS I2C core, and the
//! look is reads alone, because on most laptops that class is also the
//! processor and PCH thermal subsystems and the telemetry function.

use nonos_devmodel::FakeBar;

use crate::constants::LPSS_PRIV_CAPS;
use crate::init::proves_lpss_i2c;
use crate::model::{live, WINDOW};
use crate::regs::Regs;

fn contents(bar: &FakeBar) -> Vec<u8> {
    (0..WINDOW).map(|o| bar.wrote8(o)).collect()
}

#[test]
fn a_function_that_is_not_a_designware_core_is_refused_and_left_untouched() {
    // A thermal subsystem's window: nothing at the DesignWare offsets.
    let bar = FakeBar::new(WINDOW);
    let before = contents(&bar);
    assert!(proves_lpss_i2c(Regs::new(bar.base())).is_err());
    assert_eq!(contents(&bar), before, "the look wrote into the window");
}

#[test]
fn an_lpss_i2c_core_proves_itself() {
    let bar = live();
    let before = contents(&bar);
    assert!(proves_lpss_i2c(Regs::new(bar.base())).is_ok());
    assert_eq!(contents(&bar), before, "the look wrote into the window");
}

#[test]
fn an_lpss_uart_or_an_undecoded_capabilities_register_is_refused() {
    for caps in [1 << 4, 2 << 4, u32::MAX] {
        let bar = live();
        bar.present32(LPSS_PRIV_CAPS as usize, caps);
        assert!(proves_lpss_i2c(Regs::new(bar.base())).is_err(), "caps {caps:#x} taken for I2C");
    }
}
