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

//! What the parse found: the first usable region of each structure.

use super::msix::MsixLayout;
use super::region::Region;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ModernCaps {
    pub common: Option<Region>,
    pub notify: Option<Region>,
    /// Bytes between consecutive queue notify addresses, from the same
    /// capability as `notify`. Zero means every queue shares one address.
    pub notify_multiplier: u32,
    pub isr: Option<Region>,
    pub device: Option<Region>,
    /// The function's MSI-X table and PBA, when it has an MSI-X capability.
    pub msix: Option<MsixLayout>,
}
