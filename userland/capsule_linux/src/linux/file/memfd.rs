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

/* `memfd_create` and `ftruncate`. */

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest, Kind};

use super::slot::install;

#[path = "memfd_seal.rs"]
mod memfd_seal;
#[path = "memfd_seal_maps.rs"]
mod memfd_seal_maps;
#[path = "memfd_seal_rules.rs"]
mod memfd_seal_rules;

pub use memfd_seal::{fcntl_seal, seals_start};
pub use memfd_seal_maps::write_sealed;
pub use memfd_seal_maps::{is_seal_cmd, map_sealed, memfd_write, resize_sealed, seals_keep};

pub fn is_memfd(guest: &Guest, fd: u64) -> bool {
    matches!(guest.fds.get(fd as usize), Some(e) if e.kind == Kind::Memfd)
}

pub fn memfd_create(guest: &mut Guest) -> u64 {
    match install(guest, Fd::memfd()) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}
