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

//! What lies past the host's switch port: the switch's one rule that
//! matters here, and a DHCP server. Hyper-V's switch with MAC spoofing off
//! drops every frame whose source is not the address it assigned the port,
//! and delivers to the port only frames for that address, broadcast or
//! multicast; with spoofing on it carries the guest's own address too
//! (Microsoft's documentation of the MacAddressSpoofing port setting).
//! The DHCP server offers an address to every DISCOVER that reaches it, to
//! the broadcast address when the client set the broadcast flag (RFC 2131
//! 4.1), else to the client's hardware address.

pub struct Wire {
    pub spoofing: bool,
    pub dhcp: bool,
    /// Every frame that left the port, and every DISCOVER answered.
    pub out: Vec<Vec<u8>>,
    pub offers: u32,
}

impl Wire {
    pub fn new() -> Self {
        Self { spoofing: false, dhcp: true, out: Vec::new(), offers: 0 }
    }

    /// Whether the switch carries a frame from the port, `assigned` being
    /// the port's address and `taken` the guest's own the host accepted.
    pub fn carries_out(&self, frame: &[u8], assigned: [u8; 6], taken: Option<[u8; 6]>) -> bool {
        let src = &frame[6..12];
        src == assigned || (self.spoofing && taken.is_some_and(|t| t == src))
    }

    /// Whether the switch delivers `frame` to the port.
    pub fn carries_in(&self, frame: &[u8], assigned: [u8; 6], taken: Option<[u8; 6]>) -> bool {
        let dst = &frame[0..6];
        dst[0] & 1 == 1 || dst == assigned || (self.spoofing && taken.is_some_and(|t| t == dst))
    }

    /// The frames a frame that left the port brings back.
    pub fn answer(&mut self, frame: &[u8]) -> Vec<Vec<u8>> {
        self.out.push(frame.to_vec());
        if !self.dhcp || frame.len() < 14 + 20 + 8 + 240 || frame[12..14] != [8, 0] || frame[23] != 17 {
            return Vec::new();
        }
        let udp = &frame[34..];
        let bootp = &udp[8..];
        if udp[0..4] != [0, 68, 0, 67] || bootp[0] != 1 || bootp[240..243] != [53, 1, 1] {
            return Vec::new();
        }
        self.offers += 1;
        let chaddr: [u8; 6] = bootp[28..34].try_into().unwrap();
        let broadcast = bootp[10] & 0x80 != 0;
        let mut offer = Vec::new();
        offer.extend_from_slice(if broadcast { &[0xFF; 6] } else { &chaddr });
        offer.extend_from_slice(&[0x02, 0, 0, 0, 0, 0x01]);
        offer.extend_from_slice(&[8, 0]);
        let mut ip = [0u8; 20];
        (ip[0], ip[8], ip[9]) = (0x45, 64, 17);
        ip[2..4].copy_from_slice(&((20 + 8 + 244) as u16).to_be_bytes());
        ip[12..16].copy_from_slice(&[10, 0, 0, 1]);
        ip[16..20].copy_from_slice(&[255; 4]);
        offer.extend_from_slice(&ip);
        offer.extend_from_slice(&[0, 67, 0, 68]);
        offer.extend_from_slice(&((8 + 244) as u16).to_be_bytes());
        offer.extend_from_slice(&[0, 0]);
        let mut reply = [0u8; 244];
        (reply[0], reply[1], reply[2]) = (2, 1, 6);
        reply[4..8].copy_from_slice(&bootp[4..8]);
        reply[10] = bootp[10];
        reply[16..20].copy_from_slice(&[10, 0, 0, 50]);
        reply[28..34].copy_from_slice(&chaddr);
        reply[236..240].copy_from_slice(&[99, 130, 83, 99]);
        reply[240..244].copy_from_slice(&[53, 1, 2, 255]);
        offer.extend_from_slice(&reply);
        vec![offer]
    }
}

impl Default for Wire {
    fn default() -> Self {
        Self::new()
    }
}
