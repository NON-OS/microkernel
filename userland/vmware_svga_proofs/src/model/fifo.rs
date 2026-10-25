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

//! The FIFO, run as QEMU runs it on SYNC: every command from STOP to
//! NEXT_CMD, word by word with the wrap, STOP moved past each.

use super::svga::Svga;

impl Svga {
    fn word(&self, i: u32) -> u32 {
        assert!((i as usize) * 4 < self.fifo.1, "FIFO word {i} past the memory");
        // SAFETY: inside the FIFO's host memory, which outlives the model.
        unsafe { ((self.fifo.0 as usize + i as usize * 4) as *const u32).read_volatile() }
    }

    fn set_word(&self, i: u32, v: u32) {
        // SAFETY: as for `word`.
        unsafe { ((self.fifo.0 as usize + i as usize * 4) as *mut u32).write_volatile(v) }
    }

    pub(super) fn run_fifo(&mut self) {
        if self.config_done == 0 || self.enable == 0 {
            return;
        }
        let (min, max, next) = (self.word(0), self.word(1), self.word(2));
        assert!(
            min >= 16 && max <= self.fifo.1 as u32 && max >= min + 10 * 1024,
            "FIFO layout QEMU refuses"
        );
        let mut stop = self.word(3);
        let take = |stop: &mut u32| {
            let v = self.word(*stop / 4);
            *stop = if *stop + 4 >= max { min } else { *stop + 4 };
            v
        };
        let mut got = Vec::new();
        while stop != next {
            let cmd = take(&mut stop);
            assert_eq!(cmd, 1, "only SVGA_CMD_UPDATE is sent");
            got.push((take(&mut stop), take(&mut stop), take(&mut stop), take(&mut stop)));
        }
        self.set_word(3, stop);
        self.updates.extend(got);
    }
}
