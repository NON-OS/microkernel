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

//! Which listings have been offered a proof: each installed Linux listing
//! once per store session, the first time it is selected (ruling 1). After
//! a yes or a no it is not offered again unbidden; `p` offers it again. No
//! store file keeps it: one that outlives the boot is zero-state's question.
//!
//! Pure; market_proofs holds it.

use alloc::vec::Vec;

#[derive(Default)]
pub struct Offered(Vec<Vec<u8>>);

/// A listing a proof can be asked for: a Linux one, installed.
pub fn offerable(id: &[u8], installed: bool) -> bool {
    installed && id.starts_with(b"linux.")
}

impl Offered {
    /// True the first time `id` is asked, and it is then offered: never again.
    pub fn first(&mut self, id: &[u8]) -> bool {
        if self.0.iter().any(|o| o[..] == *id) {
            return false;
        }
        self.0.push(id.to_vec());
        true
    }
}
