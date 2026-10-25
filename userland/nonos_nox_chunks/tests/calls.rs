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

#[path = "common/terms.rs"]
mod terms;
#[path = "common/word.rs"]
mod word;

use nonos_nox_chunks::{dispute_call, fund, fund_from, refund, release, Dispute};
use nonos_settle_proto::kind_of;
use terms::terms;
use word::word;

#[test]
fn settle_takes_each_call_as_its_kind() {
    let t = terms();
    let d = Dispute {
        index: 1,
        cipher_chunk: vec![0xCC; 1024],
        plain_hash: [0xDD; 32],
        cipher_proof: vec![[1; 32]; 2],
        leaf_proof: vec![[2; 32]; 3],
    };
    let refunded = refund(&t, &[8; 20], &[9; 32]);
    let calls = [
        (9, fund(&t)),
        (9, fund_from(&t, 5)),
        (10, dispute_call(&t, &d).unwrap()),
        (11, refunded),
        (12, release(&t)),
    ];
    for (kind, cd) in calls {
        assert_eq!(kind_of(kind, &cd).map(|k| k.id), Some(kind), "kind {kind}");
    }
}

#[test]
fn refund_names_its_committed_destination() {
    let t = terms();
    let cd = refund(&t, &[8; 20], &[9; 32]);
    assert_eq!(cd.len(), 4 + 32 * 16);
    assert_eq!(&cd[4..4 + 32 * 14], t.words().concat());
    assert_eq!(&cd[4 + 32 * 14..], [word(&[8; 20]), [9; 32]].concat());
    assert_eq!(&fund_from(&t, 5)[4 + 32 * 14..], word(&[5]));
    assert_eq!(fund(&t).len(), 4 + 32 * 14);
}
