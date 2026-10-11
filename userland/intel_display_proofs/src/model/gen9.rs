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

//! The model's register file. `regs` is what a read returns: the last
//! value written, as the PRM says reads of double-buffered registers do.
//! `live` holds the latched copies the scanout uses. Time passes as the
//! code reads the frame counter or a live address: every `period` such
//! reads is one vertical blank on every pipe.

use std::collections::HashMap;

pub struct Gen9 {
    pub regs: HashMap<u32, u32>,
    pub live: HashMap<u32, u32>,
    /// Pipes with plane 1 armed by PLANE_SURF, and with the cursor armed.
    pub armed: [bool; 3],
    pub cursor_armed: [bool; 3],
    pub ggtt: Vec<u64>,
    pub flushes: u32,
    /// IOVA page -> 4096 bytes, the memory the engine reads through GGTT.
    pub memory: HashMap<u64, Vec<u8>>,
    pub period: u32,
    pub clock: u32,
    pub writes: Vec<(u32, u32)>,
    /// A DisplayPort sink behind one DDI's AUX channel, and a channel
    /// that never finishes when set.
    pub aux: Option<(crate::engine::aux_regs::Ddi, Box<dyn crate::dp::aux::Aux>)>,
    pub aux_stuck: bool,
}

impl Gen9 {
    pub fn new(ggtt_entries: usize) -> Self {
        Gen9 {
            regs: HashMap::new(),
            live: HashMap::new(),
            armed: [false; 3],
            cursor_armed: [false; 3],
            ggtt: vec![0; ggtt_entries],
            flushes: 0,
            memory: HashMap::new(),
            period: 3,
            clock: 0,
            writes: Vec::new(),
            aux: None,
            aux_stuck: false,
        }
    }

    pub fn get(&self, off: u32) -> u32 {
        *self.regs.get(&off).unwrap_or(&0)
    }

    pub fn set(&mut self, off: u32, v: u32) {
        self.regs.insert(off, v);
        self.live.insert(off, v);
    }
}
