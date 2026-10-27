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

//! What one step of a stream came to, for drive.rs to apply.

use super::entry::Entry;

pub enum Outcome {
    /// Nothing changed that the socket needs to know.
    Same,
    /// Open through net.socks5.
    Opened,
    /// Open as net.anon's stream `id`, which the socket takes on.
    Anon {
        port: u32,
        id: u16,
    },
    Failed(i64),
}

/// Not opened, with `errno`, for the reason the line will say.
pub fn fail(e: &mut Entry, errno: i64, why: &'static str) -> Outcome {
    e.why.1 = why;
    Outcome::Failed(errno)
}
