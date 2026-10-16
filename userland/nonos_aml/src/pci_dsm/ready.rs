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

//! How long to leave a function alone once it is back in D0. The
//! specifications' times are the ceilings: 10 ms of recovery from D3hot
//! (PCI Power Management), and 100 ms after a conventional reset, which a
//! function brought up from D3cold has been through (PCI Express Base).
//! Firmware's function 9, for its host bridge's functions, may only
//! shorten them: a value at or past a ceiling, all ones included, or no
//! value, gives the ceiling. Both ceilings are recalled, unverified.

use super::facts::Readiness;
use crate::device_power::DState;

pub const D3HOT_TO_D0_MAX_US: u32 = 10_000;
pub const RESET_MAX_US: u32 = 100_000;

/// The two ways back to D0 that leave a function unready for a while.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wake {
    FromD3Hot,
    FromD3Cold,
}

/// None for a move that needs no wait: not to D0, or from D0, D1 or D2.
pub fn wake_of(from: DState, to: DState) -> Option<Wake> {
    match (from, to) {
        (DState::D3Hot, DState::D0) => Some(Wake::FromD3Hot),
        (DState::D3Cold, DState::D0) => Some(Wake::FromD3Cold),
        _ => None,
    }
}

/// Firmware's duration when it offers one shorter than the ceiling; the
/// ceiling otherwise. Never longer than the ceiling.
pub fn ready_wait_us(r: Option<Readiness>, w: Wake) -> u32 {
    let (ceiling, said) = match w {
        Wake::FromD3Hot => (D3HOT_TO_D0_MAX_US, r.map(|r| r.d3hot_to_d0_us)),
        Wake::FromD3Cold => (RESET_MAX_US, r.map(|r| r.reset_us)),
    };
    said.map_or(ceiling, |s| s.min(ceiling))
}

/// Milliseconds to sleep for `us`, rounded up and one more, so a clock
/// that ticks just after the sleep starts still never cuts it short.
pub fn sleep_ms_for(us: u32) -> u64 {
    u64::from(us).div_ceil(1000) + 1
}
