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
//! A UAS interface's pipes, as pipes.rs reads them.

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Pipe {
    pub address: u8,
    pub max_packet: u16,
    pub burst: u8,
    /// Streams it takes as 2^n; 0, none.
    pub streams: u8,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct UasPipes {
    pub interface: u8,
    pub alt: u8,
    pub command: Pipe,
    pub status: Pipe,
    pub data_in: Pipe,
    pub data_out: Pipe,
}

impl UasPipes {
    pub(crate) fn whole(&self) -> bool {
        let p = [self.command, self.status, self.data_in, self.data_out];
        p.iter().all(|p| p.address != 0)
    }
}
