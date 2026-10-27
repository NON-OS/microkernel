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

//! Which transport the window runs on.

use alloc::boxed::Box;

use super::core_channel::CoreChannel;
use super::transcript::{Transcript, SESSION};
use super::Transport;

/// The channel to the core when it is up, else the recorded session.
pub fn open() -> Box<dyn Transport> {
    let core = CoreChannel;
    if core.up() {
        return Box::new(core);
    }
    Box::new(Transcript::new(SESSION))
}
