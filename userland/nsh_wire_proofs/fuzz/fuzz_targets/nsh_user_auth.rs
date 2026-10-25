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

//! `USER_AUTH` and `ESCALATE`: what parses signs all but its signature, and
//! the grant inside obeys every rule a grant must.

#![no_main]

mod rules;

use libfuzzer_sys::fuzz_target;
use nsh_wire::keys::HYBRID_SIG_LEN;
use nsh_wire::{escalate, GrantKind, UserAuth};

fuzz_target!(|data: &[u8]| {
    if let Ok(u) = UserAuth::parse(data) {
        assert_eq!(u.signed, &data[..data.len() - HYBRID_SIG_LEN]);
        assert_eq!(u.grant.kind, GrantKind::Base);
        rules::hold(&u.grant);
    }
    if let Ok(g) = escalate(data) {
        assert_eq!(g.kind, GrantKind::Escalation);
        rules::hold(&g);
    }
});
