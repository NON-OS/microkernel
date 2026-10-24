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

//! Display power wells on Skylake (SKL PRM Vol 2c, HSW_PWR_WELL_CTL2, the
//! driver's request register; Vol 12, power well map). Power well 1 holds
//! pipe A, the eDP transcoder and DDI A and is up from boot; power well 2
//! holds pipes B and C, transcoders A to C and DDIs B to E. Each well and
//! DDI I/O has a request bit and, one below it, a state bit (bit
//! positions unverified against the volume).

use super::access::{rmw, Expiry, Mmio};
use super::error::Error;
use super::pipe::Transcoder;
use super::wait::wait_for;

pub const PWR_WELL_CTL2: u32 = 0x4_5404;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Well {
    Two,
    /// The I/O of DDI A (with E), B, C or D.
    DdiIo(u32),
}

/// The request bit; the state bit is the one below it.
pub fn request_bit(w: Well) -> u32 {
    match w {
        Well::Two => 1 << 31,
        Well::DdiIo(0) => 1 << 3,
        Well::DdiIo(d) => 1 << (3 + 2 * d.min(3)),
    }
}

/// Request `w` on and wait for its state bit.
pub fn enable(m: &mut dyn Mmio, w: Well, until: &dyn Expiry) -> Result<(), Error> {
    let req = request_bit(w);
    rmw(m, PWR_WELL_CTL2, req, req);
    wait_for(m, PWR_WELL_CTL2, req >> 1, req >> 1, until)
}

/// The wells an output on `t` driving DDI `ddi` needs, besides well 1.
pub fn needed(t: Transcoder, ddi: u32) -> [Option<Well>; 2] {
    let two = (t != Transcoder::Edp || ddi != 0).then_some(Well::Two);
    [two, Some(Well::DdiIo(ddi))]
}
