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

//! The contract a generation's mode setting meets. The capsule holds one
//! for its function's generation and calls it on MODE_SET and on the
//! hot-plug poll; the generation's code does the programming.

use nonos_edid::Detailed;

use crate::access::{Clock, Mmio};
use crate::error::Error;
use crate::pipe::{Pipe, Transcoder};

/// One lit path: the pipe, the transcoder feeding its port, and the port
/// (DDI) as the PRM letters it, A as 0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Output {
    pub pipe: Pipe,
    pub trans: Transcoder,
    pub ddi: u8,
}

/// Mode setting for one generation. Each call checks everything it can
/// before its first write, and an `Err` leaves the output as it found it
/// or dark, never half programmed.
pub trait ModeSet {
    /// Drive `out` at `timing`, already checked against the monitor's
    /// limits (`nonos_modes`).
    fn set(
        &mut self,
        m: &mut dyn Mmio,
        c: &dyn Clock,
        out: Output,
        timing: &Detailed,
    ) -> Result<(), Error>;
    /// Darken `out`: plane, pipe, transcoder, port and its clock off.
    fn off(&mut self, m: &mut dyn Mmio, c: &dyn Clock, out: Output) -> Result<(), Error>;
    /// Ports whose hot-plug state changed since the last poll, bit n for
    /// DDI n.
    fn hpd_poll(&mut self, m: &mut dyn Mmio) -> u8;
}
