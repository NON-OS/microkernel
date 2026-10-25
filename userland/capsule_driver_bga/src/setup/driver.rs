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

use nonos_gfx_serve::{Platform, Server};

use crate::display::Bga;
use crate::handles::BrokerHandles;

/// A driver that came up: its broker handles, read for the adapter's id (a
/// DGON names it) and held so their drop writes the firmware's DISPI state
/// back, gives up both mappings and releases the claim, only when the driver
/// ends; and the wire server over the adapter, its primaries on `P` (the
/// shipping driver's `nonos_gfx_pool::Pool`, which asks the kernel).
pub struct Driver<P: Platform> {
    pub handles: BrokerHandles,
    pub server: Server<Bga, P>,
}
