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

//! The parser's reports, each routed to the part of the terminal that acts
//! on it.

use super::state::Term;
use crate::parser::{Handler, Seq};

impl Handler for Term {
    fn print(&mut self, c: char) {
        self.print_char(c);
    }

    fn execute(&mut self, b: u8) {
        self.control(b);
    }

    fn csi(&mut self, seq: &Seq) {
        self.csi_dispatch(seq);
    }

    fn esc(&mut self, seq: &Seq) {
        self.esc_dispatch(seq);
    }

    fn osc(&mut self, data: &[u8]) {
        self.osc_dispatch(data);
    }

    fn dcs(&mut self, seq: &Seq, data: &[u8]) {
        self.dcs_dispatch(seq, data);
    }
}
