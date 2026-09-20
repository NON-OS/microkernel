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

//! One row of the catalogue.

use alloc::vec::Vec;

/// Where a listing came from, read off the namespace its id starts with.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Source {
    NonOs,
    Linux,
    Community,
}

impl Source {
    pub fn of(listing_id: &[u8]) -> Source {
        match listing_id {
            id if id.starts_with(b"linux.") => Source::Linux,
            id if id.starts_with(b"community.") => Source::Community,
            _ => Source::NonOs,
        }
    }

    pub fn label(self) -> &'static [u8] {
        match self {
            Source::NonOs => b"NONOS",
            Source::Linux => b"Linux",
            Source::Community => b"Community",
        }
    }
}

pub struct Listing {
    pub id: Vec<u8>,
    pub measurement: [u8; 32],
    pub name: Vec<u8>,
    /// The market capsule's verdict across every install gate, taken as given.
    pub ready: bool,
    pub source: Source,
}

impl Listing {
    pub fn new(id: Vec<u8>, measurement: [u8; 32], name: Vec<u8>, ready: bool) -> Listing {
        let source = Source::of(&id);
        Listing { id, measurement, name, ready, source }
    }
}
