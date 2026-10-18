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
//! One report's fingers, read through the slots: each slot's tip,
//! confidence, identifier and position, the report's Contact Count (how
//! many fingers the whole frame has, in hybrid mode only on its first
//! report) and the clickpad button.

use super::super::decode::{holds, position};
use super::super::read_bits::read_bits;
use super::super::Field;
use super::slots::{Slots, MAX_SLOTS};

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Touch {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub tip: bool,
    /// A deliberate finger; true where the device does not say.
    pub confidence: bool,
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Frame {
    pub touches: [Touch; MAX_SLOTS],
    pub n: usize,
    /// Contact Count, or `None` where the report has none.
    pub count: Option<u32>,
    pub button: bool,
}

/// The report `body` (after its report id) read through `slots`; `None`
/// when it is too short for every field, which a torn read is.
pub fn decode_frame(body: &[u8], slots: &Slots, count: Field, button: Field) -> Option<Frame> {
    let mut need = [count; MAX_SLOTS * 5 + 2];
    let used = slots.list[..slots.n].iter().flat_map(|s| [s.tip, s.confidence, s.id, s.x, s.y]);
    for (n, f) in need.iter_mut().zip(used.chain([button])) {
        *n = f;
    }
    if !holds(body, &need) {
        return None;
    }
    let read = |f: Field| read_bits(body, f.bit_offset, f.bit_size);
    let mut frame =
        Frame { n: slots.n, button: button.present() && read(button) != 0, ..Frame::default() };
    frame.count = count.present().then(|| read(count));
    for (t, s) in frame.touches.iter_mut().zip(&slots.list[..slots.n]) {
        *t = Touch {
            id: read(s.id),
            x: position(body, &s.x).0,
            y: position(body, &s.y).0,
            tip: read(s.tip) != 0,
            confidence: !s.confidence.present() || read(s.confidence) != 0,
        };
    }
    Some(frame)
}
