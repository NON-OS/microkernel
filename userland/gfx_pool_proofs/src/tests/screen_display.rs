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

//! The screen's half of the wire: one output, VGA to XGA.

use nonos_gfx_serve::{Display, Shadow};
use nonos_gfx_wire::body::caps::{MODE_RANGE, MODE_SET};
use nonos_gfx_wire::body::mode::{CURRENT, PREFERRED};
use nonos_gfx_wire::body::state::SCANNING_OUT;
use nonos_gfx_wire::errno::{EINVAL, ENOSYS};
use nonos_gfx_wire::{Caps, Cursor, Errno, Mode, Rect, State};

use super::screen::Screen;

impl Display for Screen {
    fn caps(&self) -> Caps {
        let (flags, budget) = (MODE_SET | MODE_RANGE, 1024 * 768 * 4);
        let caps = Caps {
            kind: 7,
            outputs: 1,
            flags,
            max_width: 1024,
            max_height: 768,
            ..Caps::default()
        };
        Caps { width_align: 8, surface_budget: budget, ..caps }
    }

    fn modes(&self, output: u16, out: &mut [Mode]) -> Result<usize, Errno> {
        Self::output(output)?;
        let Some(m) = out.first_mut() else { return Ok(0) };
        *m = Mode { width: self.size.0, height: self.size.1, flags: PREFERRED | CURRENT };
        Ok(1)
    }

    fn state(&mut self, output: u16) -> Result<State, Errno> {
        Self::output(output)?;
        let (width, height) = self.size;
        Ok(State { generation: 1, width, height, flags: SCANNING_OUT, ..State::default() })
    }

    fn set_mode(&mut self, output: u16, w: u32, h: u32, _: &Shadow) -> Result<(), Errno> {
        Self::output(output)?;
        let fits = (640..=1024).contains(&w) && (480..=768).contains(&h) && w.is_multiple_of(8);
        fits.then(|| self.size = (w, h)).ok_or(EINVAL)
    }

    fn flush(&mut self, output: u16, _: &[Rect], _: &Shadow) -> Result<(), Errno> {
        Self::output(output)?;
        self.flushes += 1;
        Ok(())
    }

    fn cursor(&mut self, _: u16, _: &Cursor, _: &[u8]) -> Result<(), Errno> {
        Err(ENOSYS)
    }
}
