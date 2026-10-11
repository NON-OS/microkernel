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

/// What `GITS_TYPER` says the ITS can do.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ItsTyper {
    /// It translates to physical LPIs; an ITS without this is of no use here.
    pub physical: bool,
    /// Bytes per interrupt translation table entry.
    pub ite_bytes: u32,
    pub event_bits: u32,
    pub device_bits: u32,
    /// Commands name a redistributor by address, not processor number.
    pub pta: bool,
    /// Collections the ITS holds itself, needing no table in memory.
    pub hcc: u32,
}

pub const fn decode(typer: u64) -> ItsTyper {
    ItsTyper {
        physical: typer & 1 != 0,
        ite_bytes: ((typer >> 4) & 0xf) as u32 + 1,
        event_bits: ((typer >> 8) & 0x1f) as u32 + 1,
        device_bits: ((typer >> 13) & 0x1f) as u32 + 1,
        pta: typer & (1 << 19) != 0,
        hcc: ((typer >> 24) & 0xff) as u32,
    }
}
