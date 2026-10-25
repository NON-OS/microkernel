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

//! A token holding exactly the given caps, unexpired, for the map's tests.

extern crate alloc;
use alloc::vec::Vec;

use crate::capabilities::{Capability, CapabilityToken};

pub(super) fn token(permissions: Vec<Capability>) -> CapabilityToken {
    CapabilityToken {
        owner_module: 0,
        permissions,
        expires_at_ms: None,
        nonce: 0,
        signature: [0u8; 64],
        token_id: 0,
        subject_capsule_id: 0,
        subject_asid: 0,
        subject_measurement: [0u8; 32],
        boot_session_nonce: [0u8; 16],
        revocation_epoch: 0,
        delegation_depth: 0,
    }
}
