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

//! A card that is in, waiting for its link: the timing of an arrival.

use super::slot::{Act, Phase, Read};
use super::times::{LINK_UP_MS, READY_MS};

/// A card that is in: probed once it has been up `READY_MS`; let go, and
/// its power cut, if its link is not up within `LINK_UP_MS`.
pub(super) fn timed(r: Read, since: u64, steady: Option<u64>, now: u64) -> (Phase, Act) {
    let steady = if r.here() { steady.or(Some(now)) } else { None };
    match steady {
        Some(s) if now.saturating_sub(s) >= READY_MS => (Phase::Occupied, Act::Probe),
        Some(_) => (Phase::Coming { since, steady }, Act::Nothing),
        None if !r.card() => (Phase::Empty, Act::Nothing),
        None if now.saturating_sub(since) >= LINK_UP_MS => (Phase::Off, Act::PowerOff),
        None => (Phase::Coming { since, steady: None }, Act::Nothing),
    }
}
