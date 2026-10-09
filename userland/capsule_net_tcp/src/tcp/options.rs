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

const END: u8 = 0;
const NOP: u8 = 1;
const MSS: u8 = 2;

/*
 * The options area between the fixed header and the data (RFC 9293 3.1):
 * end-of-list and no-operation are one byte, every other option is a kind,
 * a length counting both, and that many bytes. A length under two or past
 * the area ends the walk with what was read so far, so no length a sender
 * writes can step outside the header. Only the MSS is used. A window scale
 * option is read past and not applied: this stack never sends one, and RFC
 * 7323 applies scaling only when both SYNs carry it.
 */
pub fn peer_mss(options: &[u8]) -> Option<u16> {
    let mut mss = None;
    let mut at = 0usize;
    while let Some(&kind) = options.get(at) {
        match kind {
            END => break,
            NOP => at += 1,
            _ => {
                let Some(&len) = options.get(at + 1) else { break };
                let len = usize::from(len);
                let Some(body) = options.get(at + 2..at + len.max(2)) else { break };
                if len < 2 {
                    break;
                }
                if kind == MSS && body.len() == 2 {
                    mss = Some(u16::from_be_bytes([body[0], body[1]]));
                }
                at += len;
            }
        }
    }
    mss
}
