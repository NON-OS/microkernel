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

//! The functions the vendor-bit proofs ask about.

use super::quirk_bits::{Ident, Route};

pub(super) const HDA: Ident = Ident {
    vendor: 0x8086,
    class: 0x04,
    subclass: 0x03,
    prog_if: 0,
    pcie_cap: None,
    before: [0xFF; 3],
    device: 0,
    route: Route::NONE,
};
pub(super) const DSP: Ident = Ident {
    vendor: 0x8086,
    class: 0x04,
    subclass: 0x01,
    prog_if: 0,
    pcie_cap: None,
    before: [0xFF; 3],
    device: 0,
    route: Route::NONE,
};
pub(super) const AMD_HDA: Ident = Ident {
    vendor: 0x1022,
    class: 0x04,
    subclass: 0x03,
    prog_if: 0,
    pcie_cap: None,
    before: [0xFF; 3],
    device: 0,
    route: Route::NONE,
};
pub(super) const WIFI: Ident = Ident {
    vendor: 0x10EC,
    class: 0x02,
    subclass: 0x80,
    prog_if: 0,
    pcie_cap: Some(0x70),
    before: [0xFF; 3],
    device: 0,
    route: Route::NONE,
};
pub(super) const NVME: Ident = Ident {
    vendor: 0x8086,
    class: 0x01,
    subclass: 0x08,
    prog_if: 0,
    pcie_cap: Some(0x70),
    before: [0xFF; 3],
    device: 0,
    route: Route::NONE,
};
