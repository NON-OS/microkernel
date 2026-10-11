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

//! Ethernet sizing, the envelope every NIC capsule answers in. The MAC
//! strips the FCS from what it hands up only in the length the receive
//! descriptor reports, so tg3_rx takes ETH_FCS_LEN off that length; the
//! bound here applies after that.

const ETH_HEADER_LEN: usize = 14;
const MTU: usize = 1500;

pub const MAC_LEN: usize = 6;
/// A bare header is the shortest frame taken; the MAC pads a short frame
/// on the wire.
pub const MIN_ETHERNET_FRAME: usize = ETH_HEADER_LEN;
pub const MAX_ETHERNET_FRAME: usize = MTU + ETH_HEADER_LEN;
/// ETH_FCS_LEN, taken off the length a receive descriptor reports.
pub const FCS_LEN: usize = 4;
