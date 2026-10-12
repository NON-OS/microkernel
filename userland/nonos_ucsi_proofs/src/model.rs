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

//! A PPM as the OPM sees it through the mailbox: a command written and
//! signalled completes after `delay` refreshes, or never when `stuck`;
//! connector changes queue and are shown in the CCI one at a time until
//! acknowledged. Every refresh, pause and command is counted.

use std::collections::VecDeque;

#[derive(Default)]
pub struct Model {
    /// Each connector's GET_CONNECTOR_STATUS reply (9 bytes used).
    pub ports: Vec<[u8; 16]>,
    pub cci: u32,
    pub message: [u8; 16],
    pub changes: VecDeque<u8>,
    pub notify_mask: u16,
    pub delay: u32,
    pub stuck: bool,
    pub refuse_status: bool,
    pub commands: Vec<u8>,
    pub reads: u32,
    pub pauses: u32,
    pub(crate) control: Option<(u64, u32)>,
}

impl Model {
    pub fn new(ports: usize) -> Self {
        Model { ports: vec![[0; 16]; ports], ..Model::default() }
    }

    /// A partner plugged into (or pulled from) connector `n`.
    pub fn plug(&mut self, n: u8, connected: bool, provider: bool) {
        let p = &mut self.ports[usize::from(n) - 1];
        p[0..2].copy_from_slice(&(1u16 << 14).to_le_bytes());
        let w = 3 | u16::from(connected) << 3 | u16::from(provider) << 4 | 2 << 13;
        p[2..4].copy_from_slice(&w.to_le_bytes());
        p[8] = u8::from(connected && !provider);
        if !self.changes.contains(&n) {
            self.changes.push_back(n);
        }
    }

    pub(crate) fn shown(&self) -> u32 {
        let c = if self.notify_mask != 0 { self.changes.front().copied().unwrap_or(0) } else { 0 };
        u32::from(c) << 1
    }
}
