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

//! The names the sweep uses, here the shipping server crate as this proof
//! crate mounts it.

pub use crate::display::Display;
pub use crate::platform::{Platform, Region};
pub use crate::server::Server;
pub use nonos_gfx_wire::errno::ENOMEM;
pub use nonos_gfx_wire::{
    ops, Caps, Cursor, Errno, Flush, Header, Modes, Rect, Reply, State, Surface,
};
pub use nonos_gfx_wire::{HDR_LEN, MAGIC, VERSION};
