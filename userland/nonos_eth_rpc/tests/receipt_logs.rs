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

use nonos_eth_rpc::{parse_receipt_logs, parse_receipt_ok};

fn log(n: u8) -> String {
    format!(
        "{{\"address\":\"0x{a}\",\"topics\":[\"0x{t}\"],\"data\":\"0x{n:02x}\",\"blockNumber\":\"0x9\",\"transactionHash\":\"0x{h}\",\"removed\":false}}",
        a = "aa".repeat(20),
        t = "0a".repeat(32),
        h = "cc".repeat(32),
    )
}

#[test]
fn a_receipts_logs_are_its_own_array_only() {
    let resp = format!(
        "{{\"id\":1,\"result\":{{\"status\":\"0x1\",\"logs\":[{},{}],\"logsBloom\":\"0x00\",\"extra\":{{\"x\":\"]{{\"}}}}}}",
        log(1),
        log(2)
    );
    let logs = parse_receipt_logs(resp.as_bytes()).unwrap();
    assert_eq!(logs.iter().map(|l| l.data.clone()).collect::<Vec<_>>(), vec![vec![1], vec![2]]);
    assert_eq!(parse_receipt_ok(resp.as_bytes()), Some(true));
    assert_eq!(parse_receipt_logs(b"{\"id\":1,\"result\":null}"), None, "not mined");
    let empty = b"{\"result\":{\"status\":\"0x0\",\"logs\":[]}}";
    assert_eq!(parse_receipt_logs(empty), Some(vec![]));
    let cut = &resp[..resp.find("],\"logsBloom").unwrap()];
    assert_eq!(parse_receipt_logs(cut.as_bytes()), None, "an array that never closes");
}
