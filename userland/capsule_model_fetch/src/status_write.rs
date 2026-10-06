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

/* The fetcher's side of `status_wire`: an answer as bytes. */

use crate::status_wire::{Status, LEN, MAGIC};

impl Status {
    pub fn encode(&self) -> [u8; LEN] {
        let mut out = [0u8; LEN];
        out[..4].copy_from_slice(&MAGIC);
        (out[4], out[5], out[6], out[7]) = (self.stage, self.route, self.try_n, self.tries);
        for (i, v) in [self.total, self.done, self.rate].iter().enumerate() {
            out[8 + i * 8..16 + i * 8].copy_from_slice(&v.to_le_bytes());
        }
        out
    }
}
