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
//! Who holds the line. One client at a time opens it; the rest are turned
//! away until it closes, or until the kernel says it is gone, so a client
//! that died holding the line does not keep it from the next.

/// The caller does not hold the line.
pub const E_PERM: i32 = -1;
pub const E_BUSY: i32 = -16;

#[derive(Default)]
pub struct Owner {
    pub pid: Option<u32>,
}

impl Owner {
    /// OPEN by `peer`; `alive` asks the kernel about the holder.
    pub fn open(&mut self, peer: u32, alive: impl Fn(u32) -> bool) -> Result<(), i32> {
        match self.pid {
            Some(o) if o != peer && alive(o) => Err(E_BUSY),
            _ => {
                self.pid = Some(peer);
                Ok(())
            }
        }
    }

    /// Every other op: the holder's alone.
    pub fn own(&self, peer: u32) -> Result<(), i32> {
        if self.pid == Some(peer) {
            Ok(())
        } else {
            Err(E_PERM)
        }
    }

    pub fn close(&mut self) {
        self.pid = None;
    }
}
