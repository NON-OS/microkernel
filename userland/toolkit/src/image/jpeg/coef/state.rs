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

use alloc::vec::Vec;

use super::frame::Frame;
use super::huff::Coder;
use super::plane::Plane;
use crate::image::jpeg::dht::HuffmanTable;
use crate::image::jpeg::dqt::QuantTable;

mod begin;
mod header;

/// Everything the marker segments before and between scans establish.
pub struct State {
    k: usize,
    max_coef: usize,
    pub frame: Option<Frame>,
    pub planes: Vec<Plane>,
    qt: [QuantTable; 4],
    /* Each component's quant table in natural order, fixed at its first
     * scan as the progressive process requires. */
    pub qnat: [[u16; 64]; 4],
    taken: [bool; 4],
    tables: [[HuffmanTable; 4]; 2],
    pub dc: Vec<Coder>,
    pub ac: Vec<Coder>,
    pub ri: usize,
    pub adobe: Option<u8>,
}

impl State {
    pub fn new(k: usize, max_coef: usize) -> State {
        let t = || [0; 4].map(|_| HuffmanTable::new());
        let coders = |_| (0..4).map(|_| Coder::new(HuffmanTable::new())).collect::<Vec<_>>();
        let (qt, tables) = ([QuantTable::new(); 4], [t(), t()]);
        let (dc, ac) = (coders(0), coders(1));
        State {
            k,
            max_coef,
            frame: None,
            planes: Vec::new(),
            qt,
            qnat: [[0; 64]; 4],
            taken: [false; 4],
            tables,
            dc,
            ac,
            ri: 0,
            adobe: None,
        }
    }
}
