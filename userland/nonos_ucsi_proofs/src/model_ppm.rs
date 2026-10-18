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

//! The model reached as the session reaches a real PPM: through the
//! mailbox and the _DSM's two signals.

use crate::model::Model;
use crate::ppm::Ppm;

impl Ppm for Model {
    fn write_control(&mut self, control: u64) -> bool {
        self.control = Some((control, 0));
        true
    }
    fn read_cci(&mut self) -> Option<u32> {
        Some(self.cci | self.shown())
    }
    fn read_message(&mut self, out: &mut [u8; 16]) -> bool {
        *out = self.message;
        true
    }
    fn signal_write(&mut self) -> bool {
        self.commands.push(self.control.map_or(0, |(c, _)| c as u8));
        self.cci &= !(0xFF << 24);
        true
    }
    fn signal_read(&mut self) -> bool {
        self.reads += 1;
        self.advance();
        true
    }
    fn pause(&mut self) {
        self.pauses += 1;
    }
}
