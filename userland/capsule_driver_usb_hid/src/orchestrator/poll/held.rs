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

//! What the driver keeps for one USB host: the HID devices found through
//! it, and what its ports showed (`RootPorts`). Port and slot numbers are
//! the host's own, so each host keeps its own.

use crate::orchestrator::enumerate::{Devices, RootPorts};

pub struct Held {
    pub devs: Devices,
    pub roots: RootPorts,
}

impl Held {
    pub fn new() -> Self {
        Held { devs: Devices::new(), roots: RootPorts::new() }
    }
}
