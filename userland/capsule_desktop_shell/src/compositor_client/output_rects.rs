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

//! NCMP 0x0021 OUTPUT_RECTS: where each output sits on the canvas, so the
//! lock screen draws its clock and field on every one.

use alloc::vec::Vec;

use super::wire::call_payload_within;

const OP: u16 = 0x0021;
const SLOTS: usize = 4;
const REPLY_LEN: usize = 4 + 16 * SLOTS;
const TIMEOUT_MS: u64 = 300;

pub fn output_rects(port: u32, request_id: u32) -> Option<Vec<(u32, u32, u32, u32)>> {
    let mut body = [0u8; REPLY_LEN];
    match call_payload_within(port, OP, request_id, &[], &mut body, TIMEOUT_MS) {
        Ok(0) => {}
        _ => return None,
    }
    let at = |i: usize| u32::from_le_bytes([body[i], body[i + 1], body[i + 2], body[i + 3]]);
    let n = (at(0) as usize).min(SLOTS);
    let rects: Vec<_> = (0..n)
        .map(|k| {
            let o = 4 + k * 16;
            (at(o), at(o + 4), at(o + 8), at(o + 12))
        })
        .filter(|r| r.2 > 0 && r.3 > 0)
        .collect();
    (!rects.is_empty()).then_some(rects)
}
