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

//! Which record in a response is the answer: one of the type asked for, owned
//! by the name asked for or by a name the CNAME chain from it leads to, read
//! through compression pointers that only ever point back.

use crate::dns::{first_address, HDR_LEN};

extern crate alloc;
use alloc::vec::Vec;

const A: u16 = 1;
const CNAME: u16 = 5;
const AAAA: u16 = 28;

/// A name in wire form, uncompressed.
fn name(dotted: &str) -> Vec<u8> {
    let mut out = Vec::new();
    for label in dotted.split('.').filter(|l| !l.is_empty()) {
        out.push(label.len() as u8);
        out.extend_from_slice(label.as_bytes());
    }
    out.push(0);
    out
}

/// A response header and one question, `qtype` for `qname`.
fn response(qname: &str, qtype: u16, answers: u16) -> Vec<u8> {
    let mut m = alloc::vec![0x12, 0x34, 0x81, 0x80, 0, 1, 0, answers as u8, 0, 0, 0, 0];
    m.extend_from_slice(&name(qname));
    m.extend_from_slice(&qtype.to_be_bytes());
    m.extend_from_slice(&[0, 1]);
    m
}

/// Append a record: owner as given (wire bytes, may be a pointer), then type,
/// class IN, TTL and the data.
fn record(m: &mut Vec<u8>, owner: &[u8], rtype: u16, ttl: u32, rdata: &[u8]) {
    m.extend_from_slice(owner);
    m.extend_from_slice(&rtype.to_be_bytes());
    m.extend_from_slice(&[0, 1]);
    m.extend_from_slice(&ttl.to_be_bytes());
    m.extend_from_slice(&(rdata.len() as u16).to_be_bytes());
    m.extend_from_slice(rdata);
}

/// A pointer to the question name.
const QNAME: [u8; 2] = [0xC0, HDR_LEN as u8];

fn answer_v4(m: &[u8]) -> Option<[u8; 4]> {
    first_address(m).ok().and_then(|(_, a)| a).and_then(|a| a.ipv4)
}

/*
 * An address record for some other name is not the answer, wherever it sits
 * in the response. The first A or AAAA of any owner was taken.
 */
#[test]
fn an_address_for_another_name_is_not_the_answer() {
    let mut m = response("example.com", A, 1);
    record(&mut m, &name("evil.example"), A, 300, &[6, 6, 6, 6]);
    assert_eq!(answer_v4(&m), None);
}

#[test]
fn the_cname_chain_is_followed_past_a_decoy() {
    let mut m = response("www.example.com", A, 3);
    record(&mut m, &name("evil.example"), A, 300, &[6, 6, 6, 6]);
    record(&mut m, &QNAME, CNAME, 300, &name("cdn.example.net"));
    record(&mut m, &name("cdn.example.net"), A, 60, &[1, 2, 3, 4]);
    assert_eq!(answer_v4(&m), Some([1, 2, 3, 4]));
}

#[test]
fn a_chain_given_out_of_order_is_still_followed() {
    let mut m = response("a.example", A, 3);
    record(&mut m, &name("c.example"), A, 60, &[9, 9, 9, 9]);
    record(&mut m, &name("b.example"), CNAME, 60, &name("c.example"));
    record(&mut m, &QNAME, CNAME, 60, &name("b.example"));
    assert_eq!(answer_v4(&m), Some([9, 9, 9, 9]));
}

#[test]
fn a_cname_loop_ends_with_no_answer() {
    let mut m = response("a.example", A, 2);
    record(&mut m, &QNAME, CNAME, 60, &name("b.example"));
    record(&mut m, &name("b.example"), CNAME, 60, &name("a.example"));
    assert_eq!(answer_v4(&m), None);
}

/*
 * Asked for A, a response whose first record is the name's AAAA still has
 * an A answer after it. The AAAA was returned, and the A lookup then failed.
 */
#[test]
fn only_the_type_asked_for_is_the_answer() {
    let mut m = response("example.com", A, 2);
    record(&mut m, &QNAME, AAAA, 60, &[0x20, 1, 0xd, 0xb8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]);
    record(&mut m, &QNAME, A, 60, &[192, 0, 2, 1]);
    assert_eq!(answer_v4(&m), Some([192, 0, 2, 1]));
}

#[test]
fn owner_names_match_without_case() {
    let mut m = response("Example.COM", A, 1);
    record(&mut m, &name("eXample.com"), A, 60, &[192, 0, 2, 7]);
    assert_eq!(answer_v4(&m), Some([192, 0, 2, 7]));
}

/// RFC 2181 8: a TTL with the top bit set is read as zero, and a chain is
/// only as fresh as its least fresh link.
#[test]
fn ttls_are_sane_and_the_chain_takes_the_least() {
    let mut m = response("example.com", A, 1);
    record(&mut m, &QNAME, A, 0x8000_0001, &[192, 0, 2, 1]);
    let Ok((_, a)) = first_address(&m) else { panic!("parses") };
    assert_eq!(a.expect("answer").ttl, 0);
    let mut c = response("www.example.com", A, 2);
    record(&mut c, &QNAME, CNAME, 30, &name("example.com"));
    record(&mut c, &name("example.com"), A, 3600, &[192, 0, 2, 1]);
    let Ok((_, a)) = first_address(&c) else { panic!("parses") };
    assert_eq!(a.expect("answer").ttl, 30);
}

/// A name is at most 255 bytes on the wire (RFC 1035 3.1); a longer one is
/// not a name.
#[test]
fn a_name_longer_than_255_bytes_is_refused() {
    let long: Vec<&str> = (0..5).map(|_| "a123456789b123456789c123456789d123456789e123456789f1234567").collect();
    let qname = long.join(".");
    assert!(name(&qname).len() > 255);
    let mut m = response(&qname, A, 1);
    record(&mut m, &QNAME, A, 60, &[1, 1, 1, 1]);
    assert!(first_address(&m).is_err());
}

/*
 * Compression pointers lead only backwards, each to before where the run of
 * labels it ends began, so no chain of them can come round again. A pointer
 * back into its own name, at itself or forward is refused.
 */
#[test]
fn pointers_that_do_not_lead_back_are_refused() {
    let base = response("example.com", A, 1);
    let at = base.len() as u8;
    // "x" then a pointer back to the "x": a loop through a label.
    let looped = [1, b'x', 0xC0, at];
    // A pointer to itself, and one to just past itself.
    let own = [0xC0, at];
    let forward = [0xC0, at + 2, 0];
    for owner in [&looped[..], &own[..], &forward[..]] {
        let mut m = base.clone();
        record(&mut m, owner, A, 60, &[1, 1, 1, 1]);
        assert_eq!(answer_v4(&m), None, "{owner:?}");
    }
}

#[test]
fn a_pointer_chain_back_through_earlier_names_is_read() {
    // Question www.example.com; the answer owner is "w2" then a pointer to
    // the "example.com" inside the question: w2.example.com, not the qname.
    let mut m = response("www.example.com", A, 2);
    let example_at = (HDR_LEN + 4) as u8;
    record(&mut m, &[2, b'w', b'2', 0xC0, example_at], A, 60, &[2, 2, 2, 2]);
    record(&mut m, &QNAME, A, 60, &[3, 3, 3, 3]);
    assert_eq!(answer_v4(&m), Some([3, 3, 3, 3]), "w2.example.com is another name");
}

fn xorshift(state: &mut u32) -> u32 {
    *state ^= *state << 13;
    *state ^= *state >> 17;
    *state ^= *state << 5;
    *state
}

/// A name laid into a query by the capsule's own encoder reads back as the
/// same labels, lowercased.
#[test]
fn a_name_written_into_a_query_reads_back_the_same() {
    const CHARS: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-";
    let mut s = 0xD15C_0001u32;
    for _ in 0..50_000 {
        let labels: Vec<alloc::string::String> = (0..1 + xorshift(&mut s) % 6)
            .map(|_| {
                let len = 1 + (xorshift(&mut s) % 40) as usize;
                (0..len).map(|_| CHARS[xorshift(&mut s) as usize % CHARS.len()] as char).collect()
            })
            .collect();
        let dotted = labels.join(".");
        let mut q = [0u8; 512];
        let Ok(len) = crate::dns::build_a_query(7, &dotted, &mut q) else { continue };
        let Ok((read, end)) = crate::dns::read_name(&q[..len], HDR_LEN) else {
            panic!("{dotted} reads back")
        };
        assert_eq!(read.wire(), name(&dotted.to_ascii_lowercase()).as_slice());
        assert_eq!(end, len - 4, "the name ends where the type begins");
    }
}

/// Over random bytes and random starting points the reader never panics,
/// ends inside the message, and yields only names within the limits.
#[test]
fn reading_names_from_noise_stays_within_every_limit() {
    let mut s = 0x0DD5_0EEDu32;
    for _ in 0..100_000 {
        let len = (xorshift(&mut s) % 160) as usize;
        let m: Vec<u8> = (0..len)
            .map(|_| match xorshift(&mut s) % 4 {
                0 => 0xC0 | (xorshift(&mut s) % 2) as u8,
                1 => (xorshift(&mut s) % 8) as u8,
                _ => xorshift(&mut s) as u8,
            })
            .collect();
        let start = if len == 0 { 0 } else { xorshift(&mut s) as usize % len };
        if let Ok((n, end)) = crate::dns::read_name(&m, start) {
            assert!(end > start && end <= m.len());
            let w = n.wire();
            assert!(w.len() <= 255 && w.last() == Some(&0));
            let mut at = 0;
            while w[at] != 0 {
                assert!(w[at] <= 63, "a label longer than 63");
                at += 1 + w[at] as usize;
            }
            assert_eq!(at + 1, w.len());
        }
    }
}
