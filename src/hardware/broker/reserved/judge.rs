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

//! Whether a range may be handed to a driver, given the reservations. A
//! device's own BAR that lies wholly inside one reservation is the device
//! firmware placed there, and passes; any other range that shares even one
//! byte with a reservation of its space is refused. Ranges that only abut
//! share nothing.

use super::wire::Reservation;

fn shares(r: &Reservation, space: u8, base: u64, end: u64) -> bool {
    r.space == space && base < r.end() && r.base < end
}

fn holds(r: &Reservation, space: u8, base: u64, end: u64) -> bool {
    r.space == space && r.base <= base && end <= r.end()
}

/// The reservation a request runs into, if any. A request that wraps is
/// taken to reach the top of the space.
pub fn conflict(
    reserved: &[Reservation],
    space: u8,
    base: u64,
    len: u64,
    own_bar: bool,
) -> Option<Reservation> {
    let end = base.saturating_add(len);
    if len == 0 || (own_bar && reserved.iter().any(|r| holds(r, space, base, end))) {
        return None;
    }
    reserved.iter().find(|r| shares(r, space, base, end)).copied()
}

/// Whether reservations of one space cover every byte of the range, joined
/// end to end where they abut or overlap.
pub fn covered(reserved: &[Reservation], space: u8, base: u64, len: u64) -> bool {
    let end = base.saturating_add(len);
    let mut at = base;
    // each pass moves past one reservation, or stops
    for _ in 0..=reserved.len() {
        if at >= end {
            return true;
        }
        let next = reserved.iter().filter(|r| r.space == space && r.base <= at && at < r.end());
        match next.map(|r| r.end()).max() {
            Some(e) => at = e,
            None => return false,
        }
    }
    at >= end
}
