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

//! Ethernet sizing. The capsule never inspects frame contents but enforces
//! the upper bound at the IPC boundary so a caller cannot drive the TX DMA
//! buffer past its grant. RCTL.SECRC strips the FCS in hardware, as
//! igc_setup_rctl does, so a received frame is at most `MAX_ETHERNET_FRAME`
//! bytes. The bounds match `capsule_driver_e1000` so the stack sees one
//! envelope across every NIC backend.

const ETH_HEADER_LEN: usize = 14;
const MTU: usize = 1500;

pub const MAC_LEN: usize = 6;
/// A bare header is the shortest frame taken. TCTL.PSP has the part pad
/// anything under 60 bytes, and an ARP (42) or a bare TCP ACK (54) is
/// shorter than that.
pub const MIN_ETHERNET_FRAME: usize = ETH_HEADER_LEN;
pub const MAX_ETHERNET_FRAME: usize = MTU + ETH_HEADER_LEN;
