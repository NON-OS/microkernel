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

//! The reason codes: their numbers, and which frame may carry which.

use nsh_wire::{abort, auth_refused, plain_abort, Reason, WireError};

#[test]
fn the_reason_codes_are_the_specified_ones() {
    use Reason::*;
    let table = [
        (Ok, 0),
        (Protocol, 1),
        (Version, 2),
        (Policy, 3),
        (Limit, 4),
        (Busy, 5),
        (GrantSig, 16),
        (GrantTime, 17),
        (GrantScope, 18),
        (GrantRevoked, 19),
        (RevocationStale, 20),
        (RoleUnknown, 21),
        (CapsCeiling, 22),
        (CapUnknown, 23),
        (UserSig, 24),
        (EscParent, 32),
        (EscApproval, 33),
        (EscTooLong, 34),
        (EscActive, 35),
    ];
    for (reason, code) in table {
        assert_eq!((reason.code(), Reason::from_code(code)), (code, Result::Ok(reason)));
    }
}

#[test]
fn each_frame_takes_only_its_own_reasons() {
    let (mut known, mut sets) = (0, [0; 3]);
    for code in 0..=u16::MAX {
        let b = code.to_le_bytes();
        let Ok(r) = Reason::from_code(code) else {
            assert_eq!(
                (abort(&b), auth_refused(&b)),
                (Err(WireError::Value), Err(WireError::Value))
            );
            continue;
        };
        known += 1;
        let got = [plain_abort(&b).is_ok(), abort(&b).is_ok(), auth_refused(&b).is_ok()];
        let want = [matches!(code, 2 | 5), (1..=5).contains(&code), (16..=24).contains(&code)];
        assert_eq!(got, want, "{r:?}");
        sets.iter_mut().zip(got).for_each(|(n, g)| *n += usize::from(g));
    }
    assert_eq!((known, sets), (19, [2, 5, 9]));
}
