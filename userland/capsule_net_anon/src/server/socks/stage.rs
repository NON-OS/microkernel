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

//! How far a conversation has got, and the bounds it keeps to.

/// Largest answer handed back in one exchange. The caller takes up to 36 KiB
/// and an API reply carries up to 32 KiB; one byte goes to the marker.
pub const OUT_MAX: usize = 32 * 1024 - 1;

/// Bytes from the caller the stream may hold unsent. Past this the caller is
/// writing faster than the exit grants windows, which a browser never does
/// inside one stream window, so the conversation ends rather than buffering
/// without bound.
pub const UNSENT_MAX: usize = 256 * 1024;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Stage {
    #[default]
    Greeting,
    Request,
    Connecting(u16),
    Relay(u16),
}

/// Where one stage left the conversation.
pub(super) enum Step {
    Next,
    Wait,
    Over,
}
