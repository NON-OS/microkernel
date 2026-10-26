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

//! The rule the trust declaration applies, pure, so baked_trust_proofs
//! mounts it alone.

/// What may stand in for the release's trust: a test set and a test policy key.
pub(super) const TEST_TRUST: [&str; 2] = ["NONOS_BAKED_TRUST_DIR", "NONOS_DEVICE_POLICY_PUBKEY"];

/// Why a build may not go on: a kernel the seal may seal for release
/// (nonos-release) embeds only the tree's trust, never a set or key named at
/// build time.
pub(super) fn refusal(release: bool, named: &[(&str, bool)]) -> Option<String> {
    let set: Vec<&str> = named.iter().filter(|(_, on)| *on).map(|(v, _)| *v).collect();
    (release && !set.is_empty()).then(|| {
        format!("a nonos-release kernel embeds only the tree's trust; unset {}", set.join(", "))
    })
}
