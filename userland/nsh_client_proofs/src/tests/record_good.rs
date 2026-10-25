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

//! The record checker on good records: complete, with checkpoints, with a
//! provisional `BOX` entry, and the `BOX` body's own rules.

use crate::kit::attest::TestAttester;
use crate::kit::pair::policy;
use crate::kit::record::{record_doc, Rec, START};
use nsh_client::{verify_record, Bound, Policy, RecordRefusal, Report, Verdict};

pub fn check(record: &[u8], ckpts: &[Vec<u8>]) -> Result<Verdict, RecordRefusal> {
    check_with(record, ckpts, &TestAttester::default())
}

pub fn check_with(
    record: &[u8],
    ckpts: &[Vec<u8>],
    attester: &TestAttester,
) -> Result<Verdict, RecordRefusal> {
    report(record, ckpts, &Bound::default(), &policy(), attester).map(|r| r.verdict)
}

/// The whole report, with what the header must name and the policy.
pub fn report(
    record: &[u8],
    ckpts: &[Vec<u8>],
    expect: &Bound,
    policy: &Policy,
    attester: &TestAttester,
) -> Result<Report, RecordRefusal> {
    let ckpts: Vec<&[u8]> = ckpts.iter().map(Vec::as_slice).collect();
    verify_record(record, &record_doc(&[0x7A; 64]), &ckpts, expect, policy, attester)
}

fn complete(entries: u32, end_ms: u64, boxed: bool) -> Result<Verdict, RecordRefusal> {
    Ok(Verdict::Complete { entries, end_ms, end_reason: 1, boxed })
}

#[test]
fn a_good_record_is_complete() {
    let rec = Rec::good(130);
    assert_eq!(check(&rec.bytes(), &[]), complete(131, START + 1305, false));
    let ckpts = [63, 127, 130, 0].map(|s| rec.ckpt(s));
    assert_eq!(check(&rec.bytes(), &ckpts), complete(131, START + 1305, false));
}
