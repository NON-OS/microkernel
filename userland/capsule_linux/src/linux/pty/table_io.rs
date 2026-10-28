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

//! The two things done to a pair that raise a signal for its foreground
//! group: ^C (or ^\ or ^Z) typed at its master, and a new window size.
//! The signals wait in the table for the serve loop. Pure.

use super::table::{Table, To};
use crate::linux::abi::errno::EIO;

pub const SIGWINCH: u8 = 28;

impl Table {
    /// The master of pair `n` writes `bytes`: how many went in.
    pub fn typed_at(&mut self, n: u32, bytes: &[u8]) -> Result<usize, i64> {
        let p = self.get(n).ok_or(EIO)?;
        let (fg, (took, raised)) = (p.fg, p.master_write(bytes)?);
        self.signals.extend(raised.into_iter().filter(|_| fg != 0).map(|s| (To::Group(fg), s)));
        Ok(took)
    }

    /// TIOCSWINSZ on pair `n`: a size that changed raises SIGWINCH, as
    /// Linux's tty_do_resize does, once a foreground group is set.
    pub fn resize(&mut self, n: u32, ws: [u8; 8]) -> Result<(), i64> {
        let p = self.get(n).ok_or(EIO)?;
        let fg = Some(p.fg).filter(|&g| p.set_winsize(ws) && g != 0);
        self.signals.extend(fg.map(|g| (To::Group(g), SIGWINCH)));
        Ok(())
    }
}
