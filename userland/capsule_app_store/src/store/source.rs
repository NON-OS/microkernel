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

//! Where a listing came from, read off the namespace its id starts with.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Source {
    NonOs,
    Linux,
    /// A Qwen tier: a model the fetcher downloads, listed in the linux
    /// namespace (`linux.qwen-<tier>`) because the personality installs it,
    /// but no Linux package. Shown as a Linux app, a build whose index held
    /// only the seventeen tiers looked like a Linux tab of seventeen
    /// packages.
    Model,
    Community,
}

impl Source {
    pub fn of(listing_id: &[u8]) -> Source {
        match listing_id {
            id if id.starts_with(b"linux.qwen-") => Source::Model,
            id if id.starts_with(b"linux.") => Source::Linux,
            id if id.starts_with(b"community.") => Source::Community,
            _ => Source::NonOs,
        }
    }

    pub fn label(self) -> &'static [u8] {
        match self {
            Source::NonOs => b"NONOS",
            Source::Linux => b"Linux",
            Source::Model => b"Model",
            Source::Community => b"Community",
        }
    }

    /// The line under a card's name. The store sells apps: where a Linux
    /// package is fetched from is provenance, kept in the signed listing and
    /// checked at install, not what the card is about.
    pub fn origin(self) -> &'static [u8] {
        match self {
            Source::Linux => b"Linux app",
            Source::Model => b"Qwen model",
            _ => self.label(),
        }
    }
}
