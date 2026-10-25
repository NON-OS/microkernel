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
//! The search a network driver runs: the scanner, handed the station
//! address drawn once a run. net.core asks for the address only when its
//! bound port changes, so a rebind after a loss keeps it. No port is
//! addressed before there is an address to hand its bind.

use super::bound::Bound;
use super::scanner::Scanner;
use crate::run::BindFn;
use crate::say::say;
use crate::station::{draw, Station};

pub struct NetScanner {
    scanner: Scanner,
    station: Option<Station>,
}

impl NetScanner {
    pub fn new(tag: &[u8]) -> Self {
        let station = draw();
        if station.is_none() {
            say(tag, &[b"no entropy for a station address, no device bound until there is"]);
        }
        Self { scanner: Scanner::new(), station }
    }

    pub fn pass<N>(&mut self, tag: &[u8], bind: BindFn<N>) -> Option<Bound<N>> {
        self.station = self.station.or_else(draw);
        let station = self.station?;
        self.scanner.pass(tag, |bus, found| bind(bus, found, station))
    }

    pub fn lost(&mut self, xhci: u32, port: u8) {
        self.scanner.lost(xhci, port);
    }
}
