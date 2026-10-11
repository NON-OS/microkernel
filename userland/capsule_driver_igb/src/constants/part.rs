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
//! Which igb MAC a device ID is, as Linux igb_set_mac_type decides it. The
//! reset, the receive-address table and the PHY access differ between them.

/// The port's media as its device ID names it (igb_pci_tbl's comments,
/// the Intel part names): only copper is brought up (init/media.rs).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Media {
    Copper,
    /// Fiber, SerDes, SGMII or backplane.
    Other,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Part {
    I82575,
    I82576,
    /// 82580 and the DH89xxCC, which Linux runs as an 82580.
    I82580,
    I350,
    I354,
    I210,
    I211,
}

impl Part {
    /// igb_setup_copper_link_82575 clears Go Link Disconnect on these; on an
    /// older MAC the register is not there.
    pub fn has_go_link_disconnect(self) -> bool {
        matches!(self, Part::I82580 | Part::I350 | Part::I210 | Part::I211)
    }

    /// The I210 and I211 PHY (GS40G) is paged: igb_read_phy_reg_gs40g writes
    /// the page before every access, page 0 for the IEEE registers.
    pub fn paged_phy(self) -> bool {
        matches!(self, Part::I210 | Part::I211)
    }

    /// Receive-address entries, igb_init_mac_params_82575: E1000_RAR_ENTRIES
    /// for the 82575 (and I210/I211, which keep its count), _82576, _82580,
    /// _I350 (and I354).
    pub fn rar_entries(self) -> usize {
        match self {
            Part::I82575 | Part::I210 | Part::I211 => 16,
            Part::I82576 | Part::I82580 => 24,
            Part::I350 | Part::I354 => 32,
        }
    }
}
