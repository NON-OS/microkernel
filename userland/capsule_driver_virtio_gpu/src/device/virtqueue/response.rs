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

//! Where a command's answer is: the staging page, after the request.

use super::control_queue::ControlQueue;
use crate::constants::VQ_STAGING_OFFSET;

impl ControlQueue {
    /// The answer, from the staging page after the `request_len` bytes of
    /// the request it answers.
    pub fn read_response(&self, request_len: usize, dst: &mut [u8]) {
        let from = VQ_STAGING_OFFSET.saturating_add(request_len);
        for (i, b) in dst.iter_mut().enumerate() {
            *b = self.layout.ring.r8(from.saturating_add(i));
        }
    }
}
