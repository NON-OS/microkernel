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

/*
 * One key applied to a line being typed, for every step that takes text:
 * the network step's passphrase and the name step. Printable keys are
 * characters here, so j, k and digits go into the line rather than moving
 * a list. `check` sees the line so far and the byte, and says why it may
 * not go next; a full line takes nothing more.
 */

use crate::server::step::{K_ENTER, K_ENTER_LF, K_ESC};

const K_BACKSPACE: u32 = 0x08;
const K_DELETE: u32 = 0x7F;

pub enum Typed<R> {
    /* A byte went in or came out. */
    Changed,
    /* The byte was kept out, and why. */
    Refused(R),
    Enter,
    Esc,
    /* A key that means nothing while typing. */
    Ignored,
}

pub fn key<R>(
    line: &mut [u8],
    len: &mut usize,
    code: u32,
    check: impl Fn(&[u8], u8) -> Result<(), R>,
) -> Typed<R> {
    match code {
        K_ENTER | K_ENTER_LF => Typed::Enter,
        K_ESC => Typed::Esc,
        K_BACKSPACE | K_DELETE if *len > 0 => {
            *len -= 1;
            line[*len] = 0;
            Typed::Changed
        }
        0x20..=0x7E => match check(&line[..*len], code as u8) {
            Ok(()) if *len < line.len() => {
                line[*len] = code as u8;
                *len += 1;
                Typed::Changed
            }
            Ok(()) => Typed::Ignored,
            Err(why) => Typed::Refused(why),
        },
        _ => Typed::Ignored,
    }
}
