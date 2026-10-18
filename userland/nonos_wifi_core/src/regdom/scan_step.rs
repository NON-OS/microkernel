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

//! One step of a sweep: a channel, and whether the station may probe there.

use super::band::Band;

/// Long enough on an active channel for probe responses to come back.
pub const ACTIVE_DWELL_MS: u32 = 40;
/// Longer than one 102.4 ms beacon interval, so a passive dwell hears every AP.
pub const PASSIVE_DWELL_MS: u32 = 120;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ScanMode {
    /// Probe requests may be sent.
    Active,
    /// Listen only.
    Passive,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ScanStep {
    pub band: Band,
    pub channel: u8,
    pub mhz: u32,
    pub mode: ScanMode,
    pub dwell_ms: u32,
}
