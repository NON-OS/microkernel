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

//! What the root port scan keeps per port: the tries that came to nothing
//! since the port was last empty, and its `Budget`, the tries of the last
//! minute counted across disconnects. A device that reconnected as fast as
//! it was looked at was addressed again at every rescan, the loop held for
//! each, and a hub posing as one was powered up each time as well.

use crate::hub::Budget;

pub struct RootPorts {
    pub tries: [u8; 256],
    pub budget: [Budget; 256],
}

impl RootPorts {
    pub fn new() -> Self {
        Self { tries: [0; 256], budget: [Budget::new(); 256] }
    }
}
