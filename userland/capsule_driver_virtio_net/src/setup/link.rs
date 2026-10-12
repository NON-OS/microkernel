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

use crate::constants::VIRTIO_NET_S_LINK_UP;
use crate::setup::Driver;

impl Driver {
    /// The device's word on the link when it reports link status; up
    /// otherwise, as the specification has a driver assume.
    pub fn link_up(&self) -> bool {
        !self.status_supported
            || self.transport.config_r16(self.net_status_offset) & VIRTIO_NET_S_LINK_UP != 0
    }
}
