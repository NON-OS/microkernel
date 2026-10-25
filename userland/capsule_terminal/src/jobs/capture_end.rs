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

//! Writing the held output when the program ends, or dropping it on Ctrl+C.

use crate::command::dispatch::{not_written, write_out, wrote_to};
use crate::command::output::Output;

use super::Capture;

impl Capture {
    /// The program ended: write the file and say how that went. False when
    /// the file does not hold all the program wrote.
    pub fn finish(&mut self, out: &mut Output<'_>) -> bool {
        if self.path.is_empty() {
            return true;
        }
        let held = core::mem::take(&mut self.held);
        if let Err(e) = write_out(self.owner, &self.path, &held, self.append) {
            out.writeln_error(&not_written(&self.path, e));
            return false;
        }
        out.writeln(&wrote_to(&self.path));
        if self.over > 0 {
            out.writeln_error(&[&self.path[..], b": the output past 1 MiB was not kept"].concat());
            return false;
        }
        true
    }

    /// The program was stopped with Ctrl+C: what it wrote is dropped, with
    /// the file left as it stood when it started.
    pub fn interrupted(&self, out: &mut Output<'_>) {
        if !self.path.is_empty() {
            out.writeln_error(&not_written(&self.path, "the program was interrupted"));
        }
    }
}
