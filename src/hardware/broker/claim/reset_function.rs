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

//! One function's config space as the reset sees it (K10a): its PCIe and
//! Advanced Features capabilities, and the header an FLR clears.

use crate::drivers::pci::config::ConfigSpace;

const CAP_PCIE: u8 = 0x10;
const CAP_AF: u8 = 0x13;

pub(super) struct Function {
    pub(super) cfg: ConfigSpace,
    pub(super) pcie: Option<u16>,
    pub(super) af: Option<u16>,
    /// The type 0 header, as saved before the reset.
    pub(super) header: [u32; 16],
    /// Device Control and Device Control 2, as saved.
    pub(super) control: (u16, u16),
}

impl Function {
    pub(super) fn new(cfg: ConfigSpace) -> Self {
        let (pcie, af) = (capability(&cfg, CAP_PCIE), capability(&cfg, CAP_AF));
        Self { cfg, pcie, af, header: [0; 16], control: (0, 0) }
    }
}

/// The capability `id` by its offset; the walk is bounded against a loop.
fn capability(cfg: &ConfigSpace, id: u8) -> Option<u16> {
    let mut at = cfg.capabilities_pointer().ok()? & 0xFC;
    for _ in 0..48 {
        if at < 0x40 {
            return None;
        }
        if cfg.read8(at as u16).ok()? == id {
            return Some(at as u16);
        }
        at = cfg.read8(at as u16 + 1).ok()? & 0xFC;
    }
    None
}
