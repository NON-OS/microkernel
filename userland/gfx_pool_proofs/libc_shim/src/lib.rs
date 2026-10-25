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

//! What the included pool files take from `nonos_libc`, on the host: the
//! anonymous mappings (host memory, handed out poisoned, so only the
//! driver's own zeroing makes a primary zero), the driver's pid, the
//! compositor's service, and the surface registry, modelled on the kernel's
//! rules as its handlers state them. A test reads back what is mapped and
//! what is registered, and may cue a call to refuse.

mod cues;
mod mem;
mod mem_read;
mod registry;
mod service;
mod share;
pub mod surface_registry;

pub use cues::{live, refuse_registers, refuse_shares, registered_dirty, reset};
pub use mem::{mk_mmap, mk_munmap, POISON};
pub use mem_read::{maps, refuse_maps, unmapped_wrong};
pub use registry::{mk_surface_register, mk_surface_release, Entry};
pub use service::{mk_getpid, mk_service_lookup, set_compositor, OWN_PID};
pub use share::mk_surface_share;
pub use surface_registry::{SurfaceDescriptor, SURFACE_FORMAT_ARGB8888};
