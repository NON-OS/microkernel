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

//! How long to leave a function alone after it was in a low-power
//! state, reset, or function-level reset. The specifications' times are
//! the ceilings (PCI Power Management, PCI Express Base; all recalled,
//! unverified): none from D1, 200 us from D2, 10 ms from D3hot, and 100 ms
//! after a conventional reset, which a function brought up from D3cold has
//! been through, or after a function-level reset. Firmware's function 9,
//! for its host bridge's functions, may only shorten a wait it has a
//! field for: a value at or past the ceiling, all ones included, or no
//! value gives the ceiling.

use super::facts::Readiness;
use crate::device_power::DState;

pub const D2_TO_D0_MAX_US: u32 = 200;
pub const D3HOT_TO_D0_MAX_US: u32 = 10_000;
pub const RESET_MAX_US: u32 = 100_000;
pub const FLR_MAX_US: u32 = 100_000;

/// Each way back to a function that can be used again.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wake {
    FromD1,
    FromD2,
    FromD3Hot,
    FromD3Cold,
    Reset,
    Flr,
}

/// None for a move that is not back to D0, or is from D0.
pub fn wake_of(from: DState, to: DState) -> Option<Wake> {
    match (from, to) {
        (DState::D1, DState::D0) => Some(Wake::FromD1),
        (DState::D2, DState::D0) => Some(Wake::FromD2),
        (DState::D3Hot, DState::D0) => Some(Wake::FromD3Hot),
        (DState::D3Cold, DState::D0) => Some(Wake::FromD3Cold),
        _ => None,
    }
}

/// The ceiling, and firmware's field for it when function 9 has one.
fn limits(r: Option<Readiness>, w: Wake) -> (u32, Option<u32>) {
    match w {
        Wake::FromD1 => (0, None),
        Wake::FromD2 => (D2_TO_D0_MAX_US, None),
        Wake::FromD3Hot => (D3HOT_TO_D0_MAX_US, r.map(|r| r.d3hot_to_d0_us)),
        Wake::FromD3Cold | Wake::Reset => (RESET_MAX_US, r.map(|r| r.reset_us)),
        Wake::Flr => (FLR_MAX_US, r.map(|r| r.flr_us)),
    }
}

/// Firmware's duration when it offers one shorter than the ceiling; the
/// ceiling otherwise. Never longer than the ceiling.
pub fn ready_wait_us(r: Option<Readiness>, w: Wake) -> u32 {
    let (ceiling, said) = limits(r, w);
    said.map_or(ceiling, |s| s.min(ceiling))
}
