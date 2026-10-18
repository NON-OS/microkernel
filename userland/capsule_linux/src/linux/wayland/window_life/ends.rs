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

//! What ends a guest's window, and which ends take it down. Pure, so the
//! host proofs hold it.

/// What ended.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum End {
    /// xdg_toplevel.destroy of this toplevel.
    Toplevel(u32),
    /// wl_surface.destroy of this surface.
    Surface(u32),
}

/// The surface the shown window's pixels come from, and its toplevel.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Shown {
    pub surface: u32,
    pub toplevel: Option<u32>,
}

/// Whether `end` takes down the window `shown`. A closed connection or an
/// ended process takes down whatever window it had (close/close_window.rs).
pub fn takes_down(end: End, shown: Option<Shown>) -> bool {
    let Some(s) = shown else { return false };
    match end {
        End::Toplevel(t) => s.toplevel == Some(t),
        End::Surface(id) => s.surface == id,
    }
}

/// Whether closing descriptor `fd` closes the display connection, given
/// which of the process's descriptors are connected display sockets: `fd`
/// is one, and no other is. A dup keeps the connection open.
pub fn ends_connection(connected: &[bool], fd: usize) -> bool {
    connected.get(fd) == Some(&true) && connected.iter().enumerate().all(|(i, &c)| i == fd || !c)
}
