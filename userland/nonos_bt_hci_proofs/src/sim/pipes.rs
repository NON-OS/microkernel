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

//! The USB side of the simulated controller. Each event it sends is cut
//! into interrupt transfers of at most one endpoint packet, as a device
//! sends a long event (Vol 4, Part B, 2.1).

use std::collections::VecDeque;

use super::config::Config;
use nonos_bt_hci::usb::Setup;

pub struct SimPipes {
    pub cfg: Config,
    /// The interrupt endpoint's max packet.
    pub packet: usize,
    /// Each command with how many transfers were waiting when it came.
    pub commands: Vec<(Setup, Vec<u8>, usize)>,
    pub interrupt: VecDeque<Vec<u8>>,
    pub bulk_out: Vec<Vec<u8>>,
    pub bulk_in: VecDeque<Vec<u8>>,
    /// Bytes the device takes of each bulk OUT, if not all.
    pub bulk_take: Option<usize>,
    pub pauses: u32,
}

impl SimPipes {
    pub fn new(cfg: Config) -> Self {
        let (interrupt, bulk_in) = (VecDeque::new(), VecDeque::new());
        let (commands, bulk_out) = (Vec::new(), Vec::new());
        Self { cfg, packet: 16, commands, interrupt, bulk_out, bulk_in, bulk_take: None, pauses: 0 }
    }

    /// Queue `event` as the device would send it.
    pub fn send_event(&mut self, event: &[u8]) {
        self.interrupt.extend(event.chunks(self.packet).map(<[u8]>::to_vec));
    }
}

pub(super) fn give(q: &mut VecDeque<Vec<u8>>, out: &mut [u8]) -> Result<Option<usize>, i32> {
    let Some(t) = q.pop_front() else { return Ok(None) };
    assert!(t.len() <= out.len(), "a transfer longer than the armed length is babble");
    out[..t.len()].copy_from_slice(&t);
    Ok(Some(t.len()))
}
