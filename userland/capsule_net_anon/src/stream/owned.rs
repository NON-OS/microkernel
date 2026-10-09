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

//! Which caller a stream answers to.
//!
//! Stream ids are sequential, so an id names a stream but proves nothing about
//! who may use it. Every API request carries the sender's pid, and a stream
//! answers only the pid that opened it. Streams the SOCKS front opens belong to
//! `FRONT`, which no API sender can be, since the serve loop drops sender 0;
//! the front keeps its own per-caller map.

extern crate alloc;

use alloc::vec::Vec;

use super::table::Stream;

/// The owner of a stream the SOCKS front opened.
pub const FRONT: u32 = 0;

/// The owner of a stream an onion lookup opens to an HSDir. No pid is this
/// value, so no caller can reach it, and it is never reaped.
pub const ONION: u32 = u32::MAX;

/// The index of stream `id` if `owner` opened it.
pub fn owned(streams: &[Stream], owner: u32, id: u16) -> Option<usize> {
    streams.iter().position(|s| s.id == id && s.owner == owner)
}

/// Whether `owner` may open one more stream in a table of at most `max`. No
/// one caller holds more than half, so none can take every stream from the
/// rest: one that did would refuse Anyone to every other program.
pub fn may_open(streams: &[Stream], owner: u32, max: usize) -> bool {
    streams.len() < max && streams.iter().filter(|s| s.owner == owner).count() < max / 2
}

/// The ids of the streams whose owner `alive` says has ended. Only the owner
/// can close a stream, so a caller that ended without closing left each one
/// holding a place for good, ended by the far end or not. The SOCKS front's
/// streams are its own to end.
pub fn orphaned(streams: &[Stream], alive: impl Fn(u32) -> bool) -> Vec<u16> {
    streams
        .iter()
        .filter(|s| s.owner != FRONT && s.owner != ONION && !alive(s.owner))
        .map(|s| s.id)
        .collect()
}

/// True when circuit `circuit` carries a stream someone other than `owner`
/// opened, so tearing it down would end another caller's stream.
pub fn carries_others(streams: &[Stream], owner: u32, circuit: u32) -> bool {
    streams.iter().any(|s| s.circuit == circuit && s.owner != owner)
}
