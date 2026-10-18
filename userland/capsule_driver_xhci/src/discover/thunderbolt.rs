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

const VENDOR_INTEL: u16 = 0x8086;
/// Intel Thunderbolt and USB4 xHCI functions (as observed, unverified):
/// Alpine Ridge, Titan Ridge, Maple Ridge, and the TCSS controllers of Ice
/// Lake, Tiger Lake, Alder Lake, Raptor Lake and Meteor Lake. Their ports
/// are the Type-C ones only.
const INTEL_THUNDERBOLT_XHCI: [u16; 15] = [
    0x15b5, 0x15b6, 0x15c1, 0x15d4, 0x15db, 0x15e9, 0x15ec, 0x15f0, 0x1138, 0x8a13, 0x9a13, 0x461e,
    0x464e, 0xa71e, 0x7ec0,
];

pub fn is_thunderbolt(vendor: u16, device: u16) -> bool {
    vendor == VENDOR_INTEL && INTEL_THUNDERBOLT_XHCI.contains(&device)
}
