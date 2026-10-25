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

//! Whether the host carries frames for the drawn station address. Hyper-V
//! with MAC spoofing off drops every frame sent from an address other than
//! the one it assigned, without a word. So before the link is reported up,
//! the driver asks the network for a DHCP offer from the drawn address and
//! watches for an answer to it: a frame sent to that address, or a DHCP
//! reply naming it. None within the bounded wait, and the host-assigned
//! address is used instead, and said so (the lead's ruling on netvsc).
//!
//! The DISCOVER carries only what the egress rules allow a DHCP message
//! (tools/nonos_egress/dhcp_rules.py): option 53 alone, to the broadcast
//! address, sname and file empty. Pure: building it and judging a frame.

use alloc::vec::Vec;

const BROADCAST: [u8; 6] = [0xFF; 6];

/// The internet checksum of an IPv4 header.
fn checksum(h: &[u8]) -> u16 {
    let mut sum: u32 = h.chunks(2).map(|c| u32::from(u16::from_be_bytes([c[0], *c.get(1).unwrap_or(&0)]))).sum();
    while sum >> 16 != 0 {
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    !(sum as u16)
}

/// A DHCPDISCOVER from `mac` with transaction id `xid`, as one Ethernet
/// frame: to the broadcast address, from 0.0.0.0, UDP 68 to 67, the
/// broadcast flag set so a server answers to every address.
pub fn discover(mac: &[u8; 6], xid: u32) -> Vec<u8> {
    let mut bootp = [0u8; 240 + 4];
    (bootp[0], bootp[1], bootp[2]) = (1, 1, 6);
    bootp[4..8].copy_from_slice(&xid.to_be_bytes());
    bootp[10] = 0x80;
    bootp[28..34].copy_from_slice(mac);
    bootp[236..240].copy_from_slice(&[99, 130, 83, 99]);
    bootp[240..244].copy_from_slice(&[53, 1, 1, 255]);
    let udp_len = 8 + bootp.len();
    let ip_len = 20 + udp_len;
    let mut f = Vec::with_capacity(14 + ip_len);
    f.extend_from_slice(&BROADCAST);
    f.extend_from_slice(mac);
    f.extend_from_slice(&[0x08, 0x00]);
    let mut ip = [0u8; 20];
    ip[0] = 0x45;
    ip[2..4].copy_from_slice(&(ip_len as u16).to_be_bytes());
    (ip[8], ip[9]) = (64, 17);
    ip[16..20].copy_from_slice(&[255; 4]);
    let c = checksum(&ip);
    ip[10..12].copy_from_slice(&c.to_be_bytes());
    f.extend_from_slice(&ip);
    f.extend_from_slice(&68u16.to_be_bytes());
    f.extend_from_slice(&67u16.to_be_bytes());
    f.extend_from_slice(&(udp_len as u16).to_be_bytes());
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(&bootp);
    f
}

/// Whether `frame` answers the drawn address `mac`: sent to it, or a DHCP
/// reply (UDP 67 to 68, BOOTREPLY) for transaction `xid` naming it.
pub fn answers(frame: &[u8], mac: &[u8; 6], xid: u32) -> bool {
    if frame.get(..6) == Some(&mac[..]) {
        return true;
    }
    if frame.len() < 14 + 20 || frame[12..14] != [0x08, 0x00] || frame[14 + 9] != 17 {
        return false;
    }
    let ihl = usize::from(frame[14] & 0x0F) * 4;
    let Some(udp) = frame.get(14 + ihl..) else { return false };
    if udp.len() < 8 + 240 || udp[0..4] != [0, 67, 0, 68] {
        return false;
    }
    let bootp = &udp[8..];
    bootp[0] == 2 && bootp[4..8] == xid.to_be_bytes() && bootp[28..34] == mac[..]
}

/// Which station address the driver takes, and why.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    /// The drawn address: the network answered it.
    Drawn,
    /// The host-assigned address: the host refused the drawn one outright.
    Refused,
    /// The host-assigned address: nothing answered the drawn one in the
    /// bounded wait (MAC spoofing off).
    Unanswered,
}

/// The probe's state: how many DISCOVERs were sent and when the last went.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Probe {
    pub sent: u32,
    pub last_ms: u64,
}

/// What the probe does next, at `now_ms`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// Send the next DISCOVER.
    Send,
    /// Keep listening.
    Listen,
    /// The wait is over and nothing answered.
    GiveUp,
}

impl Probe {
    pub const fn new() -> Self {
        Self { sent: 0, last_ms: 0 }
    }

    /// `tries` DISCOVERs `every` ms apart, then one more wait as long.
    pub fn step(&self, now_ms: u64, tries: u32, every: u64) -> Step {
        let due = self.sent == 0 || now_ms.saturating_sub(self.last_ms) >= every;
        match (due, self.sent < tries) {
            (true, true) => Step::Send,
            (true, false) => Step::GiveUp,
            (false, _) => Step::Listen,
        }
    }

    pub fn sent_at(&mut self, now_ms: u64) {
        self.sent += 1;
        self.last_ms = now_ms;
    }
}

impl Default for Probe {
    fn default() -> Self {
        Self::new()
    }
}
