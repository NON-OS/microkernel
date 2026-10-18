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

use super::active::Active;

/// Longer than any FIFO here (the PL011's 32 bytes, the mini-UART's 8), so
/// a full one is drained in one pass, and finite, so a receiver that keeps
/// reporting data cannot hold the CPU in the handler.
const DRAIN_LIMIT: usize = 64;

/// Drain the receive FIFO and acknowledge the interrupt.
///
/// The bytes go nowhere. Nothing in the kernel consumes serial input on any
/// architecture yet. Draining is not optional even with no consumer: a
/// receive FIFO left full keeps the interrupt asserted and the CPU never
/// gets out of the handler.
pub fn handle_uart_interrupt() {
    let Some(uart) = Active::current() else {
        return;
    };
    for _ in 0..DRAIN_LIMIT {
        if uart.receive().is_none() {
            break;
        }
    }
    uart.acknowledge();
}
