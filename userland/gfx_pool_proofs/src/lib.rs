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

//! nonos_gfx_pool, proved on the host. Each `#[path]` pulls in the shipping
//! pool source, so these tests run the code every display driver's
//! primaries live in: anonymous pages zeroed when made and when scrubbed,
//! registered and shared with the compositor alone, and given back whole.
//! `libc_shim` stands in for the kernel's mapping and surface registry
//! calls, by the rules the kernel's handlers state, and hands out mappings
//! poisoned, so only the pool's own zeroing makes a primary zero.

#[path = "../../nonos_gfx_pool/src/anon.rs"]
mod anon;
#[path = "../../nonos_gfx_pool/src/pool.rs"]
pub mod pool;
#[path = "../../nonos_gfx_pool/src/receiver.rs"]
mod receiver;
#[path = "../../nonos_gfx_pool/src/surface.rs"]
mod surface;

pub use pool::Pool;

#[cfg(test)]
mod tests;
