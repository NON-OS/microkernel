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

//! What a listing's gate vector actually means for the user.

use crate::store::market::Readiness;

/// The package gate's position in the vector the capsule returns.
const PACKAGE_GATE: usize = 1;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    Ready,
    /// Every gate passes except the one asking for something to fetch.
    /// That is what a capsule already in the image looks like.
    Installed,
    Blocked,
}

impl Verdict {
    pub fn of(r: &Readiness) -> Verdict {
        if r.install_ready {
            return Verdict::Ready;
        }
        let others = r.gates.iter().enumerate().all(|(i, ok)| *ok || i == PACKAGE_GATE);
        match others && !r.gates[PACKAGE_GATE] {
            true => Verdict::Installed,
            false => Verdict::Blocked,
        }
    }

    pub fn label(self) -> &'static [u8] {
        match self {
            Verdict::Ready => b"Install",
            Verdict::Installed => b"Installed",
            Verdict::Blocked => b"Blocked",
        }
    }

    /// The same verdict as something to read rather than a word to decode.
    pub fn sentence(self) -> &'static [u8] {
        match self {
            Verdict::Ready => b"Ready to install on this machine",
            Verdict::Installed => b"Already in this image, nothing to fetch",
            Verdict::Blocked => b"This machine will not run it yet",
        }
    }
}
