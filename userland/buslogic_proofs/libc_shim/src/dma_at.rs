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

//! The engine's view of the grants: a bus address to the host memory behind
//! it.

use crate::dma::MAPPED;

/// The host pointer behind bus address `dev`, as the engine reaches it.
pub fn dma_at(dev: u64) -> *mut u8 {
    MAPPED.with(|m| {
        let m = m.borrow();
        let &(base, va, len) = m
            .iter()
            .find(|(b, _, l)| dev >= *b && dev < b + l)
            .expect("bus address outside every grant");
        assert!(dev - base < len);
        (va + (dev - base)) as *mut u8
    })
}
