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

//! What the walk over every controller and port leaves behind.

use alloc::vec::Vec;

use super::open::Opened;
use crate::choose::Candidate;
use crate::engine::Port;
use crate::error::AhciError;

/// A port that came up, kept running until the choice is made.
pub(super) struct Probed {
    pub port: Port,
    pub cand: Candidate,
}

/// Everything the walk opened and brought up.
pub(super) struct Walk {
    pub opened: Vec<Opened>,
    pub probed: Vec<Probed>,
    /// The first controller that could not be opened, if any.
    pub first_error: Option<AhciError>,
    /// The first port that held something but did not come up as a disk the
    /// driver serves. An empty port is no error.
    pub first_port_error: Option<AhciError>,
}
