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

//! Locality 0: ask for it, and give it back.

use super::bus::{wait_bits, FifoBus};
use super::fail::FifoFail;
use super::regs::{
    ACCESS_ACTIVE_LOCALITY, ACCESS_REQUEST_USE, ACCESS_VALID, TIMEOUT_A_MS, TPM_ACCESS,
};

const GRANTED: u8 = ACCESS_VALID | ACCESS_ACTIVE_LOCALITY;

/// Every command runs inside a granted locality. The grant is read only
/// together with tpmRegValidSts, because without it the other bits of the
/// access register are not defined.
pub(crate) fn request<B: FifoBus>(bus: &mut B) -> Result<(), FifoFail> {
    if bus.read8(TPM_ACCESS) & GRANTED == GRANTED {
        return Ok(());
    }
    bus.write8(TPM_ACCESS, ACCESS_REQUEST_USE);
    match wait_bits(bus, TPM_ACCESS, GRANTED, GRANTED, TIMEOUT_A_MS) {
        Some(_) => Ok(()),
        None => Err(FifoFail::LocalityNotGranted),
    }
}

/// Writing activeLocality gives the locality up. Done on every path, since
/// a locality held after a failure would lock out every later command.
pub(crate) fn relinquish<B: FifoBus>(bus: &mut B) {
    bus.write8(TPM_ACCESS, ACCESS_ACTIVE_LOCALITY);
}
