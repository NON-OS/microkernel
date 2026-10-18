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

//! INTMS masks the pin interrupts nothing services when no MSI-X vector is
//! bound. A reset clears it, so every enable masks again; under MSI-X the
//! driver leaves INTMS alone (NVMe 1.4, 3.1.4). `up_on` checks the first
//! enable either way.

use crate::constants::REG_INTMS;
use crate::fake::{register, up, up_on, Up};

#[test]
fn without_msix_every_enable_after_a_reset_masks_again() {
    let Up { regs, mut device, mut admin, .. } = up();
    for _ in 0..2 {
        assert!(device.disable().is_ok(), "the fake disables");
        assert_eq!(register(regs, REG_INTMS), 0, "the reset cleared the mask");
        admin.restart(regs);
        assert!(device.enable().is_ok(), "and enables again");
        assert_eq!(register(regs, REG_INTMS), u32::MAX, "masked again");
    }
}

#[test]
fn under_msix_intms_is_never_written() {
    let Up { regs, mut device, mut admin, .. } = up_on(9);
    assert!(device.disable().is_ok());
    admin.restart(regs);
    assert!(device.enable().is_ok());
    assert_eq!(register(regs, REG_INTMS), 0);
}
