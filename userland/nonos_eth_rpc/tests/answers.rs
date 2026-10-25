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

//! Answers read strictly: the HTTP framing whole, the result where it is.

use nonos_eth_rpc::*;

#[test]
fn a_whole_200_answer_gives_its_body_and_anything_else_does_not() {
    let ok = b"HTTP/1.1 200 OK\r\nContent-Length: 39\r\n\r\n{\"jsonrpc\":\"2.0\",\"result\":\"0x1\",\"id\":1}";
    let Http::Body(body) = http_answer(ok) else { panic!("whole") };
    assert_eq!(parse_u64(&body), Some(1));
    assert_eq!(http_answer(&ok[..ok.len() - 1]), Http::Incomplete);
    assert_eq!(http_answer(b"HTTP/1.1 429 Too Many\r\n\r\n"), Http::Status(429));
    let chunked = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n3\r\nabc\r\n0\r\n\r\n";
    assert_eq!(http_answer(chunked), Http::Body(b"abc".to_vec()));
    assert_eq!(http_answer(&chunked[..chunked.len() - 5]), Http::Incomplete);
}

#[test]
fn quantities_and_hashes_read_from_the_result() {
    let balance = b"{\"jsonrpc\":\"2.0\",\"result\":\"0xde0b6b3a7640000\",\"id\":1}";
    let q = parse_quantity32(balance).unwrap();
    assert_eq!(&q[24..], &1_000_000_000_000_000_000u64.to_be_bytes());
    let h = b"{\"result\":\"0x2222222222222222222222222222222222222222222222222222222222222222\"}";
    assert_eq!(parse_hash32(h), Some([0x22; 32]));
    assert_eq!(quantity(b"0x10"), Some(16));
    assert_eq!(
        (quantity(b"10"), quantity(b"0x"), quantity(b"0x11111111111111111")),
        (None, None, None)
    );
    assert_eq!(hex_bytes(b"0x0aFf"), Some(vec![0x0a, 0xff]));
    assert_eq!((hex_bytes(b"0x0"), hex_bytes(b"0xzz")), (None, None));
}
