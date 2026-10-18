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

//! The VESA DMT modes, from the data file that holds the standard's
//! table, checked and laid out when the crate compiles.

use crate::dmt_parse::{parse, Table};
use nonos_edid::Detailed;

/// Where a DMT mode's timing comes from (the data file's last column).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// The DMT's own timing, from before CVT.
    Own,
    /// CVT standard blanking.
    Cvt,
    /// CVT reduced blanking, version 1.
    Reduced,
    /// CVT reduced blanking, version 2.
    ReducedV2,
}

/// One DMT mode.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dmt {
    pub id: u8,
    pub hz: u32,
    pub kind: Kind,
    pub timing: Detailed,
}

static TABLE: Table = parse(include_bytes!("../data/dmt.txt"));

/// Every DMT mode, in the data file's order.
pub fn all() -> &'static [Dmt] {
    &TABLE.modes[..TABLE.len]
}

/// The DMT mode of this size and nominal rate, with reduced blanking or
/// without it.
pub fn dmt(width: u32, height: u32, hz: u32, reduced: bool) -> Option<Dmt> {
    all().iter().copied().find(|m| {
        let rb = matches!(m.kind, Kind::Reduced | Kind::ReducedV2);
        m.timing.h_active == width && m.timing.v_active == height && m.hz == hz && rb == reduced
    })
}
