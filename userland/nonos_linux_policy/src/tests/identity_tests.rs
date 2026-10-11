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

use super::sample::policy;
use crate::{app_id, policy_hash};

fn hex(b: &[u8; 32]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

#[test]
fn policy_hash_is_domain_separated() {
    let b = policy().encode();
    let mut cat = b"NONOS.linux.policy.v1".to_vec();
    cat.extend_from_slice(&b);
    assert_eq!(policy_hash(&b), *blake3::hash(&cat).as_bytes());
    assert_ne!(policy_hash(&b), *blake3::hash(&b).as_bytes());
}

#[test]
fn app_id_binds_program_and_policy() {
    let ph = policy_hash(&policy().encode());
    let id = app_id(&[0x11; 32], &ph);
    let mut cat = b"NONOS.linux.app.v1".to_vec();
    cat.extend_from_slice(&[0x11; 32]);
    cat.extend_from_slice(&ph);
    assert_eq!(id, *blake3::hash(&cat).as_bytes());
    assert_ne!(id, app_id(&[0x12; 32], &ph));
    let mut other = ph;
    other[31] ^= 1;
    assert_ne!(id, app_id(&[0x11; 32], &other));
}

/// The bytes a release signs and a proof is checked against: a change here
/// is a format change, and every signed policy stops verifying.
#[test]
fn pinned_vectors() {
    let ph = policy_hash(&policy().encode());
    assert_eq!(hex(&ph), "aeb8c5679d33b3ba33e89e1b06c6603b212587470c898320af88c163b590c851");
    assert_eq!(
        hex(&app_id(&[0x11; 32], &ph)),
        "372ece38921fb4fe5f30f1609cd036526930a6cdf406b555620a9bb8aa4d2cfe"
    );
}
