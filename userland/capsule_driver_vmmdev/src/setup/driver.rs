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

//! The live state the poll loop holds.

use crate::regs::Memory;
use crate::req::{HostVersion, Page};

pub struct Driver {
    pub memory: Memory,
    pub page: Page,
    pub host: HostVersion,
    /// The last position posted, so a repeat is not posted again.
    pub last: Option<(i32, i32)>,
}
