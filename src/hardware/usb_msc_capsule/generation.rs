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

//! The re-bind generation that ends driver.usb_msc0's state reply
//! (userland/capsule_driver_usb_msc/src/server/handlers/get_state.rs): how
//! many devices the driver served after the first, and media it served
//! afresh. It moves whenever what is served may be another medium, so the
//! block layer can tell and read the store header again before it writes.
//! Pure, so kernel_proofs holds both lengths of the reply.

use super::report_words::REPORT_LEN;

/// The driver's counters, and the search report after them (report.rs).
pub(crate) const COUNTERS_LEN: usize = 48;
const GENERATION_AT: usize = COUNTERS_LEN + REPORT_LEN;

/// The generation in a state reply `body`. A driver that predates it ends
/// its reply at the report, and never served a second device: 0. None for
/// a reply shorter than that.
pub(crate) fn generation(body: &[u8]) -> Option<u32> {
    match body.get(GENERATION_AT..GENERATION_AT + 4) {
        Some(g) => Some(u32::from_le_bytes([g[0], g[1], g[2], g[3]])),
        None if body.len() == GENERATION_AT => Some(0),
        None => None,
    }
}
