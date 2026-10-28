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

//! The far end of each circuit Net opened: what it was sent, and its answers.

use std::boxed::Box;
use std::vec::Vec;

use crate::cell::{body, pack, unpack, Cell, Frame, RelayHeader, CELL_RELAY};
use crate::circuit::{open, seal};
use crate::manager::pump_tick;

use super::exit_net::Net;

pub const BEGIN: u8 = 1;
pub const DATA: u8 = 2;
pub const END: u8 = 3;
pub const CONNECTED: u8 = 4;

impl Net {
    /// The relay messages sent since the last look, as the exits read them:
    /// circuit, command, stream and body. Cells to other circuits are passed.
    pub fn look(&mut self) -> Vec<(u32, u8, u16, Vec<u8>)> {
        let sent = core::mem::take(&mut self.state.link.as_mut().expect("link up").sent);
        let mut out = Vec::new();
        for mut cell in sent.iter().map(|bytes| Cell::decode(bytes).expect("whole cells")) {
            let Some((_, far)) = self.far.iter_mut().find(|(id, _)| *id == cell.circuit) else {
                continue;
            };
            if cell.command == CELL_RELAY
                && open(core::slice::from_mut(far), &mut cell.payload).is_some()
            {
                let (h, said) = (unpack(&cell.payload), body(&cell.payload).unwrap().to_vec());
                out.push((cell.circuit, h.command, h.stream, said));
            }
        }
        out
    }

    /// The exit at the end of `circuit` answers stream `id`; the pump reads it.
    pub fn answer(&mut self, circuit: u32, command: u8, id: u16, said: &[u8]) {
        let far = &mut self.far.iter_mut().find(|(c, _)| *c == circuit).expect("Net's own").1;
        let header = RelayHeader { command, recognized: 0, stream: id, length: said.len() as u16 };
        let mut cell = Cell::new(circuit, CELL_RELAY);
        cell.payload = pack(&header, said).expect("fits a cell");
        seal(core::slice::from_mut(far), 0, &mut cell.payload).expect("one hop");
        self.state.link.as_mut().unwrap().inbound.push_back(Frame::Fixed(Box::new(cell)));
        pump_tick(&mut self.state, self.now);
    }
}
