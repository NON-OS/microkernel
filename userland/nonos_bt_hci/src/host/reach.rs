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

//! What a caller may reach of the host: its transport, and how many ACL
//! buffers the controller has free.

use super::state::Host;
use crate::transport::Transport;

impl<T: Transport> Host<T> {
    pub fn transport(&mut self) -> &mut T {
        &mut self.t
    }

    /// ACL buffers free on the controller now.
    pub fn acl_free(&self) -> u16 {
        self.acl_free
    }
}
