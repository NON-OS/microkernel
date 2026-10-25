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
//! The shell told that an unauthenticated Bluetooth device was refused on
//! the secure path (G-76). The notice is a delivery whose envelope flags
//! carry NOTICE_UNPROVEN and whose event is no event (NOTICE_KIND), so a
//! receiver that reads only the event drops it as a kind it does not know.
//! It goes to the shell alone, which believes only the router's frames.

use nonos_libc::{mk_ipc_send_to_pid, InputEvent};

use crate::protocol::{encode_delivery, DELIVERY_LEN, NOTICE_KIND, NOTICE_UNPROVEN};

/// Whether the notice reached `shell`.
pub(super) fn tell_shell(shell: u32) -> bool {
    if shell == 0 {
        return false;
    }
    let event = InputEvent { kind: NOTICE_KIND, ..InputEvent::default() };
    let mut frame = [0u8; DELIVERY_LEN];
    encode_delivery(&mut frame, &event);
    frame[6..8].copy_from_slice(&NOTICE_UNPROVEN.to_le_bytes());
    mk_ipc_send_to_pid(shell, frame.as_ptr(), frame.len()) >= 0
}
