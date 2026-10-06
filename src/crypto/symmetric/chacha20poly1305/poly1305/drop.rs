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

use super::types::Poly1305;
use crate::crypto::constant_time::{compiler_fence, volatile_write};

impl Drop for Poly1305 {
    fn drop(&mut self) {
        volatile_write(&mut self.h, [0; 3]);
        volatile_write(&mut self.r, [0; 3]);
        volatile_write(&mut self.pad, [0; 2]);
        volatile_write(&mut self.buffer, [0; 16]);
        volatile_write(&mut self.buffer_len, 0);
        compiler_fence();
    }
}
