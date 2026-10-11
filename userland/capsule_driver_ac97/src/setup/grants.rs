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

//! What a bring-up holds of the broker once the claim is through: the claim
//! epoch and the two port grants.

use nonos_libc::{mk_device_release, mk_pio_release};

#[derive(Clone, Copy, Debug)]
pub struct Grants {
    pub device_id: u64,
    pub epoch: u64,
    pub nam: u64,
    pub nabm: u64,
}

impl Grants {
    /// Give back both port grants and the claim.
    pub fn release(&self) {
        let _ = mk_pio_release(self.nabm);
        let _ = mk_pio_release(self.nam);
        let _ = mk_device_release(self.device_id);
    }
}
