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
    /// The Qwen tiers, apart from the Linux packages they share a namespace
    /// with (`Source::Model`).
    Models,
    Community,
}

pub const TABS: [Tab; 5] = [Tab::All, Tab::NonOs, Tab::Linux, Tab::Models, Tab::Community];

impl Tab {
    pub fn label(self) -> &'static [u8] {
        match self {
            Tab::All => b"All",
            Tab::NonOs => b"NONOS",
            Tab::Linux => b"Linux",
            Tab::Models => b"Models",
            Tab::Community => b"Community",
        }
    }

    /// Why this tab shows nothing of a catalogue that is not empty, one line
    /// per `\n`. The signed baseline lists only what can be installed: the
    /// NONOS apps ship in the image (tools/nonos-market-index, --no-nonos),
    /// a build may list no Linux packages at all, and no community
    /// submission is in it yet. Said, so an empty tab does not read as a
    /// catalogue that failed to load.
    pub fn empty(self) -> &'static [u8] {
        match self {
            Tab::NonOs => {
                b"this catalogue lists no NONOS apps:\nthey ship in this image, nothing to install"
            }
            Tab::Linux => b"this build lists no Linux packages:\nQwen models are under Models",
            Tab::Models => b"this catalogue lists no Qwen models",
            Tab::Community => b"this catalogue lists no community apps yet",
            Tab::All => b"nothing under this tab",
        }
    }

    pub fn accepts(self, source: Source) -> bool {
        match self {
            Tab::All => true,
            Tab::NonOs => source == Source::NonOs,
            Tab::Linux => source == Source::Linux,
            Tab::Models => source == Source::Model,
            Tab::Community => source == Source::Community,
        }
    }
}
