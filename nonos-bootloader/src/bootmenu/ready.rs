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

//! What an entry will need from this machine, read before it is chosen so
//! the menu can say up front that a boot will be refused. It mirrors the
//! policy in security/enforce (core.rs and modes/), which stays the only
//! check: this list decides nothing, it only names what that check will.

use crate::menu::{MenuAction, SecurityMode};
use crate::security::{SecurityContext, SecurityPolicy};

/// Up to eight unmet requirements, as short names.
pub(super) struct Missing {
    pub names: [&'static [u8]; 8],
    pub len: usize,
}

impl Missing {
    fn add(&mut self, unmet: bool, name: &'static [u8]) {
        if unmet && self.len < self.names.len() {
            self.names[self.len] = name;
            self.len += 1;
        }
    }
}

/// The policy `action` runs under: the menu can raise the build floor, never
/// lower it. None for an entry that boots nothing.
pub(super) fn policy_of(action: MenuAction) -> Option<SecurityPolicy> {
    let asked = match action {
        MenuAction::Shutdown => return None,
        MenuAction::Boot(SecurityMode::Hardened) => SecurityPolicy::Hardened,
        _ => SecurityPolicy::Standard,
    };
    Some(SecurityPolicy::from_build().stricter(asked))
}

pub(super) fn missing(s: &SecurityContext, policy: SecurityPolicy) -> Missing {
    let mut m = Missing { names: [b""; 8], len: 0 };
    m.add(!s.blake3_health_ok || !s.ed25519_health_ok, b"crypto self-test");
    m.add(!s.production_keys_loaded || s.key_count == 0, b"signing keys");
    if policy == SecurityPolicy::Development {
        return m;
    }
    m.add(!s.hardware_rng_available, b"hardware RNG");
    if policy == SecurityPolicy::Hardened {
        m.add(!s.secure_boot_enabled, b"Secure Boot");
        m.add(!s.platform_key_verified, b"PK");
        m.add(!s.signature_database_valid, b"db");
        m.add(!s.measured_boot_active, b"TPM 2.0");
    }
    m
}
