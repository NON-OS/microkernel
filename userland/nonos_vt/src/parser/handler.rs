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

//! What the parser reports as it recognises each piece of the byte stream.

use super::seq::Seq;

pub trait Handler {
    /// A character to draw.
    fn print(&mut self, c: char);
    /// A C0 control byte.
    fn execute(&mut self, b: u8);
    /// `ESC [ ... final`.
    fn csi(&mut self, seq: &Seq);
    /// `ESC intermediates final`.
    fn esc(&mut self, seq: &Seq);
    /// `ESC ] ... BEL` or `ESC ] ... ESC \`.
    fn osc(&mut self, data: &[u8]);
    /// `ESC P ... final data ESC \`.
    fn dcs(&mut self, seq: &Seq, data: &[u8]);
}
