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

//! The source filter across the top of the list.

use super::listing::Source;

/// Which of the three namespaces the list is filtered to, or all of them.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tab {
    All,
    NonOs,
    Linux,
    Community,
}

pub const TABS: [Tab; 4] = [Tab::All, Tab::NonOs, Tab::Linux, Tab::Community];

impl Tab {
    pub fn label(self) -> &'static [u8] {
        match self {
            Tab::All => b"All",
            Tab::NonOs => b"NONOS",
            Tab::Linux => b"Linux",
            Tab::Community => b"Community",
        }
    }

    pub fn accepts(self, source: Source) -> bool {
        match self {
            Tab::All => true,
            Tab::NonOs => source == Source::NonOs,
            Tab::Linux => source == Source::Linux,
            Tab::Community => source == Source::Community,
        }
    }
}
