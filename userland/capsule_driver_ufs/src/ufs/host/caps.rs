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

//! What the host says it can do, read once it is enabled.

use crate::ufs::env::Mmio;
use crate::ufs::facts::hci::{CAP, CAP_64AS, VER, VER_1X};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Caps {
    /// DMA addresses above 4 GiB reach the host.
    pub a64: bool,
    /// UFSHCI 1.0 or 1.1, which takes the older command types in a UTRD.
    pub v1x: bool,
}

pub fn read<M: Mmio>(m: &M) -> Caps {
    Caps { a64: m.r32(CAP) & CAP_64AS != 0, v1x: m.r32(VER) & VER_1X != 0 }
}
