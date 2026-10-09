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

use super::cursor::Cur;
use super::ident::ident;
use super::list::list;
use super::list_kind::Kind;

/* Identifier arguments up to the closing parenthesis: exactly one, or with
 * `many` one or more separated by whitespace; `star` also accepts '*'. */
pub(super) fn words(c: &mut Cur, many: bool, star: bool) -> Option<()> {
    let mut n = 0;
    loop {
        c.trivia();
        if c.close() {
            return (n > 0).then_some(());
        }
        if n > 0 && !many {
            return None;
        }
        if !(star && c.eat(b'*')) {
            ident(c)?;
        }
        n += 1;
    }
}

/* One identifier argument and the closing parenthesis, for :lang(), :dir()
 * and :state(). */
pub(super) fn ident_arg(c: &mut Cur) -> Option<String> {
    c.trivia();
    let v = ident(c)?;
    c.trivia();
    c.close().then_some(v)
}

/* A single compound selector argument, as ::slotted(), ::cue() and :host()
 * take; its specificity is returned packed. */
pub(super) fn compound_arg(c: &mut Cur) -> Option<u32> {
    let (sels, spec) = list(c, Kind::Compound)?;
    (sels.len() == 1).then_some(spec)
}
