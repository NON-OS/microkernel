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

//! Random and damaged responses never panic the parser, and a response laid
//! out from parts parses back to those parts.

use nonos_http::parse_response;

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

const ALPHABET: &[u8] = b"HTTP/1.0 2\r\n:;-,0123456789abcdefABCDEF \tContent-Length Transfer-Encoding chunked";

#[test]
fn random_bytes_never_panic() {
    let mut s = 0x0BAD_F00Du32;
    for _ in 0..100_000 {
        let len = xorshift(&mut s) % 300;
        let raw: Vec<u8> = (0..len)
            .map(|_| {
                let r = xorshift(&mut s);
                if r % 3 == 0 { r as u8 } else { ALPHABET[r as usize % ALPHABET.len()] }
            })
            .collect();
        if let Ok(r) = parse_response(&raw) {
            assert!(r.body.len() <= raw.len(), "a body is never longer than the input");
        }
    }
}

#[test]
fn damaged_real_responses_never_panic() {
    let seeds: [&[u8]; 3] = [
        b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\nX: y\r\n\r\nhello",
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n5;e\r\nhello\r\n3\r\nabc\r\n0\r\nT: x\r\n\r\n",
        b"HTTP/1.1 103 Early\r\nLink: <a>\r\n\r\nHTTP/1.1 304 NM\r\nContent-Length: 9\r\n\r\n",
    ];
    let mut s = 0x5EED_0001u32;
    for round in 0..100_000u32 {
        let mut raw = seeds[(round % 3) as usize].to_vec();
        for _ in 0..1 + xorshift(&mut s) % 3 {
            let at = xorshift(&mut s) as usize % raw.len();
            match xorshift(&mut s) % 3 {
                0 => raw[at] = xorshift(&mut s) as u8,
                1 => raw.insert(at, ALPHABET[xorshift(&mut s) as usize % ALPHABET.len()]),
                _ => {
                    raw.remove(at);
                }
            }
            if raw.is_empty() {
                break;
            }
        }
        let _ = parse_response(&raw);
    }
}

/// A response laid out from a status, fields and a body, framed by length or
/// by chunks, parses back to exactly those.
#[test]
fn a_response_laid_out_from_parts_parses_back_to_them() {
    let mut s = 0xFEED_BEEFu32;
    for _ in 0..20_000 {
        let code = 200 + (xorshift(&mut s) % 300) as u16;
        let code = if code == 204 || code == 304 { 200 } else { code };
        let fields: Vec<(String, String)> = (0..xorshift(&mut s) % 6)
            .map(|i| (format!("x-f{i}-{}", xorshift(&mut s) % 100), format!("v{}", xorshift(&mut s))))
            .collect();
        let body: Vec<u8> = (0..xorshift(&mut s) % 200).map(|_| xorshift(&mut s) as u8).collect();
        let chunked = xorshift(&mut s) % 2 == 0;
        let mut raw = format!("HTTP/1.1 {code} Reason\r\n").into_bytes();
        for (n, v) in &fields {
            raw.extend_from_slice(format!("{n}: {v}\r\n").as_bytes());
        }
        if chunked {
            raw.extend_from_slice(b"Transfer-Encoding: chunked\r\n\r\n");
            for piece in body.chunks(1 + xorshift(&mut s) as usize % 50) {
                raw.extend_from_slice(format!("{:x}\r\n", piece.len()).as_bytes());
                raw.extend_from_slice(piece);
                raw.extend_from_slice(b"\r\n");
            }
            raw.extend_from_slice(b"0\r\n\r\n");
        } else {
            raw.extend_from_slice(format!("Content-Length: {}\r\n\r\n", body.len()).as_bytes());
            raw.extend_from_slice(&body);
        }
        let r = parse_response(&raw).expect("a well formed response parses");
        assert_eq!(r.status, code);
        assert_eq!(r.body, body);
        for (n, v) in &fields {
            assert_eq!(r.header(n), Some(v.as_str()));
        }
    }
}
