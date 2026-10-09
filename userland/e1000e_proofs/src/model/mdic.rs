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

//! The MDIC register as the part serves it: a pending command is handed to
//! the PHY and its finished word put back.

use nonos_devmodel::FakeBar;

use super::phy::Phy;
use crate::constants::phy::{MDIC_OP_READ, MDIC_OP_WRITE, MDIC_READY};
use crate::constants::regs::REG_MDIC;

/// A command is a word with an op and no READY. It is read twice, so a
/// word caught half-written by the driver is not taken for a command.
pub fn serve(bar: &FakeBar, phy: &Phy) {
    let cmd = bar.wrote32(REG_MDIC);
    let pending = cmd & (MDIC_OP_READ | MDIC_OP_WRITE) != 0 && cmd & MDIC_READY == 0;
    if pending && bar.wrote32(REG_MDIC) == cmd {
        bar.present32(REG_MDIC, phy.run(cmd));
    }
}
