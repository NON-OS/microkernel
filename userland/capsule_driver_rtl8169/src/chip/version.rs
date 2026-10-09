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

/// `RTL_GIGA_MAC_VER_<nn>`, by its number. Linux declares the enum in the
/// same ascending order, so a range here is the same range as in a Linux
/// `case RTL_GIGA_MAC_VER_40 ... RTL_GIGA_MAC_VER_52:`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct MacVersion(pub u8);

impl MacVersion {
    /// The PCI RTL8169/8110 parts, which Linux starts with rtl_hw_start_8169.
    pub fn is_8169(self) -> bool {
        self.0 <= 6
    }

    /// Linux rtl_init_one sets a 64-bit DMA mask only from VER_18 (8168cp)
    /// on; the 8169, 8168b and 810x before it take 32-bit addresses.
    pub fn dma32_only(self) -> bool {
        self.0 < 18
    }

    /// Linux rtl_is_8125: the 2.5G family and everything after it.
    pub fn is_8125(self) -> bool {
        self.0 >= 61
    }

    /// The 8168g generation and later 8168s (rtl_hw_initialize's
    /// VER_40 ... VER_52 arm): MCU-gated RX, MAC OCP, the RXDV gate.
    pub fn is_8168g_up(self) -> bool {
        (40..=52).contains(&self.0)
    }

    /// Linux rtl_is_8168evl_up.
    pub fn is_8168evl_up(self) -> bool {
        self.0 >= 34 && self.0 != 39 && self.0 <= 52
    }
}
