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

//! Every length and noise into every decoder: refused, never a panic.

use crate::body::{caps::Caps, cursor::Cursor, flush::Flush, modes::Modes};
use crate::body::{state::State, surface::Surface};
use crate::header::Header;
use crate::reply::Reply;
use crate::request::Request;

fn all(b: &[u8]) {
    let _ = (Caps::decode(b), Cursor::decode(b), Flush::decode(b), Modes::decode(b));
    let _ = (State::decode(b), Surface::decode(b), Header::decode(b), Request::decode(b));
    if let Ok(h) = Header::decode(b) {
        let _ = Reply::decode(b, &h);
    }
}

#[test]
fn every_length_of_noise() {
    let mut x: u32 = 0x2545_F491;
    let mut buf = [0u8; 300];
    for round in 0..20_000 {
        for v in buf.iter_mut() {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            *v = x as u8;
        }
        if round % 2 == 0 {
            // A valid magic and version, so the length checks are reached.
            buf[..6].copy_from_slice(&[0x50, 0x47, 0x56, 0x4E, 1, 0]);
            buf[17..20].fill(0);
        }
        all(&buf[..round % 300]);
    }
}
