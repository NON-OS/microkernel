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

//! The engine as the wire server drives it: BAR0, the clock its waits run
//! on, its generation, and every output it took over, pipe A's first. Wire
//! output n is the nth head; a number past them is ENXIO.

use nonos_gfx_wire::errno::ENXIO;
use nonos_gfx_wire::Errno;

use super::frames::Frames;
use super::head::Head;
use crate::ddc::pace::Pacer;
use crate::engine::access::{Clock, Mmio};
use crate::engine::gen::Gen;
use crate::engine::hotplug::Poller;

/// A flip waits for one vertical blank: 100 ms covers a 24 Hz panel with
/// room to spare, and a pipe that never blanks gives up within a FLUSH's
/// budget.
pub const FLIP_US: u64 = 100_000;
/// Gen12's four pipes, the most any family here has; the wire server's
/// outputs too.
pub const MAX_HEADS: usize = 4;

pub struct Intel<M: Mmio, F: Frames, C: Clock> {
    pub m: M,
    pub clock: C,
    pub gen: &'static Gen,
    /// The taken-over outputs, packed from the first.
    pub heads: [Option<Head<F>>; MAX_HEADS],
    /// Each output's DDC/CI timing and what its monitor last said.
    pub ddc: [Pacer; MAX_HEADS],
    /// The hot-plug lines once `watch` armed them, and when they were
    /// last polled.
    pub plugs: Option<(Poller, u64)>,
}

impl<M: Mmio, F: Frames, C: Clock> Intel<M, F, C> {
    pub fn new(m: M, clock: C, gen: &'static Gen, heads: [Option<Head<F>>; MAX_HEADS]) -> Self {
        Intel { m, clock, gen, heads, ddc: Default::default(), plugs: None }
    }

    pub fn count(&self) -> usize {
        self.heads.iter().take_while(|h| h.is_some()).count()
    }

    /// Whether another output is held beside `output`, lit or pulled.
    pub fn held_beside(&self, output: u16) -> bool {
        self.heads.iter().enumerate().any(|(i, h)| i != output as usize && h.is_some())
    }

    pub fn head(&self, output: u16) -> Result<&Head<F>, Errno> {
        self.heads.get(output as usize).and_then(|h| h.as_ref()).ok_or(ENXIO)
    }
}
