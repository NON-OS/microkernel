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

//! A route from an output pin back to a DAC.
//!
//! Linux searches the same graph (`snd_hda_parse_nid_path` in
//! hda_generic.c): from the pin through its connection list, through
//! mixers and selectors, to an analog converter, shortest route first. A
//! selector, or a pin with more than one connection, passes one input at a
//! time, so two outputs may share it only by selecting the same input; a
//! mixer sums all of its inputs and is shared freely.

use super::widget::{Codec, Widget};
use crate::constants::{WIDGET_TYPE_DAC, WIDGET_TYPE_MIXER, WIDGET_TYPE_PIN, WIDGET_TYPE_SELECTOR};

/// Pin, then at most four mixers or selectors, then the DAC. Real codecs
/// route a pin to a DAC in one to three hops.
pub const MAX_DEPTH: usize = 6;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Path {
    /// `nodes[0]` is the pin and `nodes[len - 1]` the DAC.
    pub nodes: [u8; MAX_DEPTH],
    /// `sel[i]` is the index in `nodes[i]`'s connection list of `nodes[i + 1]`.
    pub sel: [u8; MAX_DEPTH],
    pub len: u8,
}

impl Path {
    pub fn dac(&self) -> u8 {
        self.nodes[self.len as usize - 1]
    }

    pub fn hops(&self) -> impl Iterator<Item = (u8, u8)> + '_ {
        (0..self.len as usize).map(|i| (self.nodes[i], self.sel[i]))
    }

    /// The input this path needs `nid` to select, if `nid` is on it and
    /// passes one input at a time.
    fn selection_of(&self, codec: &Codec, nid: u8) -> Option<u8> {
        let i = self.nodes[..self.len as usize - 1].iter().position(|&n| n == nid)?;
        let w = codec.get(nid)?;
        if exclusive(w) {
            Some(self.sel[i])
        } else {
            None
        }
    }
}

/// A widget that passes one of its inputs, chosen by SET_CONNECT_SEL.
fn exclusive(w: &Widget) -> bool {
    let ty = w.ty();
    (ty == WIDGET_TYPE_SELECTOR || ty == WIDGET_TYPE_PIN) && w.n_conn > 1
}

/// The shortest path from `pin` to an analog DAC that plays 48 kHz 16-bit,
/// agreeing with every path in `taken` on the inputs shared selectors pass.
pub fn find(codec: &Codec, pin: u8, taken: &[Path]) -> Option<Path> {
    let mut p = Path { nodes: [0; MAX_DEPTH], sel: [0; MAX_DEPTH], len: 1 };
    p.nodes[0] = pin;
    let mut limit = 2usize;
    while limit <= MAX_DEPTH {
        if step(codec, &mut p, limit, taken) {
            return Some(p);
        }
        limit += 1;
    }
    None
}

fn step(codec: &Codec, p: &mut Path, limit: usize, taken: &[Path]) -> bool {
    let at = p.len as usize - 1;
    let Some(w) = codec.get(p.nodes[at]) else { return false };
    if at + 1 >= limit {
        return false;
    }
    let fixed = taken.iter().find_map(|t| t.selection_of(codec, w.nid));
    for (i, &child) in w.connections().iter().enumerate() {
        if child == 0 || p.nodes[..=at].contains(&child) {
            continue;
        }
        if exclusive(w) && fixed.is_some_and(|f| f as usize != i) {
            continue;
        }
        let Some(c) = codec.get(child) else { continue };
        if c.is_digital() {
            continue;
        }
        p.sel[at] = i as u8;
        p.nodes[at + 1] = child;
        p.len = (at + 2) as u8;
        let ty = c.ty();
        if ty == WIDGET_TYPE_DAC {
            if c.plays_48k16() {
                return true;
            }
        } else if (ty == WIDGET_TYPE_MIXER || ty == WIDGET_TYPE_SELECTOR)
            && step(codec, p, limit, taken)
        {
            return true;
        }
        p.len = (at + 1) as u8;
    }
    false
}
