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

//! The calls that shape a guest's address space: `mmap`, `munmap`, `brk`,
//! `mprotect` and `mremap`, and the ones that ask about it or lock it.

mod lock;
mod advise;
mod advise_drop;
mod map;
mod map_anon;
mod map_args;
mod map_exec;
mod map_file;
mod map_fill;
mod map_free;
mod map_kind;
mod map_place;
mod map_refused;
mod map_req;
mod memory;
mod prot;
mod prot_args;
mod prot_span;
mod prot_walk;
mod remap;
mod remap_args;
mod remap_move;
mod remap_one;
mod resident;
mod sync;

pub use lock::{mlock, mlock2, mlockall};
pub use advise::madvise;
pub use map::mmap;
pub use map_req::MapReq;
pub use memory::{brk, munmap};
pub use prot::mprotect;
pub use remap::mremap;
pub use resident::mincore;
pub use sync::msync;
