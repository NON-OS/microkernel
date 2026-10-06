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

use super::script_states::S;
use super::script_step::step;
use super::script_temp::Temp;
use super::state::Tokenizer;

impl<'a> Tokenizer<'a> {
    /// Where the script text starting at the current position ends: the "<"
    /// of the first appropriate end tag met outside the double-escaped
    /// states, or the end of the input.
    pub(super) fn script_end(&self) -> usize {
        let b = self.src.as_bytes();
        let mut st = S::Data;
        let mut temp = Temp::default();
        let mut i = self.pos;
        while i < b.len() {
            let c = b[i];
            if c == b'/' && matches!(st, S::Lt | S::EscLt) && self.appropriate_end(i - 1) {
                return i - 1;
            }
            if st == S::Data {
                /* Plain script text only changes state at "<". */
                match b[i..].iter().position(|&x| x == b'<') {
                    Some(n) => i += n,
                    None => break,
                }
            }
            let (next, consumed) = step(st, b[i], &mut temp);
            st = next;
            if consumed {
                i += 1;
            }
        }
        b.len()
    }
}
