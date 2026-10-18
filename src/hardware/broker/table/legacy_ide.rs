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

//! The windows of an IDE channel in compatibility mode.
//!
//! PCI IDE 1.0, 2.1: a channel whose programming interface bit (0 for the
//! primary, 2 for the secondary) is clear decodes the ISA addresses, and its
//! BARs read zero or are ignored. QEMU's PIIX3, VirtualBox's PIIX3/PIIX4
//! and Hyper-V's PIIX4 all come up that way. The record lists those
//! addresses in the BARs a native channel would use, so a driver claims a
//! channel's ports through the same grant path either way: the command block
//! (eight ports), and the control register alone, never the floppy
//! controller's ports beside it.

use crate::hardware::broker::{Bar, BarKind};

const CLASS_STORAGE: u8 = 0x01;
const SUBCLASS_IDE: u8 = 0x01;
/// (command block, control register) of the primary and the secondary.
const LEGACY: [(u64, u64); 2] = [(0x1F0, 0x3F6), (0x170, 0x376)];
const COMMAND_PORTS: u64 = 8;
const CONTROL_PORTS: u64 = 1;

/// List the compatibility-mode channels of an IDE function in `bars`,
/// widening `span` to cover them; any other function is left as it is.
pub(super) fn fill(class: u8, subclass: u8, progif: u8, bars: &mut [Bar; 6], span: &mut u8) {
    if class != CLASS_STORAGE || subclass != SUBCLASS_IDE {
        return;
    }
    for (c, &(cmd, ctl)) in LEGACY.iter().enumerate() {
        if progif & (1 << (2 * c)) != 0 {
            continue;
        }
        bars[2 * c] = pio(cmd, COMMAND_PORTS);
        bars[2 * c + 1] = pio(ctl, CONTROL_PORTS);
        *span = (*span).max(2 * c as u8 + 2);
    }
}

fn pio(base: u64, size: u64) -> Bar {
    Bar { base, size, kind: BarKind::Pio as u8, flags: 0, aux: 0, _pad: [0; 2] }
}
