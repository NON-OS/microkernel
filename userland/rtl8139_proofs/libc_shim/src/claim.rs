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

//! The claim and the port grant the setup sequence asks the broker for, as
//! it answers them on a machine with the card: the claim at epoch 1, and
//! the port grant naming the part `attach` put behind the port calls.

use crate::grants::PioGrantOut;

/// The grant id the port calls answer under; the part ignores it.
const PIO_GRANT: u64 = 3;

pub fn mk_device_claim(_device_id: u64) -> i64 {
    1
}

pub fn mk_pio_grant(
    _device: u64,
    _epoch: u64,
    _bar: u8,
    _flags: u32,
    out: &mut PioGrantOut,
) -> i64 {
    *out = PioGrantOut { port_base: 0xC000, port_count: 0x100, _pad: 0, grant_id: PIO_GRANT };
    0
}
