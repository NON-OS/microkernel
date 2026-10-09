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

//! What discovery hands bring-up: one controller found, and the ordered
//! list of the ones worth trying.

/// Controllers one instance considers. Laptops carry one or two.
pub const MAX_CONTROLLERS: usize = 4;

#[derive(Debug, Clone, Copy)]
pub struct Found {
    pub device_id: u64,
    pub bar_size: u64,
    pub vendor: u16,
    pub device: u16,
    /// An Intel Optane memory cache module (`rank::is_cache_module`).
    pub cache: bool,
}

/// The NVMe controllers worth trying, in the order to try them.
#[derive(Debug, Clone, Copy)]
pub struct Candidates {
    pub(super) list: [Found; MAX_CONTROLLERS],
    pub(super) count: usize,
}

impl Candidates {
    pub fn iter(&self) -> impl Iterator<Item = &Found> {
        self.list[..self.count].iter()
    }
}
