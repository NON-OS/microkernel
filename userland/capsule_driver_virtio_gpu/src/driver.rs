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
//! A virtio-gpu that came up: its device id, which a DGON names, and the
//! wire server over its display (G-77).

use nonos_gfx_pool::Pool;
use nonos_gfx_serve::Server;

use crate::display::Gpu;

pub struct Driver {
    pub device_id: u64,
    pub server: Server<Gpu, Pool>,
}
