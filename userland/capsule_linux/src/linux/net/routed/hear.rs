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

//! What a guest writes to the SOCKS endpoint before its stream is open:
//! its request (socks/talk.rs), taken only as far as it reaches.

use super::entry::Phase;
use super::socks::Heard;
use super::table;
use crate::linux::abi::errno::{self, EPIPE};
use crate::linux::guest::Guest;

/// The guest's request, or the front of it: how many bytes were taken.
pub fn hear(guest: &Guest, id: u32, h: u32, bytes: &[u8]) -> u64 {
    let heard = table::with(h, |e| match &mut e.phase {
        Phase::Asking(talk) => Some(talk.hear(bytes, &mut e.out)),
        _ => None,
    });
    let Some(Some((took, heard))) = heard else {
        return errno::fail(EPIPE);
    };
    match heard {
        Heard::More => {}
        Heard::Closed => {
            table::with(h, |e| e.phase = Phase::Closed);
        }
        Heard::Connect(ver, c) => super::ask::start(guest, id, h, ver, c),
    }
    errno::ok(took as u64)
}
