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

//! What a sense said of the medium (disk/sense.rs), kept until the scanner
//! asks (scan/ask.rs, scan/check.rs): a unit says it once, under whichever
//! command meets it, and a card swapped in a reader is said so.

use super::types::State;

/// The most a sense said of the medium since the scanner last asked.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Said {
    Nothing,
    /// Any other UNIT ATTENTION but no medium (3Ah): a power-on, a reset, a
    /// mode change. Or a sense that did not read, as descriptor format
    /// does not here, or a command whose phase was lost on the bus, which
    /// may have taken a 28h with it (disk/recover.rs). The medium may be
    /// another: a reader whose transport was reset may say a power-on where
    /// it would have said 28h, as Linux takes any UNIT ATTENTION on a
    /// removable device for a change.
    Attention,
    /// UNIT ATTENTION 28h: the medium may have changed.
    Changed,
}

impl State {
    /// A sense said `said`; the most since the scanner last asked is kept.
    pub fn heard(&mut self, said: Said) {
        self.said = self.said.max(said);
    }

    /// What was said since the scanner last asked.
    pub fn said(&self) -> Said {
        self.said
    }

    /// The same, forgotten once taken: after every command of an asking,
    /// and by every check.
    pub fn take_said(&mut self) -> Said {
        core::mem::replace(&mut self.said, Said::Nothing)
    }
}
