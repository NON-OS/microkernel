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

//! One request answered. A reply can carry a key or recovery words (export,
//! shield material) and a request can carry a secret (import, restore), so
//! both buffers are wiped once the reply has gone, and before either is reused
//! or freed.

use super::wipe::wipe;

/// Sends `reply` with `send`, then wipes `reply` and `request` in full.
pub(crate) fn answer(request: &mut [u8], reply: &mut [u8], send: impl FnOnce(&[u8])) {
    send(reply);
    wipe(reply);
    wipe(request);
}
