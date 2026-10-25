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

//! A role table: one that parses names one fleet, a revocation age and
//! session lengths in range, strictly ascending and disjoint key lists, and
//! strictly ascending roles whose ceilings hold no forbidden capability.
//! Its signed bytes are everything before the signature, and checking the
//! signature never panics.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nsh_roles::caps::forbidden_mask;
use nsh_roles::{Unverified, MAX_APPROVERS, MAX_AUTHORITIES, MAX_ROLES, MAX_SPAN_MS};
use nsh_roles::MAX_REVOCATION_AGE_MS;
use nsh_wire::keys::HYBRID_SIG_LEN;

fuzz_target!(|data: &[u8]| {
    let Ok(u) = Unverified::parse(data) else { return };
    let t = &u.table;
    assert!(t.fleet_id.iter().any(|b| *b != 0));
    assert!((1..=MAX_REVOCATION_AGE_MS).contains(&t.revocation_max_age_ms));
    assert!((1..=MAX_AUTHORITIES).contains(&t.authorities.len()));
    assert!(t.approvers.len() <= MAX_APPROVERS);
    for list in [&t.authorities, &t.approvers] {
        assert!(list.windows(2).all(|w| w[0].id < w[1].id));
    }
    assert!(t.authorities.iter().all(|a| t.approver(&a.id).is_none()));
    assert!((1..=MAX_ROLES).contains(&t.roles.len()));
    assert!(t.roles.windows(2).all(|w| w[0].name < w[1].name));
    for r in &t.roles {
        assert!(nsh_wire::names::role(r.name).is_ok());
        assert!(r.approvals <= 2 && (1..=MAX_SPAN_MS).contains(&r.max_session_ms));
        assert!(!r.ceiling.is_empty());
        assert_eq!((r.ceiling.bits() | r.escalation.bits()) & forbidden_mask(), 0);
    }
    assert!(u.signed.ends_with(u.signer));
    assert_eq!(data.len(), u.signed.len() + HYBRID_SIG_LEN);
    let signer = *u.signer;
    let _ = u.verify(&signer, 0);
});
