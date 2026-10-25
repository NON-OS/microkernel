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

//! Which ACS a function has, by who made it, pure.
//! kernel_proofs::acs_group::layout_tests.

/// How a function's ACS is read and turned on.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Layout {
    /// PCIe 6.0, 7.7.11: capability and control words at +4 and +6.
    Standard,
    /// Capability and control as dwords, control at +8.
    SptDword,
    /// Redirects whatever its registers say.
    Rciep,
}

const INTEL: u16 = 0x8086;
/// PCI Express capability, device/port type (PCIe 6.0, 7.5.3.2).
const ROOT_PORT: u8 = 0x4;
const RC_END: u8 = 0x9;

/*
 * Sunrise Point (100 series), Union Point (200 series) and 7th/8th-gen
 * mobile PCH root ports keep ACS in dwords, control at +8 instead of +6
 * (100 Series PCH datasheet vol. 2, 12.1.46-47; spec update erratum 23;
 * 200 series spec update erratum 22). The ids are those parts' root ports,
 * from the same datasheets.
 */
const fn spt_root_port(device: u16) -> bool {
    matches!(device, 0xA110..=0xA11F | 0xA167..=0xA16A | 0xA290..=0xA29F | 0xA2E7..=0xA2EE)
        || matches!(device, 0x9D10..=0x9D1B)
}

/// The layout of the function with these ids and PCI Express type, None
/// for one without the capability, on `bus`. An Intel root complex
/// integrated endpoint on bus 0 forwards peer requests only on translated
/// addresses (VT-d r3.1, 3.16), so it counts as redirecting.
pub(crate) const fn layout(vendor: u16, device: u16, kind: Option<u8>, bus: u8) -> Layout {
    match kind {
        Some(ROOT_PORT) if vendor == INTEL && spt_root_port(device) => Layout::SptDword,
        Some(RC_END) if vendor == INTEL && bus == 0 => Layout::Rciep,
        _ => Layout::Standard,
    }
}
