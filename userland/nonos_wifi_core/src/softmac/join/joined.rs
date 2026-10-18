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

//! How a whole join ends.

use super::report::Report;
use crate::regdom::Band;

pub enum JoinError {
    /// Not heard; how many beacons of other networks were.
    NotFound(u32),
    NoEntropy,
    /// The radio could not take the network's address, or a scan's.
    NoAddress,
}

/// A finished attempt and where the network was heard.
pub struct Joined {
    pub report: Report,
    pub band: Band,
    pub channel: u8,
}
