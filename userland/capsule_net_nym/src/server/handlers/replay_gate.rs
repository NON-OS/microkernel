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

//! Refusing a gateway frame that has arrived before.

use spin::Mutex;

use crate::gateway_client::NONCE_BYTES;
use crate::packet::REPLAY_TAG_LEN;
use crate::state::ReplayWindow;

static SEEN: Mutex<ReplayWindow> = Mutex::new(ReplayWindow::new());

/// True the first time this nonce is offered, false every time after.
pub(super) fn fresh(nonce: &[u8; NONCE_BYTES]) -> bool {
    let mut tag = [0u8; REPLAY_TAG_LEN];
    /*
     * Left aligned and carried whole: twelve bytes into thirty-two,
     * never folded down, since truncating invents collisions.
     */
    tag[..NONCE_BYTES].copy_from_slice(nonce);
    SEEN.lock().accept(&tag)
}
