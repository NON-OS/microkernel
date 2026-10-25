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

//! Which apps another process may start through OP_OPEN_WITH: those that open
//! a file handed to them, as Files asks. Any process with IPC can send it, and
//! the shell starts the app with its own authority as a launcher, so an app
//! holding more than a viewer (Install, the Terminal, Qwen, Settings) is never
//! started this way: the person starts those from the dock or the Launchpad.

/// The editor, the image viewer and the two players: Files' handlers.
pub const OPENERS: [&[u8]; 4] =
    [b"app.text_editor", b"app.image_viewer", b"app.audio_player", b"app.video_player"];

pub fn opens_files(service: &[u8]) -> bool {
    OPENERS.contains(&service)
}
