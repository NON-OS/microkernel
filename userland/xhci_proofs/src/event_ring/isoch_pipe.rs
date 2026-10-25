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

//! An isoch OUT pipe on the fixture's controller, and a batch sent on it.

use nonos_libc::dma_host;

use super::fixture::{Fixture, SLOT};
use crate::controller::{isoch_endpoint, isoch_out, IsochEndpoint, IsochPipe};
use crate::error::XhciError;
use crate::trb::{read_volatile_at, Trb};

pub const DCI: u8 = 6;
pub const CC_SUCCESS: u8 = 1;
pub const CC_TRANSACTION: u8 = 4;
pub const CC_UNDERRUN: u8 = 14;
pub const CC_MISSED: u8 = 23;

pub fn pipe(fx: &Fixture) -> IsochPipe {
    let (ring, buf) = IsochPipe::memory(&fx.pool).expect("isoch pipe");
    let ep =
        IsochEndpoint { ring_phys: ring.phys(), ..isoch_endpoint(1, false, DCI, 1, (0, 0), 192) };
    IsochPipe::with(ring, buf, ep, false)
}

pub fn trb_at(bus: u64) -> Trb {
    // SAFETY: `bus` names a TRB of a ring the test's pool mapped.
    unsafe { read_volatile_at(dma_host(bus).expect("ring memory")) }
}

pub fn send(fx: &mut Fixture, p: &mut IsochPipe, lens: &[u16], fill: u8) -> Result<u8, XhciError> {
    let data = vec![fill; lens.iter().map(|&l| l as usize).sum()];
    isoch_out(fx.db(), fx.intr(), &mut fx.ring, p, SLOT, lens, &data)
}
