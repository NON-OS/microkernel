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

//! A parser handler that writes down what it was told, one entry a
//! report, so a test can compare the sequence.

use nonos_vt::parser::{Handler, Parser, Seq};

#[derive(Default)]
pub struct Log(pub Vec<String>);

impl Handler for Log {
    fn print(&mut self, c: char) {
        self.0.push(format!("p{c}"));
    }
    fn execute(&mut self, b: u8) {
        self.0.push(format!("x{b:02x}"));
    }
    fn csi(&mut self, s: &Seq) {
        let g: Vec<String> = s.params.groups().map(|g| format!("{g:?}")).collect();
        let i = String::from_utf8_lossy(s.inter()).to_string();
        self.0.push(format!("csi{}{i}{}{}", s.private as char, s.final_byte as char, g.join("")));
    }
    fn esc(&mut self, s: &Seq) {
        let i = String::from_utf8_lossy(s.inter()).to_string();
        self.0.push(format!("esc{i}{}", s.final_byte as char));
    }
    fn osc(&mut self, d: &[u8]) {
        self.0.push(format!("osc{}", String::from_utf8_lossy(d)));
    }
    fn dcs(&mut self, s: &Seq, d: &[u8]) {
        let i = String::from_utf8_lossy(s.inter()).to_string();
        self.0.push(format!("dcs{i}{}{}", s.final_byte as char, String::from_utf8_lossy(d)));
    }
}

pub fn run(bytes: &[u8]) -> Vec<String> {
    let mut p = Parser::new();
    let mut l = Log::default();
    p.feed(&mut l, bytes);
    l.0
}
