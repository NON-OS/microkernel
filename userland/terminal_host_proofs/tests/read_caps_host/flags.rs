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

//! Host stand-in for the terminal's command/flags, as cat calls it.

// Replaces command/flags, whose builder and accessors for valued and word
// flags cat never calls. cat's table is `-n` alone: this takes a leading
// `-n` and passes every other word as an operand, as the real parser does
// for that table.
use alloc::vec::Vec;

pub struct Spec;

impl Spec {
    pub fn new(_name: &[u8], _bools: &[u8]) -> Self {
        Spec
    }
}

pub struct Parsed<'a> {
    pub operands: Vec<&'a [u8]>,
    numbered: bool,
}

impl Parsed<'_> {
    pub fn has(&self, flag: u8) -> bool {
        flag == b'n' && self.numbered
    }
}

pub fn parse<'a>(_spec: &Spec, args: &[&'a [u8]]) -> Result<Parsed<'a>, Vec<u8>> {
    let numbered = args.first() == Some(&&b"-n"[..]);
    let operands = args[usize::from(numbered)..].to_vec();
    Ok(Parsed { operands, numbered })
}
