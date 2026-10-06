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
/// Where the controller's DMA comes from. `addr64` is HCCPARAMS1.AC64: a
/// controller without it drives only the low 32 bits of every pointer, so a
/// grant the kernel placed above 4 GiB is given back and refused rather
/// than handed to it truncated.
#[derive(Debug, Clone, Copy)]
pub struct DmaPool {
    pub device_id: u64,
    pub claim_epoch: u64,
    pub addr64: bool,
}
impl DmaPool {
    pub const fn new(device_id: u64, claim_epoch: u64) -> Self {
        Self { device_id, claim_epoch, addr64: true }
    }

    /// The pool for a controller whose HCCPARAMS1.AC64 is `ac64`.
    pub const fn with_ac64(self, ac64: bool) -> Self {
        Self { addr64: ac64, ..self }
    }
}
