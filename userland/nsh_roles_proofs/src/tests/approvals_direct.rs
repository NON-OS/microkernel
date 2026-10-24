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

//! The approval rules hold on any list, however it was read. The grant
//! reader already refuses a repeated, out-of-order or issuer approval, so
//! these lists are handed to `approvals` directly.

use crate::build::keys::{key, K};
use crate::build::table::TableB;
use nsh_crypto::{labelled, Label};
use nsh_roles::{approvals, open_policy, Refusal};
use nsh_wire::keys::HybridSig;
use nsh_wire::Approval;

pub struct Signed {
    id: [u8; 32],
    sig: Vec<u8>,
}

pub fn signed(k: K, digest: &[u8; 64]) -> Signed {
    Signed { id: key(k).id(), sig: key(k).sign(digest) }
}

pub fn view(s: &Signed) -> Approval<'_> {
    let ed = s.sig[..64].try_into().expect("ed");
    let ml = s.sig[64..].try_into().expect("ml");
    Approval { approver: &s.id, sig: HybridSig { ed, ml } }
}

pub fn run(list: &[&Signed], digest: &[u8; 64]) -> Result<(), Refusal> {
    let t = TableB::default();
    let bytes = t.bytes();
    let policy = open_policy(&bytes, &t.anchor, 0).expect("table");
    let list: Vec<_> = list.iter().map(|s| view(s)).collect();
    let need = u8::try_from(list.len()).expect("count");
    let (subject, issuer) = (key(K::User).subject(), key(K::AuthA).id());
    approvals(&policy, &subject, &issuer, digest, &list, need)
}

pub fn digest() -> [u8; 64] {
    labelled(Label::Approve, &[0x5D; 64])
}

/// Two approvers' approvals, in ascending id order.
pub fn pair() -> (Signed, Signed) {
    let (a, b) = (signed(K::Appr1, &digest()), signed(K::Appr2, &digest()));
    if a.id < b.id {
        (a, b)
    } else {
        (b, a)
    }
}

#[test]
fn two_distinct_approvals_in_order_pass() {
    let (lo, hi) = pair();
    assert_eq!(run(&[&lo, &hi], &digest()), Ok(()));
}
