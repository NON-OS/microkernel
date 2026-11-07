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

//! The signed bodies, each the proofs' default with its window moved.

use super::machine::Machine;
use super::window::Window;
use crate::kit::attest_build::keys::{hybrid, AUTHORITY};
use crate::kit::host::spec;
use crate::kit::roles_build::grant::GrantB;
use crate::kit::roles_build::revoke::RevB;
use crate::kit::roles_build::table::TableB;
use nsh_roles::MAX_REVOCATION_AGE_MS;

/// The shipped role table, trusting a list for as long as the format
/// allows.
pub fn table(w: &Window) -> TableB {
    TableB { issued_ms: w.opens_ms, max_age_ms: MAX_REVOCATION_AGE_MS, ..TableB::default() }
}

pub fn revocations(w: &Window) -> RevB {
    RevB { issued_ms: w.opens_ms, ..RevB::default() }
}

/// The user's base grant: `operator`, from the window's opening.
pub fn grant(w: &Window) -> GrantB {
    GrantB { not_before_ms: w.opens_ms, not_after_ms: w.grant_ends_ms, ..GrantB::default() }
}

/// The host's `nsh-enrol/1` certificate, signed by the fleet authority,
/// naming machine `m` and the host keys it made, `public`.
pub fn enrol_for(w: &Window, m: &Machine, public: &[u8; 32 + 1952]) -> Vec<u8> {
    let mut cert = spec().cert();
    cert.not_after_ms = w.enrol_ends_ms;
    cert.host_keys = nsh_crypto::key_digest(&public[..32], &public[32..]);
    cert.ak = m.ak;
    cert.pcrs = m.pcrs;
    cert.boot_apps = m.boot_apps.clone();
    cert.sign(hybrid(AUTHORITY))
}
