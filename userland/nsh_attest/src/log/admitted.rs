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

//! The four kinds of admission a PCR 14 event can name.

/// What admitted a capsule, as the event's kind byte says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Admitted {
    /// Kind 1: its STARK proof, under the vendor's root.
    Vendor,
    /// Kind 2: its Merkle path alone, on a development image.
    PathOnly,
    /// Kind 3: its STARK proof, under a root a person enrolled.
    Developer,
    /// Kind 4: this machine's own tag on a local build, with no proof.
    LocalBuild,
}

impl Admitted {
    pub const fn from_kind(kind: u8) -> Option<Self> {
        match kind {
            1 => Some(Self::Vendor),
            2 => Some(Self::PathOnly),
            3 => Some(Self::Developer),
            4 => Some(Self::LocalBuild),
            _ => None,
        }
    }

    pub const fn name(self) -> &'static str {
        match self {
            Self::Vendor => "vendor",
            Self::PathOnly => "path-only",
            Self::Developer => "developer",
            Self::LocalBuild => "local-build",
        }
    }
}
