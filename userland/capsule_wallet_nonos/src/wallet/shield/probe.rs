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

//! Asking whether a shield capsule is present, rather than assuming.
//!
//! The shielded screens were gated on a constant: a stub was compiled in, and
//! the UI said "not connected" because someone had written that it should. The
//! day a shield capsule exists, that constant is a second thing to remember to
//! change, and a wallet that has to be told what it can do will eventually be
//! told wrong.
//!
//! So it asks. `nonos.shield` is a service name like any other, and this is a
//! service lookup like any other: present and answering, or absent. The system
//! is capability-based, and a capability probe is the security model doing the
//! UI's job for it.
//!
//! Two properties matter and both are deliberate.
//!
//! Absence is the default. A lookup that fails, times out, or answers
//! unexpectedly leaves the screens disabled. There is no path through this
//! module that reports availability without a service having answered.
//!
//! Presence is not trust. A capsule answering this name was admitted by the
//! spawn gate like everything else, which is what makes the answer worth
//! anything; this module does not re-derive that, it relies on it.

use nonos_libc::mk_service_lookup;

/// The service a shield capsule registers.
///
/// Fixed here so the wallet, the shield capsule and anything that later wants
/// to know whether shielding is available all name it once.
pub const SHIELD_SERVICE: &[u8] = b"nonos.shield";

/// What the wallet knows about the shield capsule.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Shield {
    /// Not looked for yet this session.
    Unknown,
    /// Looked for and not found. The screens render their honest state.
    Absent,
    /// A capsule is registered under the service name and answering.
    Present,
}

impl Shield {
    pub fn available(self) -> bool {
        matches!(self, Shield::Present)
    }
}

/// Look for the shield capsule.
///
/// Cheap enough to call on a screen change and not on every paint: a service
/// lookup is an IPC round trip, and a wallet that probes sixty times a second
/// is a wallet that spends its time asking questions it already answered.
pub fn probe() -> Shield {
    let mut port: u32 = 0;
    let mut pid: u32 = 0;
    let rc = mk_service_lookup(
        SHIELD_SERVICE.as_ptr(),
        SHIELD_SERVICE.len(),
        &mut port as *mut u32,
        &mut pid as *mut u32,
    );
    // Anything other than a successful lookup naming a live pid is absence.
    // Failing closed here is the whole point: an unexpected return must never
    // read as availability.
    if rc >= 0 && pid != 0 {
        Shield::Present
    } else {
        Shield::Absent
    }
}
