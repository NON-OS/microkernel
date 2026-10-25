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

use super::super::state;
use super::device::Ns16550;

/// Longer than the 16550's 16-byte FIFO, so a full one drains in one pass,
/// and finite, so a receiver that keeps reporting data cannot hold the hart.
const DRAIN_LIMIT: usize = 64;

/// Drain the receive FIFO. The bytes go nowhere: nothing in the kernel takes
/// serial input on any architecture yet, as on aarch64. Draining is still
/// needed, since a full FIFO keeps the interrupt asserted.
pub fn handle_uart_interrupt() {
    let uart = Ns16550::new(state::uart_base());
    for _ in 0..DRAIN_LIMIT {
        if uart.getc().is_none() {
            break;
        }
    }
}
