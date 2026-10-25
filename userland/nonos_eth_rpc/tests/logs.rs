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

//! eth_getLogs answers: every log read whole, removed ones dropped, and a
//! single unreadable log refusing the page.

use nonos_eth_rpc::{parse_logs, Log};

fn log(block: u64, removed: bool, topic: &str) -> String {
    format!(
        "{{\"address\":\"0x{a}\",\"topics\":[\"0x{t}\",\"0x{u}\"],\"data\":\"0x0102\",\"blockNumber\":\"0x{block:x}\",\"transactionHash\":\"0x{h}\",\"logIndex\":\"0x0\",\"removed\":{removed}}}",
        a = "aa".repeat(20),
        t = topic,
        u = "0b".repeat(32),
        h = "cc".repeat(32),
    )
}

#[test]
fn logs_read_in_order_and_a_removed_one_is_dropped() {
    let t = "0a".repeat(32);
    let resp = format!(
        "{{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":[{},{},{}]}}",
        log(5, false, &t),
        log(6, true, &t),
        log(7, false, &t)
    );
    let logs = parse_logs(resp.as_bytes()).unwrap();
    assert_eq!(logs.len(), 2);
    let first = Log {
        address: [0xaa; 20],
        topics: vec![[0x0a; 32], [0x0b; 32]],
        data: vec![1, 2],
        block: 5,
        tx: [0xcc; 32],
    };
    assert_eq!(logs[0], first);
    assert_eq!(logs[1].block, 7);
    assert_eq!(parse_logs(b"{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":[]}"), Some(vec![]));
}

#[test]
fn an_error_answer_or_one_bad_log_refuses_the_page() {
    assert_eq!(parse_logs(b"{\"jsonrpc\":\"2.0\",\"id\":1,\"error\":{\"code\":-32005}}"), None);
    let short = "0a".repeat(31);
    let resp =
        format!("{{\"result\":[{},{}]}}", log(5, false, &"0a".repeat(32)), log(6, false, &short));
    assert_eq!(parse_logs(resp.as_bytes()), None);
}

#[test]
fn block_pages_cover_the_range_once() {
    use nonos_eth_rpc::{block_pages, LOG_PAGE_BLOCKS};
    let p: Vec<_> = block_pages(100, 25_000, LOG_PAGE_BLOCKS).collect();
    assert_eq!(p, [(100, 10_099), (10_100, 20_099), (20_100, 25_000)]);
    assert_eq!(block_pages(5, 5, 10).collect::<Vec<_>>(), [(5, 5)]);
    assert_eq!(block_pages(6, 5, 10).count(), 0);
    assert_eq!(block_pages(0, 9, 0).count(), 0);
    let top: Vec<_> = block_pages(u64::MAX - 3, u64::MAX, 2).collect();
    assert_eq!(top, [(u64::MAX - 3, u64::MAX - 2), (u64::MAX - 1, u64::MAX)]);
}
