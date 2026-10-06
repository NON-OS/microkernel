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

//! The account refresh on each network: the snapshot request is carried
//! through a direct socket's sends whole and in order, and a batch reply
//! of the shape the RPC hosts answer parses into every field.

use crate::wallet::chain;
use crate::wallet::net::read_snapshot::{parse_snapshot, snapshot_request};
use crate::wallet::net::send_cap::{front, FRAME, OVERHEAD, SEND_MAX};
use crate::wallet::rpc::http_post;

/* The network picked is one global, as in the capsule; a test holds it. */
static PICKED: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub fn hold() -> std::sync::MutexGuard<'static, ()> {
    PICKED.lock().unwrap_or_else(|e| e.into_inner())
}

const ADDR: [u8; 20] = [0xd8; 20];
/* A TLS 1.3 record around the request: header 5, inner type 1, tag 16,
 * and the 58-byte client Finished record sent ahead of it. */
const TLS_AROUND: usize = 5 + 1 + 16 + 58;

/// The bytes a direct socket's sends carry, offered the way
/// `Exchange` offers them: the rest after each send.
fn carried(out: &[u8]) -> (Vec<u8>, usize) {
    let mut got = Vec::new();
    let mut sends = 0;
    while got.len() < out.len() {
        let piece = front(&out[got.len()..]);
        assert!(!piece.is_empty());
        assert!(OVERHEAD + piece.len() <= FRAME);
        got.extend_from_slice(piece);
        sends += 1;
    }
    (got, sends)
}

fn request_on(sepolia: bool) -> Vec<u8> {
    chain::pick(sepolia);
    let host = chain::rpc_host().as_bytes();
    let http = http_post(host, &snapshot_request(&ADDR));
    let mut out = http.clone();
    out.resize(http.len() + TLS_AROUND, 0xa5);
    out
}

fn quantity(id: u64, hex: &str) -> String {
    format!("{{\"jsonrpc\":\"2.0\",\"id\":{id},\"result\":\"0x{hex}\"}}")
}

fn word(n: u64) -> String {
    format!("{n:064x}")
}

const ETH: &str = "de0b6b3a7640000";
const NOX: &str = "3635c9adc5dea00000";
const USDC: &str = "5f5e100";

fn common() -> Vec<String> {
    vec![
        quantity(2, ETH),
        quantity(3, "7"),
        quantity(4, "3b9aca00"),
        quantity(10, NOX),
        quantity(15, USDC),
    ]
}

fn reply(parts: &[String]) -> Vec<u8> {
    format!("[{}]", parts.join(",")).into_bytes()
}

fn q(hex: &str) -> [u8; 32] {
    let v = u128::from_str_radix(hex, 16).unwrap();
    let mut w = [0u8; 32];
    w[16..].copy_from_slice(&v.to_be_bytes());
    w
}

#[test]
fn mainnet_request_needs_pieces_and_is_carried_whole() {
    let _held = hold();
    let out = request_on(false);
    assert!(out.len() > SEND_MAX, "the mainnet request no longer needs pieces");
    let (got, sends) = carried(&out);
    assert_eq!(got, out);
    assert!(sends >= 2);
}

#[test]
fn sepolia_request_is_carried_whole() {
    let _held = hold();
    let out = request_on(true);
    let (got, _) = carried(&out);
    assert_eq!(got, out);
}

#[test]
fn a_large_request_is_carried_in_order() {
    let out: Vec<u8> = (0..200_000u32).map(|i| i as u8).collect();
    let (got, sends) = carried(&out);
    assert_eq!(got, out);
    assert_eq!(sends, out.len().div_ceil(SEND_MAX));
}

#[test]
fn mainnet_reply_parses_every_field() {
    let _held = hold();
    chain::pick(false);
    let mut parts = common();
    parts.push(quantity(11, "2a"));
    parts.push(quantity(12, "3"));
    /* getProtocolStats: total staked is word 0, rewards 3, emission 5. */
    let stats: String = [1_000_000, 0, 0, 9, 0, 5].iter().map(|&n| word(n)).collect();
    parts.push(format!("{{\"jsonrpc\":\"2.0\",\"id\":13,\"result\":\"0x{stats}\"}}"));
    let info = format!("{}{}{}", word(0), word(0), word(2));
    parts.push(format!("{{\"jsonrpc\":\"2.0\",\"id\":14,\"result\":\"0x{info}\"}}"));
    let s = parse_snapshot(&reply(&parts));
    assert_eq!(s.eth_balance, Some(q(ETH)));
    assert_eq!(s.nox_balance, Some(q(NOX)));
    assert_eq!(s.usdc_balance, Some(q(USDC)));
    assert_eq!(s.nonce, Some(7));
    assert_eq!(s.fee, Some(1_000_000_000));
    assert_eq!(s.claimable, Some(q("2a")));
    assert_eq!(s.positions, Some(3));
    assert_eq!(s.passes, Some(2));
    let stats = s.stats.expect("stats");
    assert_eq!(stats.total, q("f4240"));
    assert_eq!(stats.rewards, q("9"));
}

#[test]
fn sepolia_reply_parses_and_asks_nothing_of_staking() {
    let _held = hold();
    chain::pick(true);
    let body = snapshot_request(&ADDR);
    let text = String::from_utf8(body).unwrap();
    assert!(!text.contains("\"id\":11"));
    assert!(text.contains("3e5249a65ca513d5e11260222e0d26f46b465d36"));
    let s = parse_snapshot(&reply(&common()));
    assert_eq!(s.eth_balance, Some(q(ETH)));
    assert_eq!(s.nox_balance, Some(q(NOX)));
    assert_eq!(s.usdc_balance, Some(q(USDC)));
    assert_eq!(s.nonce, Some(7));
    assert_eq!(s.fee, Some(1_000_000_000));
    assert_eq!(s.claimable, None);
    assert!(s.stats.is_none());
}

#[test]
fn a_pending_receipt_is_no_receipt_yet_not_a_revert() {
    use crate::wallet::rpc::parse_receipt_ok;
    assert_eq!(parse_receipt_ok(br#"{"jsonrpc":"2.0","id":6,"result":null}"#), None);
    assert_eq!(parse_receipt_ok(br#"{"jsonrpc":"2.0","id":6,"error":{"code":-32000}}"#), None);
    let ok = br#"{"jsonrpc":"2.0","id":6,"result":{"blockNumber":"0x10","status":"0x1"}}"#;
    assert_eq!(parse_receipt_ok(ok), Some(true));
    let reverted = br#"{"jsonrpc":"2.0","id":6,"result":{"blockNumber":"0x10","status":"0x0"}}"#;
    assert_eq!(parse_receipt_ok(reverted), Some(false));
}

#[test]
fn an_empty_quantity_is_no_number_not_zero() {
    use crate::wallet::rpc::{parse_quantity32, parse_u64};
    let empty = br#"{"jsonrpc":"2.0","id":10,"result":"0x"}"#;
    assert_eq!(parse_quantity32(empty), None, "an eth_call to an address with no code");
    assert_eq!(parse_u64(empty), None);
    let zero = br#"{"jsonrpc":"2.0","id":3,"result":"0x0"}"#;
    assert_eq!(parse_u64(zero), Some(0));
    assert_eq!(parse_quantity32(zero), Some([0; 32]));
}

#[test]
fn only_a_whole_200_answer_is_an_answer() {
    use crate::wallet::rpc::{http_answer, Http};
    let body = br#"[{"id":2,"result":"0x1"}]"#;
    let mut ok = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len()).into_bytes();
    ok.extend_from_slice(body);
    assert_eq!(http_answer(&ok), Http::Body(body.to_vec()));
    assert_eq!(http_answer(&ok[..ok.len() - 3]), Http::Incomplete, "a body cut short");
    let limited = b"HTTP/1.1 429 Too Many Requests\r\nContent-Length: 2\r\n\r\n{}";
    assert_eq!(http_answer(limited), Http::Status(429));
    assert_eq!(http_answer(b"HTTP/1.1 200 OK\r\nContent-Le"), Http::Incomplete);
    assert_eq!(http_answer(b"garbage"), Http::Incomplete);
}

#[test]
fn chunks_are_joined_so_no_size_line_splits_a_value() {
    use crate::wallet::rpc::{http_answer, parse_quantity32, Http};
    let chunked = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n\
        14\r\n{\"id\":2,\"result\":\"0x\r\n9\r\nde0b6b3a\"\r\n1\r\n}\r\n0\r\n\r\n";
    let Http::Body(body) = http_answer(chunked) else { panic!("a whole chunked body") };
    assert_eq!(body, br#"{"id":2,"result":"0xde0b6b3a"}"#.to_vec());
    assert!(parse_quantity32(&body).is_some());
    let unfinished = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5\r\nabcde\r\n";
    assert_eq!(http_answer(unfinished), Http::Incomplete, "no last chunk yet");
}

#[test]
fn a_reading_that_did_not_come_is_not_read_not_the_last_value() {
    use crate::reading::take;
    let (mut value, mut ready) = (0u64, false);
    assert!(take(Some(7), &mut value, &mut ready), "a first reading is shown");
    assert!((value, ready) == (7, true));
    assert!(!take(Some(7), &mut value, &mut ready), "the same reading changes nothing");
    assert!(take(None, &mut value, &mut ready), "a missing reading changes what is shown");
    assert!(!ready, "and is shown as not read");
    assert!(!take(None, &mut value, &mut ready), "still not read");
    assert!(take(Some(7), &mut value, &mut ready), "the same value read again is shown again");
}

#[test]
fn a_receipt_counts_only_for_the_transaction_it_names() {
    use crate::wallet::rpc::parse_receipt_for;
    let hash = [0xab; 32];
    let ours = format!(
        r#"{{"id":6,"result":{{"transactionHash":"0x{}","status":"0x1"}}}}"#,
        "AB".repeat(32)
    );
    assert_eq!(parse_receipt_for(ours.as_bytes(), &hash), Some(true), "capitals or not");
    let other = format!(
        r#"{{"id":6,"result":{{"transactionHash":"0x{}","status":"0x1"}}}}"#,
        "cd".repeat(32)
    );
    assert_eq!(parse_receipt_for(other.as_bytes(), &hash), None, "another transaction's receipt");
    assert_eq!(parse_receipt_for(br#"{"id":6,"result":null}"#, &hash), None);
}

#[test]
fn a_zero_or_missing_fee_is_refused_with_its_own_reason() {
    use crate::wallet::send::gas::fee_refusal;
    assert_ne!(fee_refusal(None), fee_refusal(Some(0)));
    assert!(fee_refusal(Some(1_000_000_000)).is_none());
}

#[test]
fn an_answer_cut_mid_record_is_told_from_one_that_fails_to_open() {
    use crate::records_whole::records_whole;
    let record = |len: usize| {
        let mut r = vec![23u8, 3, 3, (len >> 8) as u8, len as u8];
        r.extend(core::iter::repeat_n(0xAAu8, len));
        r
    };
    let mut two = record(40);
    two.extend(record(300));
    assert!(records_whole(&two), "two whole records");
    assert!(records_whole(&[]), "nothing is a run of no records");
    assert!(!records_whole(&two[..two.len() - 1]), "a record one byte short");
    assert!(!records_whole(&two[..3]), "a header cut short");
    let mut trailing = two.clone();
    trailing.push(23);
    assert!(!records_whole(&trailing), "a byte past the last record");
}
