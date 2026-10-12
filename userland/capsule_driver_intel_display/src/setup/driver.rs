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

use nonos_gfx_pool::Pool;
use nonos_gfx_serve::Server;
use nonos_intel_display::display::Intel;

use crate::bar::Bar0;
use crate::clock::Uptime;
use crate::frames::DmaFrames;
use crate::handles::BrokerHandles;

pub type Engine = Intel<Bar0, DmaFrames, Uptime>;

/// A driver that came up: its broker handles, never read but held so their
/// drop gives the grants and the claim back only when the driver ends, and
/// the wire server over the taken-over output.
pub struct Driver {
    pub _handles: BrokerHandles,
    pub server: Server<Engine, Pool>,
}
