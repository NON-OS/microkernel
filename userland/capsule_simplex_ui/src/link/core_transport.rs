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

//! The channel to the core as the window sees it.

use alloc::string::String;

use super::core_channel::CoreChannel;
use super::pipe::Pipe;
use super::Transport;

impl<P: Pipe> Transport for CoreChannel<P> {
    fn send(&mut self, frame: &str) {
        if self.is_up() {
            let bytes = super::ws_frame::text(frame, self.next_mask());
            self.write_or_down(&bytes);
        }
    }

    fn recv(&mut self) -> Option<String> {
        self.frames.pop_front()
    }

    fn tick(&mut self) {
        self.pump();
    }

    fn up(&self) -> bool {
        self.is_up()
    }

    fn label(&self) -> &'static str {
        "simplex.core"
    }
}
