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

/// One SATA port that came up during the walk: its link is up and IDENTIFY
/// answered, so the driver could serve it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Candidate {
    /// Position of the controller among the controllers the walk opened, in
    /// device list order.
    pub controller: usize,
    /// AHCI port number on that controller.
    pub port: u8,
    /// The disk has the package store header magic at the store LBA.
    pub store: bool,
    /// The disk has the disk plan magic at the plan LBA.
    pub plan: bool,
}

/// The candidate the driver serves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Choice {
    /// Index into the candidate list passed to `choose`.
    pub index: usize,
    /// No candidate carries NONOS; this is the first one in walk order.
    pub fallback: bool,
}
