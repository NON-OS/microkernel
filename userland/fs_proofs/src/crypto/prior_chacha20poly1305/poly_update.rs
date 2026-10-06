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

//! Prior Poly1305 buffering of partial blocks.

use super::poly_new::Poly1305;

impl Poly1305 {
    pub fn update(&mut self, mut data: &[u8]) {
        if self.buffer_len > 0 {
            let take = core::cmp::min(16 - self.buffer_len, data.len());
            self.buffer[self.buffer_len..self.buffer_len + take].copy_from_slice(&data[..take]);
            self.buffer_len += take;
            data = &data[take..];
            if self.buffer_len == 16 {
                let buf_copy = self.buffer;
                self.block(&buf_copy, 1 << 24);
                self.buffer_len = 0;
            }
        }
        while data.len() >= 16 {
            self.block(&data[..16], 1 << 24);
            data = &data[16..];
        }
        if !data.is_empty() {
            self.buffer[..data.len()].copy_from_slice(data);
            self.buffer_len = data.len();
        }
    }
}
