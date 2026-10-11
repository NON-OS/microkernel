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

//! The index and value ports, answered by this thread's adapter model.

use crate::model::with_svga;

#[derive(Clone, Copy)]
pub struct Port {
    _grant: u64,
}

impl Port {
    pub const fn new(grant: u64) -> Self {
        Self { _grant: grant }
    }

    pub fn read32(&self, port: u16) -> u32 {
        assert_eq!(port, 1, "only the value port reads");
        with_svga(|s| s.read(s.index))
    }

    pub fn write32(&self, port: u16, value: u32) {
        with_svga(|s| match port {
            0 => s.index = value,
            1 => s.write(s.index, value),
            _ => panic!("no port {port}"),
        });
    }
}
