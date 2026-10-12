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

//! The source's transmitter as the sink sees it: the rate and width it
//! runs, the pattern it sends, the drive it uses. A wait is time passing.

use super::sink::Sink;
use crate::dp::lane::Drive;
use crate::dp::train::{Link, Pattern, TrainError};

pub struct LinkEnd(pub Sink);

impl Link for LinkEnd {
    fn start(&mut self, mbps: u32, lanes: u32) -> Result<(), TrainError> {
        let mut s = self.0.borrow_mut();
        (s.link, s.held) = (Some((mbps, lanes)), false);
        Ok(())
    }
    fn pattern(&mut self, p: Pattern) {
        self.0.borrow_mut().set_pattern(p);
    }
    fn drive(&mut self, d: Drive) {
        self.0.borrow_mut().drive = d;
    }
    fn wait_us(&mut self, us: u32) {
        self.0.borrow_mut().waited_us += us as u64;
    }
    fn stop(&mut self) {
        let mut s = self.0.borrow_mut();
        (s.link, s.held) = (None, false);
        s.pattern = Pattern::Off;
        s.drive = Drive::default();
        s.stops += 1;
    }
}
