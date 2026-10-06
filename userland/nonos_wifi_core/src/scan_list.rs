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

//! The networks a scan finds, shared by both drivers (it began in the
//! RTL8821CE driver). The channel hop or the firmware scan feeds it beacons;
//! this is the pure collection: deduplicate by access point, drop hidden
//! networks, cap the list, and encode in the format the settings panel parses:
//! a count, then per network a signal byte (0 to 100), a flags byte, the SSID
//! length and the SSID. The flags say secured (bit 0), and which personal
//! security the RSN element offers: WPA2 (bit 1: PSK or PSK-SHA256) and WPA3
//! (bit 2: SAE), so the panel can show a WPA3 network as such and the client
//! remember it as WPA3.
//!
//! The picture ages: each full channel sweep is counted, a network not heard
//! for `MAX_AGE` sweeps is dropped, and when the list is full a network heard
//! now replaces the one heard longest ago. Without that the first sixteen
//! networks ever heard stayed listed for the whole boot, gone or not, and any
//! seventeenth was never shown. Kept free of hardware so it is checked on the
//! host.

/// The most networks a scan reports.
pub const MAX_RESULTS: usize = 16;
/// Sweeps a network may go unheard before it is dropped.
pub const MAX_AGE: u32 = 3;
const SSID_MAX: usize = 32;
const FLAG_SECURED: u8 = 0x01;
/// The network offers WPA2-Personal.
pub const FLAG_WPA2: u8 = 0x02;
/// The network offers WPA3-Personal (SAE).
pub const FLAG_WPA3: u8 = 0x04;

#[derive(Clone, Copy)]
struct Entry {
    ssid: [u8; SSID_MAX],
    ssid_len: u8,
    bssid: [u8; 6],
    flags: u8,
    signal: u8,
    seen: u32,
}

/// The networks a scan found, deduplicated by access point.
pub struct ScanResults {
    entries: [Entry; MAX_RESULTS],
    count: usize,
    sweep: u32,
}

impl Default for ScanResults {
    fn default() -> Self {
        Self::new()
    }
}

impl ScanResults {
    pub fn new() -> Self {
        let empty = Entry { ssid: [0; SSID_MAX], ssid_len: 0, bssid: [0; 6], flags: 0, signal: 0, seen: 0 };
        Self { entries: [empty; MAX_RESULTS], count: 0, sweep: 0 }
    }

    pub fn count(&self) -> usize {
        self.count
    }

    /// Record one beacon with only the secured bit (the form the earliest
    /// proofs drive).
    pub fn add(&mut self, bssid: [u8; 6], ssid: &[u8], secured: bool) {
        self.add_bss(bssid, ssid, if secured { FLAG_SECURED } else { 0 });
    }

    /// Record one beacon with its security flags and no signal reading.
    pub fn add_bss(&mut self, bssid: [u8; 6], ssid: &[u8], flags: u8) {
        self.heard(bssid, ssid, flags, 0);
    }

    /// Record one beacon with its security flags and signal (0 to 100). An
    /// access point already listed is refreshed (flags and signal updated); a
    /// new one is added, or, with the list full, replaces the one heard
    /// longest ago if that was in an earlier sweep. Hidden networks (an empty
    /// SSID) and overlong SSIDs are not listed.
    pub fn heard(&mut self, bssid: [u8; 6], ssid: &[u8], flags: u8, signal: u8) {
        if ssid.is_empty() || ssid.len() > SSID_MAX {
            return;
        }
        let sweep = self.sweep;
        if let Some(e) = self.entries[..self.count].iter_mut().find(|e| e.bssid == bssid) {
            e.flags = flags;
            e.signal = signal;
            e.seen = sweep;
            return;
        }
        let slot = if self.count < MAX_RESULTS {
            self.count += 1;
            self.count - 1
        } else {
            let oldest = (0..self.count).min_by_key(|&i| self.entries[i].seen).unwrap_or(0);
            if self.entries[oldest].seen >= sweep {
                return;
            }
            oldest
        };
        let mut e = Entry { ssid: [0; SSID_MAX], ssid_len: ssid.len() as u8, bssid, flags, signal, seen: sweep };
        e.ssid[..ssid.len()].copy_from_slice(ssid);
        self.entries[slot] = e;
    }

    /// A channel sweep finished: count it and drop networks not heard for
    /// `MAX_AGE` sweeps.
    pub fn end_sweep(&mut self) {
        self.sweep = self.sweep.wrapping_add(1);
        let sweep = self.sweep;
        let mut i = 0;
        while i < self.count {
            if sweep.wrapping_sub(self.entries[i].seen) > MAX_AGE {
                self.count -= 1;
                self.entries[i] = self.entries[self.count];
            } else {
                i += 1;
            }
        }
    }

    /// Encode for the settings panel: `[count]` then per network `[signal][flags]
    /// [ssid_len][ssid]`. Returns the number of bytes written; a network that
    /// would overflow `out` is dropped rather than truncated.
    pub fn encode(&self, out: &mut [u8]) -> usize {
        if out.is_empty() {
            return 0;
        }
        let mut o = 1;
        let mut written = 0u8;
        for e in &self.entries[..self.count] {
            let n = e.ssid_len as usize;
            if o + 3 + n > out.len() {
                break;
            }
            out[o] = e.signal;
            out[o + 1] = e.flags;
            out[o + 2] = e.ssid_len;
            out[o + 3..o + 3 + n].copy_from_slice(&e.ssid[..n]);
            o += 3 + n;
            written += 1;
        }
        out[0] = written;
        o
    }
}

/// The flags for a beacon: secured when it carries an RSN element or sets the
/// privacy capability, WPA2 and WPA3 from the AKMs its RSN element offers.
pub fn beacon_flags(frame: &[u8], capability: u16) -> u8 {
    use crate::dot11::ies::{beacon_tags, find_element, EID_RSN};
    use crate::rsn::parse_rsne;
    let rsne = beacon_tags(frame).and_then(|t| find_element(t, EID_RSN)).and_then(|e| parse_rsne(&e[2..]));
    let mut flags = if rsne.is_some() || capability & 0x0010 != 0 { FLAG_SECURED } else { 0 };
    if let Some(r) = rsne {
        if r.akm_psk || r.akm_psk_sha256 {
            flags |= FLAG_WPA2;
        }
        if r.akm_sae {
            flags |= FLAG_WPA3;
        }
    }
    flags
}

/// A received signal in dBm as the panel's 0 to 100 scale (-100 dBm or
/// weaker is 0, -50 dBm or stronger is 100); 0 dBm means not measured.
pub fn signal_percent(dbm: i8) -> u8 {
    if dbm >= 0 {
        return 0;
    }
    (2 * (dbm as i16 + 100)).clamp(0, 100) as u8
}
