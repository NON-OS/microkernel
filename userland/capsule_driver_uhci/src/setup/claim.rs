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

//! The claim on the host, given back when it is dropped: a failed bring-up
//! attempt and the broker's revocation of every grant both follow from it.

use nonos_libc::{mk_device_claim, mk_device_release};

pub struct Claim {
    pub device_id: u64,
    pub epoch: u64,
}

impl Claim {
    pub fn take(device_id: u64) -> Result<Self, &'static str> {
        let epoch = mk_device_claim(device_id);
        if epoch <= 0 {
            return Err("uhci: the broker refused the claim");
        }
        Ok(Self { device_id, epoch: epoch as u64 })
    }
}

impl Drop for Claim {
    fn drop(&mut self) {
        let _ = mk_device_release(self.device_id);
    }
}
