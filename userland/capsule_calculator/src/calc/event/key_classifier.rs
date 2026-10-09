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

use crate::calc::buttons::Action;
use crate::calc::mode::Mode;
use crate::calc::op::Op;

pub enum Classified {
    Close,
    Action(Action),
    Ignored,
}

/// What a key does in `mode`. In Programmer mode the letters a to f are the
/// hex digits, as the keypad's A to F are: there they would otherwise be the
/// memory and clear keys, so typing 1C in HEX cleared the entry instead of
/// entering it. The programmer keypad has no memory keys, and Backspace still
/// clears.
pub fn classify(code: u32, mode: Mode) -> Classified {
    if code == 0x1B {
        return Classified::Close;
    }
    if code > 0x7F {
        return Classified::Ignored;
    }
    if mode == Mode::Programmer {
        if let Some(d) = (code as u8 as char).to_digit(16).filter(|d| *d >= 10) {
            return Classified::Action(Action::Digit(d as u8));
        }
    }
    match code as u8 {
        b'0'..=b'9' => Classified::Action(Action::Digit((code as u8) - b'0')),
        b'.' => Classified::Action(Action::Decimal),
        b'+' => Classified::Action(Action::Operator(Op::Add)),
        b'-' => Classified::Action(Action::Operator(Op::Sub)),
        b'*' | b'x' | b'X' => Classified::Action(Action::Operator(Op::Mul)),
        b'/' => Classified::Action(Action::Operator(Op::Div)),
        b'=' | 0x0D => Classified::Action(Action::Equals),
        b'c' | b'C' | 0x08 => Classified::Action(Action::Clear),
        b'n' | b'N' => Classified::Action(Action::Negate),
        b'%' => Classified::Action(Action::Percent),
        b'r' | b'R' => Classified::Action(Action::SquareRoot),
        b'q' | b'Q' => Classified::Action(Action::Square),
        b'i' | b'I' => Classified::Action(Action::Reciprocal),
        b'm' => Classified::Action(Action::MemoryRecall),
        b'M' => Classified::Action(Action::MemoryStore),
        b'a' | b'A' => Classified::Action(Action::MemoryAdd),
        b's' | b'S' => Classified::Action(Action::MemorySub),
        b'l' | b'L' => Classified::Action(Action::MemoryClear),
        _ => Classified::Ignored,
    }
}
