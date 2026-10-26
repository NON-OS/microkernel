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

//! The proof a run made, held in this store's memory for I-B9b (ruling 7):
//! the listing it proves and the container. I-B9b, which sends it, adds the
//! release and listing key it reads. Wiped when dropped, as the wallet wipes its secrets,
//! through zeroize's volatile writes: by a newer proof, or at once when `n`
//! was pressed while it was made.

use alloc::vec::Vec;

use zeroize::Zeroize;

/// The container a run made, for the listing it proves.
pub struct Held {
    pub listing: Vec<u8>,
    pub container: Vec<u8>,
}

impl Drop for Held {
    fn drop(&mut self) {
        self.container.zeroize();
    }
}
