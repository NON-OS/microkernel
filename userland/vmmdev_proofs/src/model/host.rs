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

//! The device's state: the event page, what the guest has told it, the
//! events pending, and the host's pointer.

#[derive(Clone, Copy)]
pub struct Options {
    /// The event page's version and size words, as the host writes them.
    pub memory_version: u32,
    pub memory_size: u32,
    /// The host never carries a request out: every status stays as written.
    pub deaf: bool,
    /// The host refuses the guest info.
    pub refuse_info: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self { memory_version: 1, memory_size: 0x0040_0000, deaf: false, refuse_info: false }
    }
}

#[derive(Default)]
pub struct Host {
    pub opt: Options,
    pub memory: Vec<u32>,
    /// Every request carried out or refused: its type and its status.
    pub requests: Vec<(u32, i32)>,
    pub info: Option<(u32, u32)>,
    pub filter: u32,
    pub caps: u32,
    pub guest_mouse: u32,
    pub pending: u32,
    /// The host's integration switch, and the pointer in the 0..0xFFFF range.
    pub integration: bool,
    pub at: (i32, i32),
    /// Moves the host sent to the PS/2 mouse instead, guest not absolute.
    pub ps2_moves: usize,
}

impl Host {
    pub fn new(opt: Options) -> Self {
        let mut memory = vec![0u32; 1024];
        memory[0] = opt.memory_size;
        memory[1] = opt.memory_version;
        Self { opt, memory, integration: true, ..Self::default() }
    }

    pub fn memory_va(&self) -> u64 {
        self.memory.as_ptr() as u64
    }

    /// The pending flag: events the guest's filter lets through.
    pub(super) fn raise(&mut self) {
        self.memory[2] = (self.pending & self.filter != 0) as u32;
    }

    pub fn types(&self) -> Vec<u32> {
        self.requests.iter().map(|r| r.0).collect()
    }
}
