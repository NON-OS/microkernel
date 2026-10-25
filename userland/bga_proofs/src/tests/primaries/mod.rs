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

//! The primaries the shipping driver serves from (`nonos_gfx_pool`), over
//! the shim's kernel: each one's share released before its memory goes.

#[path = "../../../../nonos_gfx_pool/src/anon.rs"]
mod anon;
#[path = "../../../../nonos_gfx_pool/src/pool.rs"]
mod pool;
/// Mounted once here; the pool's share tests reach them from beside.
#[path = "../../../../nonos_gfx_pool/src/receiver.rs"]
pub(super) mod receiver;
#[path = "../../../../nonos_gfx_pool/src/surface.rs"]
pub(super) mod surface;

mod release_tests;
