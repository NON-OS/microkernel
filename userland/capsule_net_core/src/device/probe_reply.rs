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

//! What the reply to an empty OP_RX_BATCH probe has to be.
//!
//! A batch server answers the probe with an empty batch: a count of zero and
//! nothing after it. Status zero alone was taken as yes, and e1000, RTL8139,
//! RTL8169, e1000e and igc answered operation 6 with their 48-byte register
//! snapshot and status zero, so net.core read them as batch servers, could
//! not parse a single batch, and dropped every frame they received. Pure, so
//! the host proofs hold it.

/// Whether a status-zero probe reply says the driver serves batches.
pub fn serves_batches(body: &[u8]) -> bool {
    body == 0u32.to_le_bytes()
}
