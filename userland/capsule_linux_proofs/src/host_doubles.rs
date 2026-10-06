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

//! Host doubles for the two modules `root.rs` and `resolve.rs` reach that make
//! syscalls: the family's private prefixes (a random id from the kernel) and
//! the clamp log (a console write). The doubles keep the signatures and model
//! install mode, where every path maps to the shared tree, which is what the
//! key and resolve proofs are about. The clamp double counts, so a proof can
//! say a clamp was reported and not only that the path came out clamped.

pub mod private {
    pub fn shared_writes_allowed() -> bool {
        true
    }
    pub fn is_private(_visible: &[u8]) -> bool {
        false
    }
    pub fn root() -> Vec<u8> {
        Vec::new()
    }
}

pub mod clamp {
    use core::sync::atomic::{AtomicU32, Ordering};

    pub static NOTED: AtomicU32 = AtomicU32::new(0);

    pub fn note(_path: &[u8]) {
        NOTED.fetch_add(1, Ordering::Relaxed);
    }
}
