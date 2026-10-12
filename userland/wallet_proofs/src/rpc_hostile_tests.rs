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

/*
 * What an RPC host, a TLS peer or settle can send the wallet: random bytes,
 * runs of the characters the scanners key on, and answers built to mislead.
 * No reader may panic or loop.
 */

use crate::fees::parse_fee_history;
use crate::records_whole::records_whole;
use crate::settle_heard::{heard, Asked};
use crate::wallet::rpc::*;

fn bytes(seed: u64, len: usize) -> Vec<u8> {
    let mut x = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    (0..len)
        .map(|_| {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            x.to_le_bytes()[0]
        })
        .collect()
}

fn misleading() -> Vec<Vec<u8>> {
    let hex = format!("{{\"result\":\"0x{}\"}}", "f".repeat(10_000));
    [
        r#"{"result":"0x"#.to_string(),
        r#"{"result":"0x1","result":"0x2"}"#.to_string(),
        r#"[{"id":1,"result":"0x1"},{"id":1,"result":"0x2"}]"#.to_string(),
        r#"{"result":{"status":"0x1","blockNumber":"0x"}}"#.to_string(),
        r#"{"result":{"baseFeePerGas":["0x1"],"reward":[["0x"]]}}"#.to_string(),
        "HTTP/1.1 200 OK\r\nContent-Length: 99999999999999999999\r\n\r\n{}".to_string(),
        "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\nffffffffffffffff\r\n".to_string(),
        "[{\"a\":".repeat(5_000),
        hex,
    ]
    .into_iter()
    .map(String::into_bytes)
    .collect()
}

#[test]
fn no_wallet_reader_panics_on_hostile_answers() {
    let asked = Asked { op: 3, id: 7, settle_pid: 42, at_ms: 0 };
    let lens = (0..300).chain([1024, 70_000]);
    let random = lens.flat_map(|n| [bytes(n as u64 + 5, n), vec![b'{'; n], vec![0x17; n]]);
    for b in random.chain(misleading()) {
        let _ = (parse_u64(&b), parse_hash32(&b), parse_quantity32(&b), parse_call_word(&b, 3));
        let _ = (parse_receipt_ok(&b), parse_receipt_for(&b, &[1; 32]), parse_receipt_block(&b));
        let _ = (object_for_id(&b, 1), broadcast_error(&b), http_answer(&b));
        let _ = (parse_fee_history(&b), records_whole(&b));
        let _ = (heard(Some(&asked), 42, &b), heard(None, 0, &b));
    }
}
