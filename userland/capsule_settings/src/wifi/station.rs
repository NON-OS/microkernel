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
//! Where the wired card's station (MAC) address came from, as its driver
//! says in word 9 of NNET's OP_STATS (nonos_usbnet nnet/wire.rs, stats.rs),
//! and the words the Network page shows for it. Only Hyper-V's netvsc
//! reports it; a driver that always serves the address it drew for the boot
//! leaves it 0, and the page shows nothing. Pure; held in
//! capsule_settings_proofs.

const MAGIC: u32 = 0x4E4E_4554;
const VERSION: u16 = 1;
pub const OP_STATS: u16 = 7;
pub const HDR_LEN: usize = 20;
const DATA_AT: usize = HDR_LEN + 4;
pub const REPLY_LEN: usize = DATA_AT + 48;

/// The OP_STATS request numbered `id`.
pub fn stats_request(id: u32) -> [u8; HDR_LEN] {
    let mut r = [0u8; HDR_LEN];
    r[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    r[4..6].copy_from_slice(&VERSION.to_le_bytes());
    r[6..8].copy_from_slice(&OP_STATS.to_le_bytes());
    r[12..16].copy_from_slice(&id.to_le_bytes());
    r
}

/// Word 9 of an OK answer to request `id`; None for anything else.
pub fn station_of(reply: &[u8], id: u32) -> Option<u32> {
    let word = |at: usize| Some(u32::from_le_bytes(reply.get(at..at + 4)?.try_into().ok()?));
    let half = |at: usize| u16::from_le_bytes([reply[at], reply[at + 1]]);
    let ours = reply.len() >= REPLY_LEN
        && word(0)? == MAGIC
        && half(4) == VERSION
        && half(6) == OP_STATS
        && word(12)? == id
        && word(HDR_LEN)? == 0;
    if !ours {
        return None;
    }
    word(DATA_AT + 9 * 4)
}

/// What the page says for a source: the value, and the line under the
/// label when the value needs one. None shows nothing.
pub fn said(source: u32) -> Option<(&'static str, Option<&'static str>)> {
    match source {
        1 => Some(("Private, new each boot", None)),
        2 | 3 => Some(("Assigned by Hyper-V", Some(HYPER_V))),
        _ => None,
    }
}

/// Why Hyper-V's address is used, as the project worded it.
pub const HYPER_V: &str =
    "Hyper-V drops a chosen address unless MAC spoofing is on for this VM's adapter.";
