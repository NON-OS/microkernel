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

//! One GET on a stream: the request sent, and the reply read off it within
//! its waits (route_read.rs).

use alloc::vec::Vec;

use super::super::route_read::{read_reply, Reply};
use super::stream::Stream;

/// Says when the reply read so far is whole.
pub(super) type Done = dyn Fn(&[u8]) -> bool;

/// `request` sent on `stream`, and the reply read off it.
pub(super) fn ask(
    s: &mut Stream,
    request: &[u8],
    max: usize,
    done: &Done,
) -> Result<Vec<u8>, &'static str> {
    s.0.write_all(request)?;
    match read_reply(s, max, done) {
        Reply::Got(bytes) => Ok(bytes),
        Reply::Nothing => Err("nothing came back"),
        Reply::TooLarge => Err("the reply is larger than any index or package"),
    }
}
