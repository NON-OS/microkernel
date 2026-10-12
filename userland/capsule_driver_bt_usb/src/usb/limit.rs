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

//! How much one look at the event endpoint arms. With B-43 applied,
//! driver.xhci0 reads an interrupt endpoint up to its wMaxPacketSize, bits
//! 10:0, capped at 64 octets, and refuses a longer read with E_INVAL. Less
//! than a whole packet must never be armed: a device that sends more than
//! was armed babbles and halts the endpoint (USB 2.0, 8.4.2).

/// The longest interrupt read driver.xhci0 takes.
pub const INTERRUPT_MAX: usize = 64;

/// The octets to arm on an event endpoint whose wMaxPacketSize is
/// `max_packet`: the whole packet, or `None` for an endpoint whose packet
/// is empty or longer than driver.xhci0 reads, which this driver cannot
/// use. The Core Specification gives the event endpoint 16 octets (Vol 4,
/// Part B, 2.1); some controllers report 64.
pub fn read_limit(max_packet: u16) -> Option<usize> {
    let packet = usize::from(max_packet & 0x07ff);
    (1..=INTERRUPT_MAX).contains(&packet).then_some(packet)
}
