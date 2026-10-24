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

//! A revocation list: one that parses names one fleet and at most 4096
//! grant ids, strictly ascending and none zero. Its signed bytes are
//! everything before the signature.

#![no_main]

use libfuzzer_sys::fuzz_target;
use nsh_roles::{RevocationList, MAX_REVOKED};
use nsh_wire::keys::HYBRID_SIG_LEN;

fuzz_target!(|data: &[u8]| {
    let Ok(l) = RevocationList::parse(data) else { return };
    assert!(l.fleet_id.iter().any(|b| *b != 0));
    assert!(l.len() <= MAX_REVOKED && l.is_empty() == (l.len() == 0));
    let ids: Vec<&[u8]> = l.ids().collect();
    assert_eq!(ids.len(), l.len());
    assert!(ids.iter().all(|id| id.len() == 16 && id.iter().any(|b| *b != 0)));
    assert!(ids.windows(2).all(|w| w[0] < w[1]));
    assert!(l.signed.ends_with(l.signer));
    assert_eq!(data.len(), l.signed.len() + HYBRID_SIG_LEN);
});
