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

//! The ports an IDE channel in compatibility mode answers at.
//!
//! Such a channel decodes the addresses the PC/AT gave its ATA channel and
//! ignores the BARs for it. The programming interface byte of class 01h/01h
//! says which mode each channel runs: bit 0 set is the primary channel in
//! native mode, bit 2 the secondary (PCI IDE Controller Specification 1.0).
//! Pure, so the proofs run it on the host.

pub(crate) const IDE_CLASS: u8 = 0x01;
pub(crate) const IDE_SUBCLASS: u8 = 0x01;
const PRIMARY_NATIVE: u8 = 1 << 0;
const SECONDARY_NATIVE: u8 = 1 << 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Ports {
    pub(crate) base: u16,
    pub(crate) count: u16,
}

// The command block, then the one control-block port (device control when
// written, alternate status when read). 0x3F7 and 0x377 stay out: a floppy
// controller's digital input register answers at 0x3F7.
pub(crate) const PRIMARY: [Ports; 2] =
    [Ports { base: 0x1F0, count: 8 }, Ports { base: 0x3F6, count: 1 }];
pub(crate) const SECONDARY: [Ports; 2] =
    [Ports { base: 0x170, count: 8 }, Ports { base: 0x376, count: 1 }];

/// BARs 0 to 3 of an IDE function, in the order the specification gives them
/// in native mode. A channel in native mode keeps its own BARs and
/// contributes nothing here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Fixed {
    pub(crate) bars: [Option<Ports>; 4],
}

pub(crate) const fn ide(class: u8, subclass: u8, progif: u8) -> Fixed {
    let mut f = Fixed { bars: [None; 4] };
    if class != IDE_CLASS || subclass != IDE_SUBCLASS {
        return f;
    }
    if progif & PRIMARY_NATIVE == 0 {
        f.bars[0] = Some(PRIMARY[0]);
        f.bars[1] = Some(PRIMARY[1]);
    }
    if progif & SECONDARY_NATIVE == 0 {
        f.bars[2] = Some(SECONDARY[0]);
        f.bars[3] = Some(SECONDARY[1]);
    }
    f
}
