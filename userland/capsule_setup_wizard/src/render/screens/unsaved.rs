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

//! The answers the settings service did not take when setup applied them.
//!
//! Setup hands each answer to the policy store in turn. One the store
//! refuses, or every one when there is no store to ask, is not in force for
//! this session, and saying nothing left the person believing it was: a Qwen
//! tier chosen and then the small one run, a keyboard picked and another in
//! use. Each refused answer sets its bit, and the review screen names them.
//!
//! Pure. Held in setup_layout_proofs.

pub const KEYBOARD: u16 = 1 << 0;
pub const TIME_ZONE: u16 = 1 << 1;
pub const WALLPAPER: u16 = 1 << 2;
pub const NAME: u16 = 1 << 3;
pub const QWEN: u16 = 1 << 4;
pub const APPS: u16 = 1 << 5;
pub const NETWORK: u16 = 1 << 6;
pub const COMPUTER: u16 = 1 << 7;
pub const PERSISTENCE: u16 = 1 << 8;
/// Every answer: what is left unsaved when there is no settings service.
pub const ALL: u16 = (1 << 9) - 1;

const NAMES: [(u16, &[u8]); 9] = [
    (KEYBOARD, b"Keyboard"),
    (TIME_ZONE, b"Time zone"),
    (WALLPAPER, b"Wallpaper"),
    (NAME, b"Name"),
    (QWEN, b"Qwen model"),
    (APPS, b"Apps"),
    (NETWORK, b"Network"),
    (COMPUTER, b"Computer"),
    (PERSISTENCE, b"Keeping"),
];

const HEAD: &[u8] = b"Not applied to this session: ";
/// With no store at all nothing was applied, said in one line.
const NONE_TAKEN: &[u8] = b"Nothing was applied to this session: there is no settings service";
/// Names on the first line; the rest go on a second, so both fit the column.
const FIRST_LINE: usize = 4;

/// The bit an answer sets when the store refuses it.
pub fn note(unsaved: &mut u16, bit: u16, written: Result<(), i32>) {
    if written.is_err() {
        *unsaved |= bit;
    }
}

fn put(out: &mut [u8], n: &mut usize, b: &[u8]) {
    let end = (*n + b.len()).min(out.len());
    out[*n..end].copy_from_slice(&b[..end - *n]);
    *n = end;
}

/// The lines naming every refused answer, into `first` and `rest`, and
/// their lengths; (0, 0) when every answer was taken. 96 bytes each hold
/// the longest.
pub fn said(unsaved: u16, first: &mut [u8], rest: &mut [u8]) -> (usize, usize) {
    let unsaved = unsaved & ALL;
    if unsaved == 0 {
        return (0, 0);
    }
    let (mut a, mut b) = (0, 0);
    if unsaved == ALL {
        put(first, &mut a, NONE_TAKEN);
        return (a, 0);
    }
    put(first, &mut a, HEAD);
    let mut count = 0;
    for (bit, name) in NAMES {
        if unsaved & bit == 0 {
            continue;
        }
        let (out, n) =
            if count < FIRST_LINE { (&mut *first, &mut a) } else { (&mut *rest, &mut b) };
        if count != 0 && count != FIRST_LINE {
            put(out, n, b", ");
        }
        put(out, n, name);
        count += 1;
    }
    (a, b)
}

/// The line under them: what Enter does now.
pub const THEN: &[u8] = b"The settings service did not take them. ENTER goes on without them";
