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

//! The market wire as its clients speak it: which replies are believed,
//! and that each body reads back what the market wrote.

use alloc::vec::Vec;

use nonos_market_proto::{
    listing_body, pair_body, parse_app, parse_list, parse_readiness, parse_release, reply_body,
    request, status_name, ReplyError, HDR_LEN, MAGIC, OP_GET_APP, OP_LIST_APPS, VERSION,
};

use crate::readiness_tests::release;
use crate::release_wire::encode;

/// A reply as the market capsule frames one: its header echoing the
/// request, then the status, then the body.
fn reply(op: u16, id: u32, status: i32, body: &[u8]) -> Vec<u8> {
    let mut rx = Vec::new();
    rx.extend_from_slice(&MAGIC.to_le_bytes());
    rx.extend_from_slice(&VERSION.to_le_bytes());
    rx.extend_from_slice(&op.to_le_bytes());
    rx.extend_from_slice(&0u16.to_le_bytes());
    rx.extend_from_slice(&0u16.to_le_bytes());
    rx.extend_from_slice(&id.to_le_bytes());
    rx.extend_from_slice(&(4 + body.len() as u32).to_le_bytes());
    rx.extend_from_slice(&status.to_le_bytes());
    rx.extend_from_slice(body);
    rx
}

fn lp(out: &mut Vec<u8>, s: &[u8]) {
    out.extend_from_slice(&(s.len() as u32).to_le_bytes());
    out.extend_from_slice(s);
}

#[test]
fn a_request_carries_the_markets_magic_and_its_own_id() {
    let tx = request(OP_LIST_APPS, 7, b"xy");
    assert_eq!(tx.len(), HDR_LEN + 2);
    assert_eq!(&tx[0..4], b"TKMN", "NMKT as a little-endian word");
    assert_eq!(u16::from_le_bytes([tx[6], tx[7]]), OP_LIST_APPS);
    assert_eq!(u32::from_le_bytes(tx[12..16].try_into().unwrap()), 7);
    assert_eq!(u32::from_le_bytes(tx[16..20].try_into().unwrap()), 2);
}

#[test]
fn the_answer_to_this_call_is_believed() {
    let rx = reply(OP_GET_APP, 9, 0, b"body");
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Ok(&b"body"[..]));
}

#[test]
fn an_answer_to_an_earlier_call_is_stale() {
    let rx = reply(OP_GET_APP, 8, 0, b"body");
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Err(ReplyError::Stale));
    let rx = reply(OP_LIST_APPS, 9, 0, b"body");
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Err(ReplyError::Stale));
}

#[test]
fn another_protocols_reply_is_foreign() {
    let mut rx = reply(OP_GET_APP, 9, 0, b"body");
    rx[0..4].copy_from_slice(&0x4E43_4D50u32.to_le_bytes());
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Err(ReplyError::Foreign));
    let mut rx = reply(OP_GET_APP, 9, 0, b"body");
    rx[4] = 2;
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Err(ReplyError::Foreign));
}

#[test]
fn a_reply_shorter_than_its_header_says_is_truncated() {
    let rx = reply(OP_GET_APP, 9, 0, b"body");
    assert_eq!(reply_body(&rx[..rx.len() - 1], OP_GET_APP, 9), Err(ReplyError::Truncated));
    let mut rx = reply(OP_GET_APP, 9, 0, b"");
    rx[16..20].copy_from_slice(&u32::MAX.to_le_bytes());
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Err(ReplyError::Truncated));
    let mut rx = reply(OP_GET_APP, 9, 0, b"");
    rx[16..20].copy_from_slice(&3u32.to_le_bytes());
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Err(ReplyError::Truncated));
}

#[test]
fn a_reply_without_a_status_is_short() {
    let rx = reply(OP_GET_APP, 9, 0, b"");
    for n in 0..rx.len() {
        assert_eq!(reply_body(&rx[..n], OP_GET_APP, 9), Err(ReplyError::Short), "{n} bytes");
    }
}

#[test]
fn a_refusal_is_its_status_and_never_a_body() {
    let rx = reply(OP_LIST_APPS, 3, -61, b"");
    assert_eq!(reply_body(&rx, OP_LIST_APPS, 3), Err(ReplyError::Status(-61)));
    assert_eq!(status_name(-61), Some("ENODATA"));
    assert_eq!(status_name(-1), None);
}

#[test]
fn trailing_bytes_past_the_payload_are_not_the_body() {
    let mut rx = reply(OP_GET_APP, 9, 0, b"body");
    rx.extend_from_slice(b"junk");
    assert_eq!(reply_body(&rx, OP_GET_APP, 9), Ok(&b"body"[..]));
}

fn catalogue() -> Vec<u8> {
    let mut body = 2u32.to_le_bytes().to_vec();
    lp(&mut body, b"linux.htop");
    body.extend_from_slice(&[7; 32]);
    lp(&mut body, b"htop");
    body.push(1);
    lp(&mut body, b"nonos.app.notes");
    body.extend_from_slice(&[8; 32]);
    lp(&mut body, b"Notes");
    body.push(0);
    body
}

#[test]
fn the_catalogue_reads_back_every_listing() {
    let list = parse_list(&catalogue()).unwrap();
    assert_eq!(list.len(), 2);
    assert_eq!(list[0].id, b"linux.htop");
    assert_eq!(list[0].measurement, [7; 32]);
    assert_eq!(list[0].name, b"htop");
    assert!(list[0].ready);
    assert_eq!(list[1].name, b"Notes");
    assert!(!list[1].ready);
}

#[test]
fn a_catalogue_cut_anywhere_is_refused_not_misread() {
    let body = catalogue();
    for n in 0..body.len() {
        assert!(parse_list(&body[..n]).is_none(), "cut at {n}");
    }
}

#[test]
fn a_catalogue_naming_more_listings_than_it_holds_is_refused() {
    let mut body = catalogue();
    body[0..4].copy_from_slice(&u32::MAX.to_le_bytes());
    assert!(parse_list(&body).is_none());
}

#[test]
fn a_listing_in_full_reads_back_its_publisher_and_description() {
    let mut body = Vec::new();
    lp(&mut body, b"linux.htop");
    body.extend_from_slice(&[7; 32]);
    lp(&mut body, b"htop");
    lp(&mut body, b"Alpine");
    body.extend_from_slice(&[9; 32]);
    lp(&mut body, b"An interactive process viewer");
    body.extend_from_slice(&2u32.to_le_bytes());
    let app = parse_app(&body).unwrap();
    assert_eq!(app.name, b"htop");
    assert_eq!(app.publisher, b"Alpine");
    assert_eq!(app.description, b"An interactive process viewer");
    assert_eq!(app.releases, 2);
    for n in 0..body.len() {
        assert!(parse_app(&body[..n]).is_none(), "cut at {n}");
    }
}

#[test]
fn a_release_reads_back_what_the_market_encodes() {
    let mut rel = release("x86_64-linux", 0);
    rel.release_id = "linux.htop@3.3.0".into();
    rel.validation.note = "hashed 123 bytes".into();
    rel.required_capabilities = alloc::vec!["net".into(), "fs".into()];
    let body = encode(&rel);
    let got = parse_release(&body).unwrap();
    assert_eq!(got.version, b"linux.htop@3.3.0");
    assert_eq!(got.short_version(), b"3.3.0");
    assert_eq!(got.note, b"hashed 123 bytes");
    // The validator id ("v", five bytes framed) and the eight-byte time
    // follow the note and are not read: a cut anywhere before them fails.
    let unread = 5 + 8;
    for n in 0..body.len() - unread {
        assert!(parse_release(&body[..n]).is_none(), "cut at {n}");
    }
}

#[test]
fn readiness_is_the_verdict_then_one_byte_a_gate() {
    let r = parse_readiness(&[1, 1, 0, 1, 1, 1, 0]).unwrap();
    assert!(r.install_ready);
    assert_eq!(r.gates, [true, false, true, true, true, false]);
    assert!(parse_readiness(&[1, 1, 1, 1, 1, 1]).is_none());
}

#[test]
fn the_install_stage_reads_as_the_kernel_writes_it() {
    use nonos_market_proto::Stage;
    assert_eq!(Stage::of(0), Stage::Idle);
    assert_eq!(Stage::of(1), Stage::Queued);
    assert_eq!(Stage::of(2), Stage::Installing);
    assert_eq!(Stage::of(3), Stage::Installed);
    assert_eq!(Stage::of(4), Stage::Refused);
    assert_eq!(Stage::of(16 + 5), Stage::Failed(5));
    assert_eq!(Stage::of(16 + 300), Stage::Failed(255));
    // An errno for the question is no stage.
    assert_eq!(Stage::of(-22), Stage::Idle);
    assert!(Stage::Queued.pending() && Stage::Installing.pending());
    assert!(!Stage::Installed.pending() && !Stage::Failed(2).pending());
}

#[test]
fn a_description_is_cut_at_spaces_to_the_width() {
    use crate::terminal::wrap;
    let text = b"an interactive process viewer for the terminal";
    let lines = wrap(text, 20);
    assert!(lines.iter().all(|l| l.len() <= 20));
    assert_eq!(lines.join(&b' '), text.to_vec());
    // One word longer than the width is cut where it must be.
    let long = [b'x'; 45];
    assert_eq!(wrap(&long, 20).iter().map(|l| l.len()).collect::<Vec<_>>(), [20, 20, 5]);
    assert!(wrap(b"", 20).is_empty());
}

#[test]
fn request_bodies_are_length_prefixed_ids() {
    assert_eq!(listing_body(b"ab"), [2, 0, 0, 0, b'a', b'b']);
    assert_eq!(pair_body(b"ab", b""), [2, 0, 0, 0, b'a', b'b', 0, 0, 0, 0]);
}
