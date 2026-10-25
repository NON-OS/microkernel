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
//! The receipt job (letter agent-02): which record a request names, and what
//! each frame does. Only the caller is answered; a drop comes only on an ack
//! for the made receipt's own seal number; a RECEIPT is answered again and
//! again; and the job is bound to RECEIPT by the argv init writes.

use agentproof_policy::boxed::hash::box_hash;
use agentproof_policy::boxed::preset::parse as box_parse;
use agentproof_receipt::seal_slot::slot0;
use nonos_linux_policy::{app_id, policy_hash, unlisted};
use nonos_proof_abi::wire::{ReceiptRequest, RECEIPT, RECEIPT_ACK};

use crate::job_bound::parse;
use crate::job_receipt_slot::slot_of;
use crate::job_receipt_step::{step, Step, EINVAL, ENOMEM, EPERM};

#[test]
fn the_record_named_is_this_app_in_this_box() {
    let policy = unlisted([7; 32]).encode();
    let req = ReceiptRequest { box_word: b"strict", policy: &policy };
    let app = app_id(&[7; 32], &policy_hash(&policy));
    let hash = box_hash(&box_parse(b"strict").unwrap());
    assert_eq!(slot_of(&req), slot0(&app, Some(&hash)));
    let none = unlisted([0; 32]).encode();
    let busybox = ReceiptRequest { box_word: b"strict", policy: &none };
    assert_eq!(slot_of(&busybox), slot0(&[0; 32], Some(&hash)), "BusyBox, which has no app_id");
    assert_eq!(slot_of(&ReceiptRequest { box_word: b"loose", policy: &policy }), None);
    let encoded = req.encode().unwrap();
    let back = ReceiptRequest::decode(&encoded).unwrap();
    assert_eq!((back.box_word, back.policy), (&b"strict"[..], &policy));
    assert!(ReceiptRequest::decode(&encoded[..encoded.len() - 1]).is_none());
}

#[test]
fn only_an_ack_for_the_made_receipt_drops_its_record() {
    let ack = |n: u64| n.to_le_bytes();
    assert_eq!(step(true, RECEIPT, b"", true, None), Step::Make);
    assert_eq!(step(true, RECEIPT, b"", true, Some(9)), Step::Make, "asked again, made again");
    assert_eq!(step(true, RECEIPT_ACK, &ack(9), true, Some(9)), Step::Drop);
    assert_eq!(step(true, RECEIPT_ACK, &ack(8), true, Some(9)), Step::Refuse(EINVAL), "wrong seal");
    assert_eq!(step(true, RECEIPT_ACK, &ack(9), true, None), Step::Refuse(EINVAL), "nothing made");
    assert_eq!(step(true, RECEIPT_ACK, &ack(9)[..7], true, Some(9)), Step::Refuse(EINVAL));
    assert_eq!(
        step(false, RECEIPT_ACK, &ack(9), true, Some(9)),
        Step::Refuse(EPERM),
        "not the caller"
    );
    assert_eq!(step(false, RECEIPT, b"", true, None), Step::Refuse(EPERM));
    assert_eq!(step(true, RECEIPT, b"", false, None), Step::Refuse(ENOMEM));
    assert_eq!(step(true, 1, b"", true, Some(9)), Step::Refuse(EINVAL), "no other op");
}

#[test]
fn init_binds_a_receipt_job_by_its_argv() {
    let a: Vec<Vec<u8>> =
        ["--nonos-prove-job", "receipt", "77"].iter().map(|s| s.as_bytes().to_vec()).collect();
    assert_eq!(parse(&a), Some((RECEIPT, 77)));
}
