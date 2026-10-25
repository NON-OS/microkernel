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

//! The adapter as a `nonos_gfx_serve::Display`: one output, so the output
//! number the server checked against CAPS is not looked at again.

use super::svga::Svga;
use super::{describe, modes};
use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::errno::ENOSYS;
use nonos_gfx_wire::{Caps, Cursor, Errno, Mode, Rect, State};

impl Display for Svga {
    fn caps(&self) -> Caps {
        describe::caps(self)
    }

    fn modes(&self, _output: u16, out: &mut [Mode]) -> Result<usize, Errno> {
        Ok(modes::modes(self, out))
    }

    fn state(&mut self, _output: u16) -> Result<State, Errno> {
        let flags = self.health();
        Ok(describe::state(self, flags))
    }

    fn set_mode(&mut self, _output: u16, w: u32, h: u32, shadow: &Shadow) -> Result<(), Errno> {
        self.program(w, h)?;
        self.copy(&[Rect { x: 0, y: 0, width: w, height: h }], shadow)
    }

    fn flush(&mut self, _output: u16, rects: &[Rect], shadow: &Shadow) -> Result<(), Errno> {
        self.copy(rects, shadow)
    }

    fn cursor(&mut self, _output: u16, _c: &Cursor, _slot: &[u8]) -> Result<(), Errno> {
        Err(ENOSYS)
    }
}
