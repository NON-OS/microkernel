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

use alloc::string::String;

use crate::browser::css::selector::{AttrOp, AttrTest, Simple};

use super::attr_parts::{attr_name, attr_op, bare};
use super::cursor::Cur;
use super::ident::{ident, starts_ident};
use super::string::string;

/* One [..] attribute selector, the cursor on its '['. The name may carry a
 * namespace prefix (taken as any namespace), the value is an identifier or
 * a string, and an i or s flag may follow it. The end of the text closes a
 * missing ']', as CSS closes every open block at the end of input. */
pub(super) fn attr(c: &mut Cur, s: &mut Simple) -> Option<()> {
    c.i += 1;
    c.trivia();
    let name = attr_name(c)?.to_ascii_lowercase();
    c.trivia();
    let mut test = AttrTest { op: AttrOp::Present, value: String::new(), case_insensitive: false };
    if !(c.eat(b']') || c.peek().is_none()) {
        test.op = attr_op(c)?;
        c.trivia();
        test.value = if matches!(c.peek(), Some(b'"' | b'\'')) { string(c)? } else { bare(c)? };
        c.trivia();
        if starts_ident(c) {
            test.case_insensitive = match ident(c)?.as_str() {
                "i" | "I" => true,
                "s" | "S" => false,
                _ => return None,
            };
            c.trivia();
        }
        if !(c.eat(b']') || c.peek().is_none()) {
            return None;
        }
    }
    s.attrs.push((name, test));
    Some(())
}
