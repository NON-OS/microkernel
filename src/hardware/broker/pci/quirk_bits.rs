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

//! The few vendor bits a driver of a claimed device may flip beyond Command
//! and MSI-X, each the exact bits Linux's driver for that class writes and
//! nothing beside them. Pure, so kernel_proofs holds every rule.
//!
//! - Intel HD Audio (class 04, subclass 03 or 01 for a DSP-mode part):
//!   TCSEL at 0x44 bits 0-2, CGCTL MISCBDCGE at 0x48 bit 6 (cleared around
//!   the controller reset on Skylake and later, Gemini Lake included) and
//!   DEVC NOSNOOP at 0x78 bit 11 (snd_hda_intel's azx_init_pci).
//! - AMD/ATI HD Audio: the snoop bits at 0x42 bits 0-2.
//! - A network controller (class 02): PCIe Device Control 2's completion
//!   timeout value and disable, bits 0-4 at the PCIe capability + 0x28, which
//!   rtw88 and iwlwifi set on real silicon.
//!
//! Anything else stays refused: these registers hold no address, no BAR, no
//! routing, and change nothing about what the device may reach.

/// What the broker knows of the claimed function.
#[derive(Debug, Clone, Copy)]
pub struct Ident {
    pub vendor: u16,
    pub class: u8,
    pub subclass: u8,
    /// The PCI Express capability's offset, when the function has one.
    pub pcie_cap: Option<u16>,
}

const INTEL: u16 = 0x8086;
const AMD: u16 = 0x1022;
const ATI: u16 = 0x1002;

/// The bits at `offset` a driver of `id` may change, or `None` when the
/// register is not one of these.
pub fn writable(id: &Ident, offset: u32) -> Option<u16> {
    let audio = id.class == 0x04 && matches!(id.subclass, 0x01 | 0x03);
    match (id.vendor, offset) {
        (INTEL, 0x44) if audio => Some(0x0007),
        (INTEL, 0x48) if audio => Some(0x0040),
        (INTEL, 0x78) if audio => Some(0x0800),
        (AMD | ATI, 0x42) if audio => Some(0x0007),
        _ if id.class == 0x02 && id.pcie_cap.is_some_and(|c| offset == c as u32 + 0x28) => {
            Some(0x001F)
        }
        _ => None,
    }
}

/// Whether `new` differs from `current` only in the bits `mask` allows.
pub fn only(mask: u16, new: u16, current: u16) -> bool {
    (new ^ current) & !mask == 0
}
