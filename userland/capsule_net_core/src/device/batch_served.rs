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

//! What the answer to net.core's empty OP_RX_BATCH probe says: whether
//! the driver behind a port serves receive batches.

/// The one answer a driver that serves batches gives the probe, which asks
/// for no frames: an empty batch, its count word zero.
const EMPTY_BATCH: [u8; 4] = [0; 4];

/// `answer` is the body of a status-zero reply to the probe, `None` for
/// any other reply or none. Status zero alone is not enough: e1000,
/// e1000e, igc, RTL8139 and RTL8169 number operation 6 for their counters
/// and answer it with status 0, and reading those as batches took in no
/// frame at all.
pub fn served(answer: Option<&[u8]>) -> bool {
    answer == Some(&EMPTY_BATCH[..])
}
