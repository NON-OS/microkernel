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

//! Each request is the JSON-RPC its name says, with its fields as hex.

use nonos_eth_rpc::*;

fn has(body: &[u8], s: &[u8]) -> bool {
    body.windows(s.len()).any(|w| w == s)
}

#[test]
fn the_wallets_requests_name_their_methods() {
    let addr = [0x11u8; 20];
    assert!(request_chain_id(1).starts_with(b"{\"jsonrpc\""));
    assert!(has(&request_balance(&addr, 2), b"eth_getBalance"));
    assert!(has(&request_nonce(&addr, 3), b"eth_getTransactionCount"));
    assert!(has(&request_fee(4), b"eth_gasPrice"));
    assert!(has(&request_broadcast(&[0x02, 0xf8, 0x01], 5), b"\"0x02f801\""));
    assert!(has(&request_receipt(&[0x22; 32], 6), b"eth_getTransactionReceipt"));
    let call = request_eth_call(&addr, &[0xab, 0xcd], 7);
    assert!(
        has(&call, b"\"to\":\"0x1111111111111111111111111111111111111111\"")
            && has(&call, b"\"data\":\"0xabcd\"")
    );
}

#[test]
fn get_logs_names_one_address_one_topic_and_a_range() {
    let r = request_logs(&[0xaa; 20], &[0x0f; 32], 0, 0x1234, 9);
    assert!(has(&r, b"\"method\":\"eth_getLogs\""));
    assert!(has(&r, b"\"address\":\"0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\""));
    assert!(has(&r, b"\"topics\":[\"0x0f0f"));
    assert!(has(&r, b"\"fromBlock\":\"0x0\",\"toBlock\":\"0x1234\""));
    assert!(has(&r, b"\"id\":9}"));
    let mut q = Vec::new();
    append_quantity(&mut q, u64::MAX);
    assert_eq!(q, b"0xffffffffffffffff");
}

#[test]
fn a_post_and_a_batch_are_framed() {
    let post = http_post(b"ethereum-rpc.publicnode.com", b"{}");
    assert!(post.starts_with(b"POST / HTTP/1.1\r\nHost: ethereum-rpc.publicnode.com"));
    let b = request_batch(&[b"{\"id\":1}", b"{\"id\":20}"]);
    assert_eq!(b, b"[{\"id\":1},{\"id\":20}]");
    assert_eq!(object_for_id(&b, 20), Some(&b"{\"id\":20}"[..]));
    assert_eq!(object_for_id(&b, 2), None);
}
