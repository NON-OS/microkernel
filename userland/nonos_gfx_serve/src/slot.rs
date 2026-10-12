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

//! One output's primaries: the current one, and after a MODE_SET the one
//! it replaced, until the compositor sends RETIRE for it.

use nonos_gfx_wire::errno::ENXIO;
use nonos_gfx_wire::Errno;

use super::display::Display;
use super::platform::Platform;
use super::primary::Primary;
use super::server::Server;

/// The most outputs a server keeps primaries for: Intel's Gen12 has four
/// pipes, the most of any device served today.
pub const MAX_OUTPUTS: usize = 4;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Slot {
    pub current: Option<Primary>,
    pub retiring: Option<Primary>,
    /// The device reported the output lost and putting the mode back has
    /// not worked yet: every STATE tries again.
    pub lost: bool,
}

impl<D: Display, P: Platform> Server<D, P> {
    /// The slot of `output`; ENXIO past the device's outputs or ours.
    pub fn slot(&mut self, output: u16) -> Result<&mut Slot, Errno> {
        if output >= self.display.caps().outputs {
            return Err(ENXIO);
        }
        self.slots.get_mut(output as usize).ok_or(ENXIO)
    }

    /// Free the primary of `output` waiting for RETIRE, if any.
    pub fn drop_retiring(&mut self, output: usize) {
        if let Some(r) = self.slots[output].retiring.take() {
            self.platform.release(Some(r.handle), r.region);
        }
    }
}
