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

//! Building a policy, refusing one that is unusable or unsafe.

use alloc::boxed::Box;
use alloc::vec::Vec;

use nonos_hash::Sha256;
use nsh_wire::keys::{ED_PUBLIC_LEN, ML_PUBLIC_LEN};

use super::{Authority, Policy, PolicyError, Release};
use super::{MAX_AUTHORITIES, MAX_RELEASES, MAX_REVOKED};

impl Authority {
    pub fn new(ed: [u8; ED_PUBLIC_LEN], ml: Box<[u8; ML_PUBLIC_LEN]>) -> Self {
        let mut h = Sha256::new();
        h.update(&ed);
        h.update(&ml[..]);
        Authority { ed, ml, id: h.finalize() }
    }

    pub fn id(&self) -> &[u8; 32] {
        &self.id
    }
}

impl Policy {
    pub fn new(
        fleet_id: [u8; 16],
        authorities: Vec<Authority>,
        releases: Vec<Release>,
        revoked: Vec<[u8; 64]>,
    ) -> Result<Self, PolicyError> {
        let zero = |v: &[u8; 32]| v.iter().all(|b| *b == 0);
        if fleet_id.iter().all(|b| *b == 0) {
            return Err(PolicyError::Fleet);
        }
        let twice = |i: usize| authorities[..i].iter().any(|a| a.id == authorities[i].id);
        let n = authorities.len();
        if n == 0 || n > MAX_AUTHORITIES || (0..n).any(twice) {
            return Err(PolicyError::Authorities);
        }
        if releases.is_empty() || releases.len() > MAX_RELEASES {
            return Err(PolicyError::Releases);
        }
        for (i, r) in releases.iter().enumerate() {
            if [&r.pcr8, &r.pcr9, &r.loader, &r.front].into_iter().any(zero) {
                return Err(PolicyError::ZeroValue);
            }
            if releases[..i].iter().any(|o| o.pcr9 == r.pcr9) {
                return Err(PolicyError::SharedPcr9);
            }
        }
        if revoked.len() > MAX_REVOKED {
            return Err(PolicyError::Revoked);
        }
        Ok(Policy { fleet_id, authorities, releases, revoked })
    }
}
