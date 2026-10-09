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

//! What a bound USB network device offers the frame service: its station
//! address, its link, and Ethernet frames in and out. Each driver turns its
//! own transfer framing (CDC-ECM, NCM blocks, RNDIS packets, vendor
//! headers) into whole frames behind this.

pub trait Nic {
    fn mac(&self) -> [u8; 6];
    fn link_up(&self) -> bool;
    /// Send one Ethernet frame, 14 to 1514 bytes, no FCS.
    fn send(&mut self, frame: &[u8]) -> Result<(), i32>;
    /// The next frame into `out`, `None` when none has come.
    fn recv(&mut self, out: &mut [u8]) -> Result<Option<usize>, i32>;
    /// Work a driver does on its own clock, such as reading a link it has
    /// no notification for; run after each request is answered, so no
    /// answer waits on it. Most drivers have none.
    fn tick(&mut self) {}
}

/// The bytes of an Ethernet frame the stack hands down or takes up: the
/// header and up to a 1500-byte payload, without the FCS.
pub const ETH_HEADER: usize = 14;
pub const ETH_FRAME_MAX: usize = 1514;
