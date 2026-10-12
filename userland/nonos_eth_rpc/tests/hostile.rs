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

//! Hostile answers from an RPC host (the owner's rule 8): random bytes, and
//! JSON and HTTP built to mislead. No parser may panic or loop.

use nonos_eth_rpc::*;

#[path = "common/bytes.rs"]
mod bytes_from_seed;
use bytes_from_seed::bytes;

fn misleading() -> Vec<Vec<u8>> {
    let (deep, hex) =
        ("[{\"a\":".repeat(5_000), format!("{{\"result\":\"0x{}\"}}", "f".repeat(10_000)));
    [
        r#"{"result":"0x"#.to_string(),
        r#"{"result":"0x1","result":"0x2"}"#.to_string(),
        r#"{"result":"0xzz"}"#.to_string(),
        r#"{"result":"\"0x1\\\""}"#.to_string(),
        r#"{"result":[{"logs":[{"topics":["0x"#.to_string(),
        r#"{"result":{"status":"0x1","logs":"#.to_string(),
        r#"{"result":{"baseFeePerGas":["0x1"],"reward":[["0x"]]}}"#.to_string(),
        "HTTP/1.1 200 OK\r\nContent-Length: 99999999999999999999\r\n\r\n{}".to_string(),
        "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\nffffffffffffffff\r\n".to_string(),
        deep,
        hex,
    ]
    .into_iter()
    .map(String::into_bytes)
    .collect()
}

#[test]
fn no_parser_panics_on_hostile_answers() {
    let lens = (0..300).chain([1024, 70_000]);
    let random = lens.flat_map(|n| [bytes(n as u64 + 3, n), vec![b'{'; n], vec![b'"'; n]]);
    for b in random.chain(misleading()) {
        let _ = (parse_u64(&b), parse_hash32(&b), parse_quantity32(&b), parse_call_word(&b, 3));
        let _ = (parse_logs(&b), parse_receipt_logs(&b), parse_receipt_ok(&b));
        let _ = (object_for_id(&b, 0), object_for_id(&b, u64::MAX));
        let _ = (parse_receipt_for(&b, &[1; 32]), parse_receipt_block(&b), parse_fee_history(&b));
        let _ = (objects(&b).len(), string_field(&b, b"result"), http_answer(&b));
        let _ = (broadcast_error(&b), hex_bytes(&b), quantity(&b), http_body(&b));
    }
}

#[test]
fn fees_stay_in_bounds_for_any_tips() {
    for tips in [vec![], vec![u128::MAX; 9], vec![0, u128::MAX, 1], vec![1; 1000]] {
        let (tip, cap) = fees(u128::MAX, tips.clone());
        assert!(tip <= cap && tip >= MIN_TIP_WEI, "{tips:?}");
        assert!(fees(0, tips).0 >= MIN_TIP_WEI);
    }
}
