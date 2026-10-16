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

//! The model's register file. `regs` is what a read returns, the last
//! value written (double-buffered registers read so); `live` the latched
//! copies the scanout uses. Every `period` reads of a frame counter or
//! live address is one vertical blank on every pipe.

use crate::engine::gen::PlaneKind;
use std::collections::HashMap;

pub struct Gen9 {
    pub regs: HashMap<u32, u32>,
    pub live: HashMap<u32, u32>,
    pub armed: [bool; 4], // plane 1 armed, per pipe; then the cursor
    pub cursor_armed: [bool; 4],
    pub ggtt: Vec<u64>,
    pub flushes: u32,
    pub memory: HashMap<u64, Vec<u8>>, // IOVA page -> its 4096 bytes
    pub period: u32,
    pub clock: u32,
    pub writes: Vec<(u32, u32)>,
    /// A DisplayPort sink behind a DDI's AUX; a channel that never ends.
    pub sink: Option<(crate::engine::aux_regs::Ddi, super::sink::Sink)>,
    pub aux_stuck: bool,
    pub pending: Vec<(u32, u32, u32)>, // landing a few reads later
    pub dead: bool,
    pub ddc: super::gmbus_state::Ddc,
    pub plane: PlaneKind, // Gen9's unless a test models another
    pub cable: super::cable::Cable,
}

impl Gen9 {
    pub fn new(ggtt_entries: usize) -> Self {
        Gen9 {
            regs: HashMap::new(),
            live: HashMap::new(),
            armed: [false; 4],
            cursor_armed: [false; 4],
            ggtt: vec![0; ggtt_entries],
            flushes: 0,
            memory: HashMap::new(),
            period: 3,
            clock: 0,
            writes: Vec::new(),
            sink: None,
            aux_stuck: false,
            pending: Vec::new(),
            dead: false,
            ddc: Default::default(),
            plane: PlaneKind::Universal,
            cable: Default::default(),
        }
    }
}
