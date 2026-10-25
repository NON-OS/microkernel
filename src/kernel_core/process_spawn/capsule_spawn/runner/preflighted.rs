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

//! What the spawn gate established, kept until a pid exists to record it
//! against.

use crate::security::attest_registry::{record_attested, record_namespace};
use crate::security::capsule_attest::Proved;
use crate::security::dev_roots::Authority;

pub(crate) struct Preflighted {
    /// What the spawn gate proved and who proved it, recorded once a pid
    /// exists. `None` when nothing was proved.
    pub proved: Option<Proved>,
    pub install_caps: u64,
    /// The publisher-signed identity of manifest and image, checked by
    /// `verify_with_publisher`.
    pub capsule_id: [u8; 32],
    /// BLAKE3 of the verified manifest's namespace: whose bound machine keys
    /// the capsule may derive, if the vendor root proved it.
    pub namespace_hash: [u8; 32],
    /// Spawned from the system's own namespace (L61).
    pub system: bool,
}

impl Preflighted {
    /*
     * Written by install before the pid can run, so its first call already
     * finds it (L61). What goes in is the measurement the gate checked, never
     * one recomputed from the image. A capsule that ran on a publisher
     * signature alone is recorded under that authority rather than left out:
     * the registry claims to be complete, and a receipt that omits a running
     * program is the one lie a verifier cannot catch.
     */
    pub(super) fn record(&self, pid: u32) {
        let (measurement, authority) = match self.proved {
            Some(proved) => (proved.measurement, proved.authority),
            None => (self.capsule_id, Authority::Publisher),
        };
        record_attested(pid, measurement, super::profile_gate::caps(self.install_caps), authority);
        record_namespace(pid, self.namespace_hash);
        if self.system {
            crate::process::core::spawn_tier::mark_system(pid);
        }
    }
}
