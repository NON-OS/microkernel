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

//! A footerless record of another session, signed at a checkpoint, is
//! held to what it must name just as a complete one is.

use super::record_good::report;
use crate::kit::attest::TestAttester;
use crate::kit::enrol::EnrolSpec;
use crate::kit::keys;
use crate::kit::pair::policy;
use crate::kit::record::{header_with, record_doc, Rec, SESSION};
use nsh_client::{Bound, Mismatch, RecordRefusal, Verdict};

#[test]
fn a_cut_short_record_of_another_session_is_refused_when_bound() {
    let th = [0x7A; 64];
    let other = [0x5F; 16];
    let header = header_with(
        other,
        keys::user().digest(),
        &th,
        &EnrolSpec::good().bytes(),
        &record_doc(&th),
    );
    let rec = Rec::for_handshake(th, header, 10);
    let ckpt = rec.ckpt_with(other, 5, rec.chain_after(5), keys::host());
    let a = TestAttester::default();
    let check = |b: Bound| report(&rec.no_footer(), std::slice::from_ref(&ckpt), &b, &policy(), &a);
    let free = check(Bound::default()).map(|r| (r.verdict, r.session_id));
    assert_eq!(free, Ok((Verdict::CutShort { seq: 5, boxed: false }, other)));
    let bound = Bound { session_id: Some(SESSION), ..Bound::default() };
    assert_eq!(check(bound), Err(RecordRefusal::Unbound(Mismatch::SessionId)));
}
