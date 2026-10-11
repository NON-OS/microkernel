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

//! The devices still in the boot identity domain: enumerated, never claimed
//! by a capsule, and able to reach all of RAM, the kernel included. The
//! posture line counts only broker grants, so without this number "no
//! unconfined grants" reads as more confinement than the machine has.

use core::sync::atomic::{AtomicU16, Ordering};

use super::state::STATE;

/// The identity domain's id; 0 is never handed out, so it means "none yet".
static IDENTITY: AtomicU16 = AtomicU16::new(0);

/// Called once by bring-up, with the domain it built.
pub fn set_identity_domain(id: u16) {
    IDENTITY.store(id, Ordering::Release);
}

/// How many devices still translate through the identity domain.
pub fn identity_devices() -> usize {
    let id = IDENTITY.load(Ordering::Acquire);
    if id == 0 {
        return 0;
    }
    STATE.lock().bindings.iter().filter(|b| b.domain.as_u16() == id).count()
}
