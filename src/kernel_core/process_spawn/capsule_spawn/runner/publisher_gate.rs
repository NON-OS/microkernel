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

use super::super::spec::{CapsuleSpecVerified, SpawnError};
use crate::security::capsule_attest::Proved;

pub(crate) fn publisher_gate(
    spec: &CapsuleSpecVerified<'_>,
    namespace: &str,
    attest_caps: u64,
) -> Result<Option<Proved>, SpawnError> {
    if !matches!(super::tier::classify(namespace), super::tier::Tier::Publisher) {
        return Err(SpawnError::AttestationRejected);
    }
    // Never a name the image's own capsules declare, running or not (L52),
    // nor one under a root or in a form only the system takes (L57).
    use crate::services::registry::{is_reserved_form, is_system_name};
    let system = |n: &str| is_system_name(n) || is_reserved_form(n);
    if system(spec.name) || system(spec.reply_inbox) {
        return Err(SpawnError::EndpointCollision);
    }
    /*
     * A publisher's capsule runs only with a v4 trailer that verifies, path and
     * STARK, under the vendor root or a root the person enrolled: the same gate,
     * which refuses a capsule that carries none.
     */
    super::attest_gate::attest_gate(spec, attest_caps)
}
