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

//! The configuration and pending tables of the redistributor every LPI is
//! delivered to. Configuration bytes start zero: every LPI disabled until
//! the ITS driver enables the one it routed.

use super::super::redistributor::GicRedistributor;
use super::encode::attrs::coherent;
use super::encode::lpi::{pend_bytes, pendbaser, prop_bytes, propbaser};
use super::memory::{alloc, free, Region};

pub(super) struct LpiTables {
    pub(super) prop: Region,
    /// Whether the redistributor snoops the configuration table; if not,
    /// each byte changed is cleaned to memory before its INV.
    pub(super) coherent: bool,
}

pub(super) fn bring_up(rd: &GicRedistributor, id_bits: u32) -> Result<LpiTables, &'static str> {
    if !rd.takes_lpis() {
        return Err("the boot CPU's redistributor takes no LPIs");
    }
    rd.stop_lpis()?;
    let prop = alloc(prop_bytes(id_bits), 0x1000).ok_or("no memory for the LPI tables")?;
    let Some(pend) = alloc(pend_bytes(id_bits), 0x1_0000) else {
        free(prop);
        return Err("no memory for the LPI tables");
    };
    let (read, _) =
        rd.set_lpi_tables(propbaser(prop.phys, id_bits, true), pendbaser(pend.phys, true));
    let coherent = coherent(read);
    if !coherent {
        rd.set_lpi_tables(propbaser(prop.phys, id_bits, false), pendbaser(pend.phys, false));
    }
    // From here the redistributor owns both tables for as long as the kernel
    // runs, which is why neither is ever freed.
    rd.start_lpis();
    Ok(LpiTables { prop, coherent })
}
