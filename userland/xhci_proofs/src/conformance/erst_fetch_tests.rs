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

//! The host model's event ring start (`erst_fetch`): an ERSTBA written
//! while Bus Master is off is fetched with it off, even though the model
//! looks from a thread of its own, and is a Host Controller Error until a
//! reset; one written with Bus Master on is fetched. Looks slow from their
//! ERSTBA read to their fetch show a Command write that waited for one look
//! too few, or for a look that counted itself before it had fetched.

use std::time::Duration;

use nonos_devmodel::{run, FakeBar};
use nonos_libc::{mk_pci_config_write, MK_PCI_CFG_COMMAND};
use nonos_libc::{MK_PCI_CMD_BUS_MASTER, MK_PCI_CMD_MEMORY_SPACE};

use super::bringup_model::{bringup_controller, CMD, ERSTBA, STS};
use super::erst_fetch::ErstFetch;
use crate::constants::{USBCMD_HCRST, USBSTS_HCE};

#[test]
fn an_erstba_written_with_bus_mastering_off_is_a_host_controller_error_until_a_reset() {
    for slow in [Duration::ZERO, Duration::from_millis(20)] {
        fetched_under_the_bus_master_bit_written_under(slow);
    }
}

/// The model's looks each `slow` from their ERSTBA read to their fetch.
fn fetched_under_the_bus_master_bit_written_under(slow: Duration) {
    let bar = bringup_controller();
    let erst = ErstFetch::on_this_thread().slowed(slow);
    let _hc = run(&bar, move |bar: &FakeBar| {
        erst.turn(bar, bar.wrote32(CMD));
    });
    // Each Command write lands once the model has taken what came before,
    // and what that look showed is in USBSTS by then.
    let command = |bits| mk_pci_config_write(7, 1, MK_PCI_CFG_COMMAND, bits);
    let (on, hce) =
        (MK_PCI_CMD_MEMORY_SPACE | MK_PCI_CMD_BUS_MASTER, || bar.wrote32(STS) & USBSTS_HCE);
    command(MK_PCI_CMD_MEMORY_SPACE);
    bar.present32(ERSTBA, 0x1000);
    command(on);
    assert_eq!(hce(), USBSTS_HCE, "slow {slow:?}: fetched with Bus Master off");
    bar.present32(CMD, USBCMD_HCRST);
    command(on);
    bar.present32(CMD, 0);
    bar.present32(ERSTBA, 0x2000);
    command(on);
    assert_eq!(hce(), 0, "slow {slow:?}: cleared by the reset, fetched with Bus Master on");
    command(0);
}
