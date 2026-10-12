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

use super::stop_regs::{halt_xhci, reset_ahci, reset_sdhci};

#[repr(C, align(4096))]
struct Regs([u32; 1024]);

impl Regs {
    fn at(&self) -> u64 {
        self.0.as_ptr() as u64
    }
}

#[test]
fn xhci_clears_run_stop_past_caplength_and_waits_for_halted() {
    let mut r = Regs([0; 1024]);
    r.0[0] = 0x0100_0020; // HCIVERSION 1.0, CAPLENGTH 0x20
    r.0[8] = 0x0000_0005; // USBCMD: Run/Stop and INTE
    r.0[9] = 0x0000_0001; // USBSTS: already halted
    assert!(halt_xhci(r.at()));
    assert_eq!(r.0[8], 0x0000_0004);
    // A controller that never reports halted is given up on, not waited for.
    r.0[8] = 0x0000_0001;
    r.0[9] = 0;
    assert!(!halt_xhci(r.at()));
    assert_eq!(r.0[8], 0);
}

#[test]
fn sdhci_sets_reset_all_and_keeps_the_bytes_beside_it() {
    let mut r = Regs([0; 1024]);
    r.0[0x2c / 4] = 0x00e0_0007; // timeout 0xe, clock enabled
    assert!(!reset_sdhci(r.at()));
    assert_eq!(r.0[0x2c / 4], 0x01e0_0007);
}

#[test]
fn ahci_sets_hba_reset_and_keeps_ahci_enable() {
    let mut r = Regs([0; 1024]);
    r.0[1] = 0x8000_0000;
    assert!(!reset_ahci(r.at()));
    assert_eq!(r.0[1], 0x8000_0001);
}
