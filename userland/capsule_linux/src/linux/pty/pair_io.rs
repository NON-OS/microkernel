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

//! A pseudo-terminal's traffic and settings: the master typing, the slave
//! writing, and new settings and window sizes. Pure.

use alloc::vec::Vec;

use super::flags::Termios;
use super::input::MOST;
use super::out::{post, OUT_MOST};
use super::pair::Pair;

impl Pair {
    /// The master writes `bytes`: how many the discipline took, and the
    /// signals they raised, in order. It stops once the slave has a full
    /// buffer unread; a line being edited past it loses all but its end.
    pub fn typed(&mut self, bytes: &[u8]) -> (usize, Vec<u8>) {
        let (t, mut signals, mut n) = (self.termios, Vec::new(), 0);
        for &c in bytes {
            if self.input.ready.len() >= MOST {
                break;
            }
            signals.extend(super::disc::receive(&t, &mut self.input, &mut self.output, c));
            n += 1;
        }
        (n, signals)
    }

    /// The slave writes `bytes`: how many the master side had room for.
    pub fn written(&mut self, bytes: &[u8]) -> usize {
        let n = bytes.len().min(OUT_MOST.saturating_sub(self.output.len()));
        post(&self.termios, &bytes[..n], &mut self.output);
        n
    }

    /// New settings. Leaving canonical mode makes the line being edited
    /// readable as it stands, as n_tty does.
    pub fn set_termios(&mut self, t: Termios) {
        self.termios = t;
        if !self.canon() && !self.input.line.is_empty() {
            let line = core::mem::take(&mut self.input.line);
            self.input.ready.extend(line);
        }
    }

    /// A new window size; true when it changed, which raises SIGWINCH.
    pub fn set_winsize(&mut self, ws: [u8; 8]) -> bool {
        core::mem::replace(&mut self.winsize, ws) != ws
    }
}
