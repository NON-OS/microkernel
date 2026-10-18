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

//! What a codec plays through: the outputs this driver drives, each with
//! its route to a DAC, as `assign` chose them, or why there are none.

pub use super::assign::plan;
use super::path::Path;
use super::pincfg::OutKind;

pub const MAX_OUTPUTS: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Output {
    pub kind: OutKind,
    pub path: Path,
    /// The jack reports whether something is plugged in.
    pub senses: bool,
}

impl Output {
    pub fn pin(&self) -> u8 {
        self.path.nodes[0]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Plan {
    pub outputs: [Option<Output>; MAX_OUTPUTS],
}

impl Plan {
    pub fn iter(&self) -> impl Iterator<Item = &Output> {
        self.outputs.iter().flatten()
    }

    pub fn has(&self, kind: OutKind) -> bool {
        self.iter().any(|o| o.kind == kind)
    }

    /// The headphone jack that decides whether the speakers play.
    pub fn sensing_headphone(&self) -> Option<u8> {
        self.iter().find(|o| o.kind == OutKind::Headphone && o.senses).map(|o| o.pin())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Class {
    /// At least one analog output with a path to a DAC.
    Analog(Plan),
    /// Output pins, all of them HDMI or DisplayPort.
    DigitalOnly,
    /// Nothing this driver can play through.
    NoOutput,
    /// No output routed, and the search ran out its budget on a pin: a
    /// graph no real codec reports, given up on rather than searched on.
    OverBudget,
}
