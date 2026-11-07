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

//! The Wayland server of the Linux personality, mounted whole at the path it
//! has in the capsule (`crate::linux::wayland`), with the rest of the
//! personality it reaches stood in for by host doubles (linux/). A session
//! (session.rs) is one client's Guest, fed request bytes and descriptors and
//! drained of events, as the capsule's sendmsg and recvmsg do.

extern crate alloc;

pub mod client;
pub mod host;
pub mod input;
pub mod linux;
pub mod record;
pub mod session;
