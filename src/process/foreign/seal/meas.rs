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

//! M, the chain of measurements a supervisor had the kernel verify.

const MEAS_DOMAIN: &[u8] = b"NONOS-FAMILY-MEAS-v1";

/// M = BLAKE3(domain | M | measurement), from M0 = zero.
pub(crate) fn meas_step(m: &[u8; 32], measurement: &[u8; 32]) -> [u8; 32] {
    let mut h = blake3::Hasher::new();
    h.update(MEAS_DOMAIN);
    h.update(m);
    h.update(measurement);
    *h.finalize().as_bytes()
}
