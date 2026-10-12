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
//! The alternate setting a stream runs on: the smallest whose endpoint
//! carries, each service interval, at least the payload the device asked
//! for in its probe (dwMaxPayloadTransferSize), so no bandwidth is held
//! that the stream does not use.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Alt {
    pub setting: u8,
    pub endpoint: u8,
    pub max_packet: u16,
    /// Bytes one service interval carries.
    pub per_interval: u32,
    /// The endpoint's bInterval.
    pub interval: u8,
}

pub fn pick_alt(alts: &[Alt], payload: u32) -> Option<Alt> {
    let fits = alts.iter().filter(|a| a.per_interval >= payload);
    fits.min_by_key(|a| (a.per_interval, a.setting)).copied()
}
