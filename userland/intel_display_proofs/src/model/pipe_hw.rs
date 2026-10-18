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

//! Transcoders in the model: the state bit follows the enable a few
//! reads later, as the hardware starts or stops the timing generator.

use super::gen9::Gen9;
use crate::engine::pipe::Transcoder;
use crate::engine::regs::trans_conf;

impl Gen9 {
    pub(super) fn pipe_write(&mut self, off: u32, v: u32) {
        let all = [Transcoder::A, Transcoder::B, Transcoder::C, Transcoder::Edp];
        if self.dead || !all.iter().any(|&t| trans_conf(t) == off) {
            return;
        }
        let state = if v & (1 << 31) != 0 { 1 << 30 } else { 0 };
        self.pending.push((2, off, (v & !(1 << 30)) | state));
    }
}
