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

//! A struct pollfd array answered in place. Pure, so the host proofs hold it.

/// fd, events, revents.
pub const POLLFD_LEN: usize = 8;
const POLLNVAL: u16 = 0x020;
const POLLERR: u16 = 0x008;
const POLLHUP: u16 = 0x010;

/// Each struct pollfd in `raw` answered by `ready`: its revents set, and
/// how many have any. A negative descriptor is an entry switched off: never
/// ready. Hang-up, error and a closed descriptor are reported unasked.
pub fn answer(raw: &mut [u8], ready: impl Fn(u64) -> u16) -> u64 {
    let mut hits = 0;
    for entry in raw.chunks_exact_mut(POLLFD_LEN) {
        let fd = i32::from_le_bytes([entry[0], entry[1], entry[2], entry[3]]);
        let events = u16::from_le_bytes([entry[4], entry[5]]);
        let revents = match u64::try_from(fd) {
            Ok(fd) => ready(fd) & (events | POLLNVAL | POLLHUP | POLLERR),
            Err(_) => 0,
        };
        hits += u64::from(revents != 0);
        entry[6..8].copy_from_slice(&revents.to_le_bytes());
    }
    hits
}
