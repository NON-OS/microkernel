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

use alloc::sync::Arc;
use core::sync::atomic::{AtomicU64, Ordering};

use crate::capabilities::{bits_to_caps, CapabilityToken};

static TOKEN_ID: AtomicU64 = AtomicU64::new(1);

/// What a process token is bound to besides its permissions.
#[derive(Clone, Copy)]
pub(crate) struct Binding {
    pub pid: u32,
    pub asid: u32,
    pub epoch: u64,
    pub boot_nonce: [u8; 16],
}

/// A signed process token for `bits`; None without a signing key.
pub(crate) fn mint(bits: u64, b: Binding) -> Option<Arc<CapabilityToken>> {
    let mut tok = CapabilityToken {
        owner_module: b.pid as u64,
        permissions: bits_to_caps(bits),
        expires_at_ms: None,
        nonce: 0,
        signature: [0u8; 64],
        token_id: TOKEN_ID.fetch_add(1, Ordering::Relaxed),
        subject_capsule_id: b.pid,
        subject_asid: b.asid,
        subject_measurement: [0u8; 32],
        boot_session_nonce: b.boot_nonce,
        revocation_epoch: b.epoch,
        delegation_depth: 0,
    };
    crate::capabilities::token::sign_token(&mut tok).ok()?;
    Some(Arc::new(tok))
}
