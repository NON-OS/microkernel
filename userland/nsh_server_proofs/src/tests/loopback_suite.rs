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

//! The hybrid suite as the wire shows it: the hellos' suite and sizes, and
//! each half of the record's signature.

use super::loopback_wire::Got;
use crate::kit::attest_build::keys::{hybrid, HOST};
use crate::kit::user::{attester, policy};
use nonos_hash::sha512;
use nsh_client::{verify_record, Bound, Event as C, RecordRefusal};
use nsh_crypto::{labelled, Label};
use nsh_wire::keys::HYBRID_SIG_LEN;

/// The hellos as the wire carries them: `NSH1`, version 2, suite 1, flags
/// 0, a 32-byte random, then the X-Wing key or ciphertext.
pub fn hellos(c2s: &[u8], s2c: &[u8]) {
    let ch = &c2s[..4 + 1 + 10 + 32 + 1216];
    assert_eq!(u32::from_le_bytes(ch[..4].try_into().expect("len")), 1 + 10 + 32 + 1216);
    assert_eq!(&ch[4..15], &[0x01, b'N', b'S', b'H', b'1', 2, 0, 1, 0, 0, 0]);
    let sh = &s2c[..4 + 1 + 10 + 32 + 1120];
    assert_eq!(u32::from_le_bytes(sh[..4].try_into().expect("len")), 1 + 10 + 32 + 1120);
    assert_eq!(&sh[4..15], &[0x02, b'N', b'S', b'H', b'1', 2, 0, 1, 0, 0, 0]);
    assert_eq!(nsh_crypto::EK_LEN, 1184 + 32, "ML-KEM-768 then X25519");
    assert_eq!(nsh_crypto::CT_LEN, 1088 + 32, "ML-KEM-768 then X25519");
}

/// Each half of the record's signature verifies alone under the host keys,
/// and spoiling either one makes the record fail its check.
pub fn record_halves(got: &Got) {
    let rec = got.events.iter().find_map(|e| match e {
        C::Record(r) => Some(r.clone()),
        _ => None,
    });
    let rec = rec.expect("a record");
    let (signed, sig) = rec.split_at(rec.len() - HYBRID_SIG_LEN);
    let digest = labelled(Label::RecordSig, &sha512(signed));
    let host = hybrid(HOST);
    let ed = nonos_ed25519::Signature::from_bytes(sig[..64].try_into().expect("ed"));
    assert!(nonos_ed25519::verify(&host.ed.public, &digest, &ed), "Ed25519 half");
    assert!(
        nonos_mldsa65::verify(&host.ml_public, &digest, sig[64..].try_into().expect("ml")),
        "ML-DSA-65 half"
    );
    let th = got.th.expect("th");
    let doc = super::good::doc_for(&th);
    for at in [rec.len() - HYBRID_SIG_LEN + 3, rec.len() - 7] {
        let mut bad = rec.clone();
        bad[at] ^= 1;
        let r = verify_record(&bad, &doc, &[], &Bound::default(), &policy(), &attester());
        assert!(matches!(r, Err(RecordRefusal::Sig(_))), "{r:?}");
    }
}
