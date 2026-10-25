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

//! The modelled EC's lies, for the F-103 storms.

use crate::ec_model::EcModel;
use std::collections::VecDeque;

/// How the modelled EC lies: bytes left in its output buffer before any
/// command, an output buffer that never empties, events that never end;
/// and the clock the storm bound reads.
#[derive(Default)]
pub struct Tricks {
    pub stale: VecDeque<u8>,
    pub obf_stuck: bool,
    pub endless: bool,
    pub now: u64,
}

impl EcModel {
    pub(crate) fn next_event(&mut self) -> u8 {
        if self.tricks.endless {
            return 0x42;
        }
        self.events.pop_front().unwrap_or(0)
    }
}
