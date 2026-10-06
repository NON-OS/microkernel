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

/// Family name and DesignWare input clock of an Intel LPSS I2C function,
/// from its PCI device id. The ids and rates are those of Linux
/// drivers/mfd/intel-lpss-pci.c: spt_i2c_info 120 MHz, bxt/apl/glk_i2c_info
/// 133 MHz, cnl_i2c_info 216 MHz. Only I2C functions are listed: the UART
/// and SPI functions of the same LPSS blocks sit at neighbouring ids and must
/// never be driven as I2C.
///
/// Where Linux and the platform datasheets disagree, or a family's rate is
/// not pinned down (Tiger Lake-LP, 120 or 133), the higher clock is used:
/// counts computed for a faster clock than the real one only slow the bus
/// and keep every minimum of UM10204 met, while the reverse runs it out of
/// spec.
pub fn device_info(device: u16) -> Option<(&'static str, u32)> {
    const SPT: u32 = 120_000_000;
    const BXT: u32 = 133_000_000;
    const CNL: u32 = 216_000_000;
    match device {
        // Broxton A/B step, Apollo Lake, Gemini Lake: I2C0..I2C7.
        0x0AAC..=0x0ABA if device & 1 == 0 => Some(("Broxton", BXT)),
        0x1AAC..=0x1ABA if device & 1 == 0 => Some(("Broxton-P", BXT)),
        0x5AAC..=0x5ABA if device & 1 == 0 => Some(("Apollo Lake", BXT)),
        0x31AC..=0x31BA if device & 1 == 0 => Some(("Gemini Lake", BXT)),
        // Skylake/Kaby Lake PCH (Sunrise Point), Kaby Lake-H, Comet Lake-V.
        0x9D60..=0x9D65 => Some(("Sunrise Point-LP", SPT)),
        0xA160..=0xA162 => Some(("Sunrise Point-H", SPT)),
        0xA2E0..=0xA2E3 => Some(("Kaby Lake-H", SPT)),
        0xA3E0..=0xA3E3 => Some(("Comet Lake-V", SPT)),
        // Cannon Lake, Comet Lake, Jasper Lake (cnl_i2c_info).
        0x9DE8..=0x9DEB | 0x9DC5 | 0x9DC6 => Some(("Cannon Point-LP", CNL)),
        0xA368..=0xA36B => Some(("Cannon Lake-H", CNL)),
        0x02E8..=0x02EB | 0x02C5 | 0x02C6 => Some(("Comet Lake", CNL)),
        0x06E8..=0x06EB => Some(("Comet Lake-H", CNL)),
        0x4DE8..=0x4DEB | 0x4DC5 | 0x4DC6 => Some(("Jasper Lake", CNL)),
        // Ice Lake and later (bxt_i2c_info, or spt_i2c_info on TGL-LP).
        0x34E8..=0x34EB | 0x34C5 | 0x34C6 => Some(("Ice Lake-LP", BXT)),
        0xA0E8..=0xA0EB | 0xA0C5 | 0xA0C6 | 0xA0D8 | 0xA0D9 => Some(("Tiger Lake-LP", BXT)),
        0x43E8..=0x43EB | 0x43AD | 0x43AE | 0x43D8 => Some(("Tiger Lake-H", BXT)),
        0x51E8..=0x51EB | 0x51C5 | 0x51C6 | 0x51D8 | 0x51D9 => Some(("Alder Lake-P", BXT)),
        0x54E8..=0x54EB | 0x54C5 | 0x54C6 => Some(("Alder Lake-N", BXT)),
        0x7ACC..=0x7ACF | 0x7AFC | 0x7AFD => Some(("Alder Lake-S", BXT)),
        0x7A4C..=0x7A4F | 0x7A7C | 0x7A7D => Some(("Raptor Lake-S", BXT)),
        0x7E78..=0x7E7B | 0x7E50 | 0x7E51 => Some(("Meteor Lake-P", BXT)),
        _ => None,
    }
}

/// Clock assumed for an Intel LPSS I2C function whose id is not in the
/// table (a newer platform): the fastest rate any LPSS generation uses, so
/// the bus can only come out slower than programmed.
pub const UNKNOWN_LPSS_CLOCK_HZ: u32 = 216_000_000;

/// True for Broxton, Apollo Lake and Gemini Lake, whose GPIO communities
/// number their ACPI pins as the community-local pad index with an 8-byte
/// pad configuration stride (Linux pinctrl-broxton.c, pinctrl-geminilake.c).
pub fn is_bxt_family(device: u16) -> bool {
    matches!(device_info(device), Some((_, 133_000_000)))
        && matches!(device >> 8, 0x0A | 0x1A | 0x5A | 0x31)
}
