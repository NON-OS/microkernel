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

//! The character sets a program can switch in: the DEC line-drawing set
//! that ncurses falls back to, and the UK set's pound sign.

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum Charset {
    #[default]
    Ascii,
    DecSpecial,
    Uk,
}

/// G0 to G3 and which of them is in use.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct Charsets {
    pub g: [Charset; 4],
    /// The set GL maps: 0 after SI, 1 after SO.
    pub gl: usize,
    /// A single shift, SS2 or SS3, for the next character only.
    pub single: Option<usize>,
}

impl Charsets {
    /// Map a printable ASCII character through the set in use.
    pub fn map(&mut self, c: char) -> char {
        let set = match self.single.take() {
            Some(i) => self.g[i],
            None => self.g[self.gl.min(3)],
        };
        match set {
            Charset::Ascii => c,
            Charset::Uk if c == '#' => '£',
            Charset::Uk => c,
            Charset::DecSpecial => dec_special(c),
        }
    }
}

pub fn designate(final_byte: u8) -> Charset {
    match final_byte {
        b'0' => Charset::DecSpecial,
        b'A' => Charset::Uk,
        _ => Charset::Ascii,
    }
}

fn dec_special(c: char) -> char {
    const TABLE: [char; 32] = [
        '\u{00A0}', '◆', '▒', '␉', '␌', '␍', '␊', '°', '±', '␤', '␋', '┘', '┐', '┌', '└', '┼', '⎺',
        '⎻', '─', '⎼', '⎽', '├', '┤', '┴', '┬', '│', '≤', '≥', 'π', '≠', '£', '·',
    ];
    match c as u32 {
        0x5F..=0x7E => TABLE[(c as u32 - 0x5F) as usize],
        _ => c,
    }
}
