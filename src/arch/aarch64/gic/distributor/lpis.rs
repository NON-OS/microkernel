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

use super::constants::GICD_TYPER;
use super::device::GicDistributor;

/// `GICD_TYPER.LPIS`: the GIC implements LPIs at all.
const TYPER_LPIS: u32 = 1 << 17;

impl GicDistributor {
    /// How many INTID bits the GIC implements, `None` when it has no LPIs
    /// and an ITS behind it could raise nothing.
    pub fn lpi_id_bits(&self) -> Option<u32> {
        let typer = self.read_reg(GICD_TYPER);
        (typer & TYPER_LPIS != 0).then_some(((typer >> 19) & 0x1f) + 1)
    }
}
