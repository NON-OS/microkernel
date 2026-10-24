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

//! The server's session replies: an escalation's result and a record part.

use nsh_wire::{EscalateResult, Reason, RecordPart, WireError, MAX_DATA};

#[test]
fn a_granted_escalation_names_its_end_and_a_refused_one_names_none() {
    let b = |code: u16, ms: u64| [code.to_le_bytes().to_vec(), ms.to_le_bytes().to_vec()].concat();
    let ok = EscalateResult { reason: Reason::Ok, expires_ms: 5 };
    assert_eq!(EscalateResult::parse(&b(0, 5)), Ok(ok));
    assert_eq!(EscalateResult::parse(&b(0, 0)), Err(WireError::Value));
    assert_eq!(EscalateResult::parse(&b(33, 5)), Err(WireError::Value));
    assert_eq!(EscalateResult::parse(&b(33, 0)).map(|r| r.reason), Ok(Reason::EscApproval));
    assert_eq!(EscalateResult::parse(&b(999, 0)), Err(WireError::Value));
    for code in [1, 2, 3, 4, 5] {
        assert_eq!(EscalateResult::parse(&b(code, 0)), Err(WireError::Value), "{code}");
    }
    assert!(EscalateResult::parse(&b(17, 0)).is_ok());
    let mut out = [0u8; 10];
    let bad = EscalateResult { reason: Reason::EscActive, expires_ms: 1 };
    assert_eq!(bad.encode(&mut out), Err(WireError::Value));
    assert_eq!(ok.encode(&mut out).map(|n| out[..n].to_vec()), Ok(b(0, 5)));
}

#[test]
fn a_record_part_is_flagged_and_never_empty() {
    assert_eq!(RecordPart::parse(&[1, 9]), Ok(RecordPart { last: true, bytes: &[9] }));
    assert_eq!(RecordPart::parse(&[0, 9]).map(|p| p.last), Ok(false));
    assert_eq!(RecordPart::parse(&[2, 9]), Err(WireError::Value));
    assert_eq!(RecordPart::parse(&[1]), Err(WireError::Value));
    assert_eq!(RecordPart::parse(&[]), Err(WireError::Short));
    assert_eq!(RecordPart::parse(&vec![1; MAX_DATA + 1]), Err(WireError::Oversize));
    assert!(RecordPart::parse(&vec![1; MAX_DATA]).is_ok());
}

#[test]
fn an_escalation_is_refused_only_for_a_grant_or_escalation_reason() {
    for code in 1..=u16::MAX {
        let body = [&code.to_le_bytes()[..], &0u64.to_le_bytes()].concat();
        let want = matches!(code, 16..=24 | 32..=35);
        assert_eq!(nsh_wire::EscalateResult::parse(&body).is_ok(), want, "{code}");
    }
}
