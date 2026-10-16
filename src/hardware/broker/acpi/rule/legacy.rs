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

//! The legacy PC ports the kernel owns, as [lo, hi) ranges.

use super::refusal::Legacy;

/// The 8237 DMA controllers (0x00-0x1F, 0xC0-0xDF) and their page registers
/// (0x81-0x8F; 0x80 is left out, as firmware writes POST codes there), the
/// 8259 pair (0x20-0x21, 0xA0-0xA1) and the IMCR beside it (0x22-0x23,
/// which can hand interrupts back to the 8259), the 8254 (0x40-0x43), and
/// the PCI configuration mechanism's 0xCF8-0xCFF, which holds the reset
/// control register at 0xCF9 too. The CMOS pair (0x70-0x71), COM1
/// (0x3F8-0x3FF), the VGA CRT pair (0x3D4-0x3D5) and the debug ports (0xE9,
/// 0x402) are the kernel's clock, log and console; port 0x92 resets the
/// processor. The ports AML may reach are `allowed::AML_PORTS`.
pub const LEGACY_PORTS: [(u64, u64, Legacy); 14] = [
    (0x00, 0x20, Legacy::Dma),
    (0x20, 0x22, Legacy::Pic),
    (0x22, 0x24, Legacy::Pic),
    (0x40, 0x44, Legacy::Pit),
    (0x70, 0x72, Legacy::Rtc),
    (0x81, 0x90, Legacy::Dma),
    (0x92, 0x93, Legacy::Reset),
    (0xA0, 0xA2, Legacy::Pic),
    (0xC0, 0xE0, Legacy::Dma),
    (0xE9, 0xEA, Legacy::Debug),
    (0x3D4, 0x3D6, Legacy::Console),
    (0x3F8, 0x400, Legacy::Serial),
    (0x402, 0x403, Legacy::Debug),
    (0xCF8, 0xD00, Legacy::PciConfig),
];
