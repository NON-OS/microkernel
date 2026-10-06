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

//! The part of the family's socket table a stream outside the family goes
//! through: an entry, its backend, and the paths that let it go (release,
//! free, shutdown, adopting a stream net.anon opened). Mounted as it ships,
//! so the closes counted here are the closes the capsule sends.

#[path = "../../../../../capsule_linux/src/linux/net/sock/adopt.rs"]
mod adopt;
#[path = "../../../../../capsule_linux/src/linux/net/sock/anon_rx.rs"]
mod anon_rx;
#[path = "../../../../../capsule_linux/src/linux/net/sock/backend.rs"]
mod backend;
#[path = "../../../../../capsule_linux/src/linux/net/sock/free.rs"]
mod free;
/* What a queued datagram is charged, which bounds a receiver's queue. */
#[path = "../../../../../capsule_linux/src/linux/net/sock/gram_room.rs"]
pub mod gram_room;
#[path = "../../../../../capsule_linux/src/linux/net/sock/kinds.rs"]
mod kinds;
#[path = "../../../../../capsule_linux/src/linux/net/sock/name.rs"]
mod name;
#[path = "../../../../../capsule_linux/src/linux/net/sock/new.rs"]
mod new;
#[path = "../../../../../capsule_linux/src/linux/net/sock/opts.rs"]
mod opts;
#[path = "../../../../../capsule_linux/src/linux/net/sock/opts_more.rs"]
mod opts_more;
#[path = "../../../../../capsule_linux/src/linux/net/sock/shut_anon.rs"]
mod shut_anon;
/*
 * The port search and the waiting calls' progress live in files not mounted,
 * and the table is one static the capsule builds with `new`, never Default.
 */
#[allow(dead_code, clippy::new_without_default)]
#[path = "../../../../../capsule_linux/src/linux/net/sock/table.rs"]
mod table;
#[path = "../../../../../capsule_linux/src/linux/net/sock/types.rs"]
mod types;
#[path = "../../../../../capsule_linux/src/linux/net/sock/unlisten.rs"]
mod unlisten;

pub use anon_rx::Got;
pub use backend::{Anon, Backend, Close, End, Via};
pub use kinds::Link;
pub use table::Socks;
pub use types::{Addr, Domain, Proto, Sock};
