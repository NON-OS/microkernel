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

//! The personality around the Wayland server, as far as the server reaches
//! it: the guest it serves (guest.rs), descriptors and memory (file.rs),
//! clocks and pipes (call.rs), the console (say.rs). The connection's queues
//! and the size rule are the shipped files.

pub mod call;
pub mod file;
pub mod guest;
pub mod guest_reads;
pub mod say;

pub mod start_say {
    pub mod clock {
        /// The first window's time on the boot clock; the host keeps none.
        pub(crate) fn window() {}
    }
}

/// net/'s side the server calls. The host serves no listener, so the input
/// pump reads the inbox itself, nothing is kept aside for it, and no routed
/// stream waits on input.
pub mod net {
    pub fn input_delivered() {}

    pub mod serve_one {
        pub fn serving() -> bool {
            false
        }

        /// Nothing is kept aside where nothing is served.
        pub fn take_aside(_into: &mut [u8]) -> Option<(usize, u32)> {
            None
        }
    }
}

#[path = "../../../capsule_linux/src/linux/unix/conn.rs"]
mod conn;

pub mod unix {
    pub use super::conn::Conn;

    /// unix/passed.rs's side the server calls. The session gives every
    /// descriptor it receives a slot of its own (session.rs), and writes a
    /// pipe's bytes through that slot after the request is served, so a
    /// slot is left as it is here: closed in the capsule, it would drop the
    /// text a paste wrote before the session wrote it out.
    pub mod passed {
        pub fn drop(_guest: &mut crate::linux::guest::Guest, _fd: u32) {}
    }
}

#[path = "../../../capsule_linux/src/linux/whole.rs"]
pub mod whole;

#[path = "../../../capsule_linux/src/linux/wayland/mod.rs"]
pub mod wayland;
