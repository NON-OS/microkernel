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

//! The display function's interrupt as the model raises it: a pipe's
//! vertical blank sets VBLANK in its DE_PIPE_IIR when DE_PIPE_IMR lets it
//! (the IMR resets masked, so an IMR never written masks it), and a
//! message goes out each time the line rises: an unmasked, enabled IIR
//! bit with Gen8's master switch on. MSI is edge signalled, so a bit left
//! standing raises nothing more.

use super::gen9::Gen9;
use crate::engine::irq_regs::{ENABLE, MASTER_INT_CTL, VBLANK};
use crate::engine::pipe::Pipe;

#[derive(Default)]
pub struct Msi {
    pub line: bool,
    /// Messages sent so far.
    pub sent: u32,
}

const fn de(p: Pipe, at: u32) -> u32 {
    0x4_4400 + 0x10 * (p as u32) + at
}

impl Gen9 {
    pub(super) fn blank_event(&mut self, p: Pipe) {
        if self.regs.get(&de(p, 4)).is_some_and(|imr| imr & VBLANK == 0) {
            let iir = self.get(de(p, 8));
            self.set(de(p, 8), iir | VBLANK);
        }
    }

    pub fn irq_line(&self) -> bool {
        let on = self.get(MASTER_INT_CTL) & ENABLE != 0;
        on && Pipe::ALL.iter().any(|&p| self.get(de(p, 8)) & self.get(de(p, 0xC)) & VBLANK != 0)
    }

    pub(super) fn msi_edge(&mut self) {
        let line = self.irq_line();
        if line && !self.msi.line {
            self.msi.sent += 1;
        }
        self.msi.line = line;
    }
}
