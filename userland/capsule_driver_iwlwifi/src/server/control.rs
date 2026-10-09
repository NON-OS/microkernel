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

//! The Wi-Fi control family Settings, first-boot setup and net_core speak to
//! whichever Wi-Fi driver is running (`nonos_wifi_client`): tag 0x57494649,
//! `[magic u32][op u16][request id u32]`.
//!
//! On an SO platform (AX211, or AX201 on that platform) the firmware is
//! brought up and runs passive scans in the background; the status op then
//! reports Ready and the scan op answers at once from what was heard, in the
//! layout the RTL8821CE driver uses: three counters (sweeps completed, frames
//! received, beacons parsed) then the network list. When the radio can join
//! (it is up, a station address was drawn, and the firmware speaks every
//! join command's layout) connect, disconnect and link go to the radio
//! (`route`, `join_wire`); otherwise they are refused here with -38 rather
//! than left to time out. When the radio is not up the status op reports how
//! far bring-up got, and every other op is refused with -38.
//!
//! The same inbox also takes net_core's link protocol (tag "NNET",
//! `nonos_wifi_core::netif`), the one it speaks to the RTL8821CE.
//!
//! The status reply is the stage byte (all the client reads), then the step
//! bring-up stopped at, its detail word, CSR_HW_REV and CSR_HW_RF_ID, and the
//! scan counters (sweeps completed, sweeps stalled, frames, beacons), all
//! little-endian.

use nonos_wifi_core::scan_list::{ScanResults, MAX_RESULTS};

/// The control family tag, distinct from this driver's own protocol tag.
const WIFI_MAGIC: u32 = 0x5749_4649;
const WIFI_HDR: usize = 10;
/// net_core's link protocol tag (`netif::wire::MAGIC_NNET`).
const NNET_MAGIC: u32 = 0x4E4E_4554;
pub const OP_CONNECT: u16 = 1;
pub const OP_DISCONNECT: u16 = 2;
pub const OP_LINK: u16 = 5;
const OP_SCAN: u16 = 3;
const OP_STATUS: u16 = 4;
/// ENOSYS: the op exists in the family and this driver does not run it.
const E_NO_OP: i32 = -38;
const SCAN_STATS: usize = 12;
const STATUS_LEN: usize = 1 + 1 + 4 * 7;
const SSID_MAX: usize = 32;

/// The longest reply: a scan with every network at the longest SSID.
pub const REPLY_MAX: usize = WIFI_HDR + SCAN_STATS + 1 + MAX_RESULTS * (3 + SSID_MAX);

/// What the radio has heard, while it is up.
pub struct Heard<'a> {
    pub list: &'a ScanResults,
    pub sweeps: u32,
    pub stalls: u32,
    pub frames: u32,
    pub beacons: u32,
}

/// What a control request is answered from.
pub struct View<'a> {
    pub stage: u8,
    pub step: u8,
    pub detail: u32,
    pub hw_rev: u32,
    pub rf_id: u32,
    /// `Some` while the firmware is up and scanning.
    pub heard: Option<Heard<'a>>,
}

/// Where a request goes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Route {
    /// net_core's link protocol: the radio's link.
    Link,
    /// A join op (connect, disconnect, link) the radio runs.
    Join(u16),
    /// The rest of the control family, answered by `answer`.
    Control,
    /// This driver's own protocol.
    Driver,
}

/// Where `req` goes; `can_join` is whether the radio runs joins now.
pub fn route(req: &[u8], can_join: bool) -> Route {
    let Some(magic) = req.get(..4) else { return Route::Driver };
    match u32::from_le_bytes([magic[0], magic[1], magic[2], magic[3]]) {
        NNET_MAGIC => Route::Link,
        WIFI_MAGIC if req.len() >= WIFI_HDR => {
            let op = u16::from_le_bytes([req[4], req[5]]);
            if can_join && matches!(op, OP_CONNECT | OP_DISCONNECT | OP_LINK) {
                Route::Join(op)
            } else {
                Route::Control
            }
        }
        _ => Route::Driver,
    }
}

fn put32(out: &mut [u8], at: usize, v: u32) {
    out[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

/// Answer a control request into `out` (at least `REPLY_MAX` bytes). `None`
/// when `req` is not one, so it goes on to this driver's own protocol.
pub fn answer(req: &[u8], view: &View<'_>, out: &mut [u8]) -> Option<usize> {
    if req.len() < WIFI_HDR || out.len() < REPLY_MAX {
        return None;
    }
    if u32::from_le_bytes([req[0], req[1], req[2], req[3]]) != WIFI_MAGIC {
        return None;
    }
    out[..WIFI_HDR].copy_from_slice(&req[..WIFI_HDR]);
    let op = u16::from_le_bytes([req[4], req[5]]);
    let n = match (op, &view.heard) {
        (OP_STATUS, heard) => {
            out[WIFI_HDR] = view.stage;
            out[WIFI_HDR + 1] = view.step;
            let (sweeps, stalls, frames, beacons) =
                heard.as_ref().map_or((0, 0, 0, 0), |h| (h.sweeps, h.stalls, h.frames, h.beacons));
            let words = [view.detail, view.hw_rev, view.rf_id, sweeps, stalls, frames, beacons];
            for (i, w) in words.iter().enumerate() {
                put32(out, WIFI_HDR + 2 + 4 * i, *w);
            }
            STATUS_LEN
        }
        (OP_SCAN, Some(h)) => {
            put32(out, WIFI_HDR, h.sweeps);
            put32(out, WIFI_HDR + 4, h.frames);
            put32(out, WIFI_HDR + 8, h.beacons);
            SCAN_STATS + h.list.encode(&mut out[WIFI_HDR + SCAN_STATS..])
        }
        _ => {
            out[WIFI_HDR..WIFI_HDR + 4].copy_from_slice(&E_NO_OP.to_le_bytes());
            4
        }
    };
    Some(WIFI_HDR + n)
}
