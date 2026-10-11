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

//! What a soft-MAC chip driver gives the shared station: a radio that tunes,
//! sends and receives whole 802.11 frames, and an environment with a clock
//! and a source of randomness. Everything above the frames (scan, join,
//! keys, the data path, the control and link protocols) is the core's, so a
//! chip driver is its registers and rings and nothing else.

use crate::regdom::Band;

/// One received frame: its length in the caller's buffer (FCS removed), the
/// signal it arrived at, and whether the chip decrypted it (CCMP header and
/// MIC left in place, as `LinkStation::receive` expects).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RxInfo {
    pub len: usize,
    pub rssi_dbm: i8,
    pub decrypted: bool,
}

pub trait Radio {
    /// The station address the radio transmits from.
    fn mac(&self) -> [u8; 6];
    /// Which bands the radio has: 2.4 GHz, 5 GHz.
    fn bands(&self) -> (bool, bool);
    /// Retune to a 20 MHz channel; false if the chip refused it.
    fn tune(&mut self, band: Band, channel: u8) -> bool;
    /// Queue one complete MPDU without FCS; false if it could not be queued.
    fn send(&mut self, mpdu: &[u8]) -> bool;
    /// The next received frame, copied into `out`, or `None` for now.
    fn recv(&mut self, out: &mut [u8]) -> Option<RxInfo>;
    /// Point the receive filter at one BSS, or open it to every network.
    fn set_bss(&mut self, bssid: Option<[u8; 6]>);
    /// Periodic chip work (calibration, temperature compensation), called
    /// from every serve pass with the uptime.
    fn service(&mut self, _now_ms: u64) {}
    /// Chip-specific words the status op carries for a person diagnosing a
    /// bring-up on a machine with no serial console.
    fn diag(&self) -> [u32; 5] {
        [0; 5]
    }
}

pub trait Env {
    /// Milliseconds of uptime; a broken clock should read as far in the future.
    fn now_ms(&self) -> u64;
    /// Fill `out` with fresh randomness; false if none was available.
    fn random(&mut self, out: &mut [u8]) -> bool;
}
