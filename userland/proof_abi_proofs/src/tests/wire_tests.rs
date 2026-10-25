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

//! nonos.prove's wire: market_proto's header under "NPRV", replies a caller
//! believes only for its own op and id, and bodies of exact length.

use nonos_proof_abi::wire::{
    job_of, reply, reply_body, request, status_body, Header, ProveRequest, ReplyError, Stage, GET,
    HDR_LEN, PROVE, PROVE_BODY, PROVE_MAGIC, STALE_REGISTRY,
};

#[test]
fn the_header_is_market_protos_layout_under_nprv() {
    let tx = request(PROVE_MAGIC, PROVE, 7, &[9; 3]);
    assert_eq!(&tx[..4], b"NPRV");
    assert_eq!((&tx[4..6], &tx[6..8], &tx[10..12]), (&[1, 0][..], &[1, 0][..], &[0, 0][..]));
    assert_eq!((&tx[12..16], &tx[16..20], tx.len()), (&[7, 0, 0, 0][..], &[3, 0, 0, 0][..], 23));
    let (h, body) = Header::parse(&tx, PROVE_MAGIC).expect("parses");
    assert_eq!((h.op, h.id, body), (PROVE, 7, &[9u8; 3][..]));
}

#[test]
fn a_frame_not_exactly_whole_or_foreign_is_refused() {
    let tx = request(PROVE_MAGIC, PROVE, 7, &[9; 3]);
    assert!(Header::parse(&[&tx[..], &[0]].concat(), PROVE_MAGIC).is_none(), "trailing");
    assert!(Header::parse(&tx[..tx.len() - 1], PROVE_MAGIC).is_none(), "short");
    assert!(Header::parse(&tx[..HDR_LEN - 1], PROVE_MAGIC).is_none(), "no header");
    assert!(Header::parse(&tx, 0x4E4D_4B54).is_none(), "market's magic");
    for at in [4, 10] {
        let mut bad = tx.clone();
        bad[at] ^= 1;
        assert!(Header::parse(&bad, PROVE_MAGIC).is_none(), "byte {at}");
    }
}

#[test]
fn a_reply_is_believed_only_for_its_own_op_and_id() {
    let (req, _) = Header::parse(&request(PROVE_MAGIC, GET, 5, &[]), PROVE_MAGIC).unwrap();
    let ok = reply(PROVE_MAGIC, &req, 0, b"abc");
    assert_eq!(reply_body(&ok, PROVE_MAGIC, GET, 5), Ok(&b"abc"[..]));
    assert_eq!(reply_body(&ok, PROVE_MAGIC, GET, 6), Err(ReplyError::Stale));
    assert_eq!(reply_body(&ok, PROVE_MAGIC, PROVE, 5), Err(ReplyError::Stale));
    let no = reply(PROVE_MAGIC, &req, STALE_REGISTRY, &[]);
    assert_eq!(reply_body(&no, PROVE_MAGIC, GET, 5), Err(ReplyError::Status(STALE_REGISTRY)));
    assert_eq!(reply_body(&ok[..ok.len() - 1], PROVE_MAGIC, GET, 5), Err(ReplyError::Malformed));
}

#[test]
fn bodies_have_one_length() {
    let (id, policy) = ([0x4C; 32], [0x5A; 256]);
    let b = ProveRequest { listing_id: &id, policy: &policy }.encode();
    let back = ProveRequest::decode(&b).expect("decodes");
    assert_eq!((back.listing_id, back.policy), (&id, &policy));
    assert!(ProveRequest::decode(&b[..PROVE_BODY - 1]).is_none());
    assert!(ProveRequest::decode(&[&b[..], &[0]].concat()).is_none());
    assert_eq!(job_of(&[3, 0, 0, 0]), Some((3, None)));
    assert_eq!(job_of(&[3, 0, 0, 0, 9, 0, 0, 0]), Some((3, Some(9))));
    assert_eq!((job_of(&[3, 0, 0]), job_of(&[0; 12])), (None, None));
    assert_eq!(status_body(Stage::Done, 70_000), [4, 0x70, 0x11, 1, 0]);
    assert_eq!((Stage::from_u8(5), Stage::from_u8(6)), (Some(Stage::Failed), None));
}
