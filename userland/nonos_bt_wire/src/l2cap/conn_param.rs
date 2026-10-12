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

//! Connection Parameter Update Request and Response (Vol 3, Part A, 4.20
//! and 4.21): a peripheral asks the central for new connection timing,
//! which the central checks against the ranges of Vol 6, Part B, 4.5.1
//! and 4.5.2 before it agrees.

use crate::cursor::Reader;
use crate::error::WireError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConnParams {
    /// In 1.25 ms units.
    pub interval_min: u16,
    pub interval_max: u16,
    /// Connection events the peripheral may skip.
    pub latency: u16,
    /// Supervision timeout, in 10 ms units.
    pub timeout: u16,
}

pub const UPDATE_ACCEPTED: u16 = 0x0000;
pub const UPDATE_REJECTED: u16 = 0x0001;

impl ConnParams {
    pub fn parse(data: &[u8]) -> Result<Self, WireError> {
        let mut r = Reader::new(data);
        let p = Self {
            interval_min: r.u16()?,
            interval_max: r.u16()?,
            latency: r.u16()?,
            timeout: r.u16()?,
        };
        r.finish()?;
        Ok(p)
    }

    /// Whether the central may accept these: intervals 7.5 ms to 4 s with
    /// min no more than max, latency under 500, timeout 100 ms to 32 s and
    /// longer than two of the longest gaps between heard events,
    /// (1 + latency) * interval_max * 2.
    pub fn acceptable(&self) -> bool {
        let interval = 0x0006..=0x0c80;
        let within = interval.contains(&self.interval_min)
            && interval.contains(&self.interval_max)
            && self.interval_min <= self.interval_max
            && self.latency <= 0x01f3
            && (0x000a..=0x0c80).contains(&self.timeout);
        // Both sides in 2.5 ms units: timeout * 4 > (1 + latency) * max.
        let gaps = (1 + u32::from(self.latency)) * u32::from(self.interval_max);
        within && 4 * u32::from(self.timeout) > gaps
    }
}
