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

//! A record whose header or footer is not the host's: the document,
//! the challenge, the enrolment and the footer's signature.

use super::record_good::{check, check_with};
use crate::kit::attest::TestAttester;
use crate::kit::enrol::{EnrolSpec, SigMode};
use crate::kit::keys;
use crate::kit::record::{header, record_doc, Rec};
use nsh_client::{AttestRefusal, EnrolRefusal, Half, HostRefusal, RecordRefusal};

fn with_header(enrol: EnrolSpec, th: [u8; 64], doc: &[u8]) -> Vec<u8> {
    Rec::for_handshake([0x7A; 64], header(&th, &enrol.bytes(), doc), 5).bytes()
}

#[test]
fn the_header_must_name_this_document_challenge_and_enrolment() {
    let th = [0x7A; 64];
    let good_doc = record_doc(&th);
    let other = with_header(EnrolSpec::good(), th, b"another document");
    assert_eq!(check(&other, &[]), Err(RecordRefusal::DocHash));
    let moved = with_header(EnrolSpec::good(), [0x7B; 64], &good_doc);
    let host = |h| Err(RecordRefusal::Host(h));
    assert_eq!(check(&moved, &[]), host(HostRefusal::Attest(AttestRefusal::Challenge)));
    let lax = TestAttester { lax: true, ..TestAttester::default() };
    assert_eq!(check_with(&moved, &[], &lax), host(HostRefusal::Challenge));
    let enrol = |edit: fn(&mut EnrolSpec)| {
        let mut e = EnrolSpec::good();
        edit(&mut e);
        check(&with_header(e, th, &good_doc), &[])
    };
    let refused = |e| host(HostRefusal::Enrol(e));
    assert_eq!(enrol(|e| e.authority_id = keys::stranger().id()), refused(EnrolRefusal::Authority));
    assert_eq!(enrol(|e| e.sig = SigMode::BadMl), refused(EnrolRefusal::Sig(Half::MlDsa65)));
    assert_eq!(enrol(|e| e.sig = SigMode::BadEd), refused(EnrolRefusal::Sig(Half::Ed25519)));
    assert_eq!(enrol(|e| e.fleet = [9; 16]), refused(EnrolRefusal::Fleet));
    assert_eq!(enrol(|e| e.ak = [1; 64]), refused(EnrolRefusal::Ak));
    assert_eq!(enrol(|e| e.host_keys = [2; 64]), refused(EnrolRefusal::HostKeys));
    assert_eq!(enrol(|e| e.pcrs[1] = [3; 32]), host(HostRefusal::FirmwarePcrs));
}

#[test]
fn the_footer_must_be_signed_by_the_host_keys_both_halves() {
    let rec = Rec::good(10);
    assert_eq!(
        check(&rec.signed_by(keys::stranger()), &[]),
        Err(RecordRefusal::Sig(Half::Ed25519))
    );
    let mut b = rec.bytes();
    let n = b.len();
    b[n - 100] ^= 1;
    assert_eq!(check(&b, &[]), Err(RecordRefusal::Sig(Half::MlDsa65)));
    b[n - 100] ^= 1;
    b[n - 3373] ^= 1;
    assert_eq!(check(&b, &[]), Err(RecordRefusal::Sig(Half::Ed25519)));
}
