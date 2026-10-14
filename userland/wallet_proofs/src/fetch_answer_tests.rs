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

//! A pending receipt, an empty quantity, a whole 200, and joined chunks.

#[test]
fn a_pending_receipt_is_no_receipt_yet_not_a_revert() {
    use nonos_eth_rpc::parse_receipt_ok;
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
