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
//! Which stream an alternate setting carries, from what was read of it.

use super::alt::Stream;

#[derive(Clone, Copy, Default)]
pub(super) struct Ep {
    pub address: u8,
    pub sync: u8,
    pub max_packet: u16,
    pub b_interval: u8,
}

#[derive(Default)]
pub(super) struct Alt {
    pub s: Stream,
    pub streaming: bool,
    pub pcm: bool,
    pub format: bool,
    pub out: Option<Ep>,
}

impl Alt {
    /// The stream this alternate plays, whatever IN endpoint (its clock
    /// feedback) it has.
    pub fn chosen(&self) -> Option<Stream> {
        let ep = self.out?;
        let usable = self.streaming && self.pcm && self.format && self.s.channels == 2;
        let (ep_address, sync, max_packet, b_interval) =
            (ep.address, ep.sync, ep.max_packet, ep.b_interval);
        usable.then_some(Stream { ep_address, sync, max_packet, b_interval, ..self.s })
    }
}
