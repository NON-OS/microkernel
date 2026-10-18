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

//! INPUT_SOURCES: the inputs each display offers, for the shell's menu. A
//! display's own VCP reading of the input source carries the values its
//! capabilities list (the driver's VCP body: now u16, max u16, n u8 and n
//! values); the answer to the shell is a count u32, then per display its
//! identity u32, the input now u16, n u8, a zero byte and eight values.

pub const OP_VCP: u8 = 23;
/// MCCS's input source control.
pub const INPUT_SOURCE: u8 = 0x60;
pub const MAX_DISPLAYS: usize = 3;
pub const MAX_INPUTS: usize = 8;
const ENTRY: usize = 16;
pub const INPUT_SOURCES_LEN: usize = 4 + MAX_DISPLAYS * ENTRY;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Offered {
    pub identity: u32,
    pub now: u16,
    pub inputs: [u8; MAX_INPUTS],
    pub n: u8,
}

/// The driver's VCP request for the input source on panel `identity`.
pub fn request(identity: u32) -> [u8; 6] {
    let i = identity.to_le_bytes();
    [OP_VCP, i[0], i[1], i[2], i[3], INPUT_SOURCE]
}

/// What a VCP answer says panel `identity` offers: None for a refusal, a
/// body shorter than it says, or a display that lists no input. Past
/// eight listed inputs the rest are not offered.
pub fn offered(identity: u32, reply: &[u8]) -> Option<Offered> {
    let (status, body) = (reply.get(..4)?, reply.get(4..)?);
    let (head, n) = (body.get(..5).filter(|_| status == [0; 4])?, *body.get(4)? as usize);
    let listed = body.get(5..5 + n).filter(|l| !l.is_empty())?;
    let mut o =
        Offered { identity, now: u16::from_le_bytes([head[0], head[1]]), ..Offered::default() };
    for (slot, v) in o.inputs.iter_mut().zip(listed) {
        (*slot, o.n) = (*v, o.n + 1);
    }
    Some(o)
}

/// The answer to the shell, the first three displays that offer inputs.
pub fn encode(list: &[Offered]) -> [u8; INPUT_SOURCES_LEN] {
    let mut out = [0u8; INPUT_SOURCES_LEN];
    let shown = &list[..list.len().min(MAX_DISPLAYS)];
    out[..4].copy_from_slice(&(shown.len() as u32).to_le_bytes());
    for (o, e) in shown.iter().zip(out[4..].chunks_exact_mut(ENTRY)) {
        e[..4].copy_from_slice(&o.identity.to_le_bytes());
        e[4..6].copy_from_slice(&o.now.to_le_bytes());
        e[6] = o.n;
        e[8..].copy_from_slice(&o.inputs);
    }
    out
}
