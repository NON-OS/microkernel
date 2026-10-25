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

//! What the VMBus core may post: the guest's channel messages alone, long
//! enough, aimed at VP 0, and naming only pages the caller holds.

use super::vmbus_out::{check, Refusal};

const HELD: u64 = 0x10_0000;
const PAGES: u64 = 4;

fn held(gpa: u64) -> bool {
    (HELD..HELD + PAGES * 4096).contains(&gpa) && gpa.is_multiple_of(4096)
}

fn msg(kind: u32, len: usize) -> Vec<u8> {
    let mut m = vec![0u8; len];
    m[0..4].copy_from_slice(&kind.to_le_bytes());
    m
}

fn put32(m: &mut [u8], at: usize, v: u32) {
    m[at..at + 4].copy_from_slice(&v.to_le_bytes());
}

fn put64(m: &mut [u8], at: usize, v: u64) {
    m[at..at + 8].copy_from_slice(&v.to_le_bytes());
}

fn contact(version: u32, vcpu: u32, third: u64, m1: u64, m2: u64) -> Vec<u8> {
    let mut m = msg(14, 40);
    put32(&mut m, 8, version);
    put32(&mut m, 12, vcpu);
    put64(&mut m, 16, third);
    put64(&mut m, 24, m1);
    put64(&mut m, 32, m2);
    m
}

#[test]
fn only_channel_messages_on_a_channel_connection() {
    let m = msg(3, 8);
    assert_eq!(check(1, 1, &m, held), Ok(()));
    assert_eq!(check(4, 1, &m, held), Ok(()));
    assert_eq!(check(2, 1, &m, held), Err(Refusal::Route));
    assert_eq!(check(1, 2, &m, held), Err(Refusal::Route));
}

#[test]
fn the_guest_s_messages_and_no_other() {
    for (kind, len) in [(3, 8), (16, 8), (7, 12), (13, 12), (11, 16), (5, 148)] {
        assert_eq!(check(1, 1, &msg(kind, len), held), Ok(()), "kind {kind}");
    }
    // Offers, the host's answers, hv_sock, modify-channel, anything unknown.
    for kind in [0, 1, 2, 4, 6, 10, 12, 15, 17, 18, 21, 22, 99, u32::MAX] {
        assert_eq!(check(1, 1, &msg(kind, 148), held), Err(Refusal::Kind), "kind {kind}");
    }
}

#[test]
fn each_is_long_enough_for_its_fields() {
    for (kind, len) in [(3, 8), (7, 12), (11, 16), (5, 28), (14, 40), (8, 28), (9, 24)] {
        let short = msg(kind, len - 1);
        assert_eq!(check(1, 1, &short, held), Err(Refusal::Short), "kind {kind}");
    }
    assert_eq!(check(1, 1, &[3, 0, 0], held), Err(Refusal::Short));
}

#[test]
fn a_contact_from_five_names_sint_two_and_held_monitor_pages() {
    let ok = contact(5 << 16, 0, 2, HELD, HELD + 4096);
    assert_eq!(check(4, 1, &ok, held), Ok(()));
    assert_eq!(check(4, 1, &contact(5 << 16, 1, 2, HELD, HELD), held), Err(Refusal::Target));
    assert_eq!(check(4, 1, &contact(5 << 16, 0, 3, HELD, HELD), held), Err(Refusal::Target));
    assert_eq!(check(4, 1, &contact(5 << 16, 0, 2, 0x2000, HELD), held), Err(Refusal::Page));
    assert_eq!(check(4, 1, &contact(5 << 16, 0, 2, HELD, 0x2000), held), Err(Refusal::Page));
}

#[test]
fn a_contact_before_five_names_a_held_interrupt_page() {
    let win8 = 2 << 16 | 4;
    assert_eq!(check(1, 1, &contact(win8, 0, HELD + 8192, HELD, HELD + 4096), held), Ok(()));
    assert_eq!(
        check(1, 1, &contact(win8, 0, 0x5000, HELD, HELD + 4096), held),
        Err(Refusal::Page),
        "the kernel's page, or anyone's"
    );
}

#[test]
fn a_channel_opens_on_vp_zero() {
    let mut m = msg(5, 148);
    assert_eq!(check(1, 1, &m, held), Ok(()));
    put32(&mut m, 20, 1);
    assert_eq!(check(1, 1, &m, held), Err(Refusal::Target));
}

fn gpadl_header(ranges: u16, pfns: &[u64]) -> Vec<u8> {
    let mut m = msg(8, 28 + 8 * pfns.len());
    put32(&mut m, 16, (ranges as u32) << 16 | (8 + 8 * pfns.len() as u32));
    put32(&mut m, 20, 4096 * pfns.len() as u32);
    for (i, p) in pfns.iter().enumerate() {
        put64(&mut m, 28 + 8 * i, *p);
    }
    m
}

#[test]
fn a_gpadl_names_only_held_frames() {
    let pfn = HELD / 4096;
    assert_eq!(check(1, 1, &gpadl_header(1, &[pfn, pfn + 1, pfn + 3]), held), Ok(()));
    assert_eq!(check(1, 1, &gpadl_header(1, &[pfn, 0x42]), held), Err(Refusal::Page));
    assert_eq!(check(1, 1, &gpadl_header(1, &[pfn + PAGES]), held), Err(Refusal::Page));
    assert_eq!(check(1, 1, &gpadl_header(1, &[u64::MAX]), held), Err(Refusal::Page));
    let wraps = pfn + (1 << 52);
    assert_eq!(
        check(1, 1, &gpadl_header(1, &[wraps]), held),
        Err(Refusal::Page),
        "a frame whose address wraps onto a held one"
    );
    assert_eq!(check(1, 1, &gpadl_header(2, &[pfn]), held), Err(Refusal::Kind), "one range");
    assert_eq!(check(1, 1, &gpadl_header(1, &[]), held), Err(Refusal::Short));
    let mut ragged = gpadl_header(1, &[pfn]);
    ragged.push(0);
    assert_eq!(check(1, 1, &ragged, held), Err(Refusal::Short));

    let mut body = msg(9, 16 + 16);
    put64(&mut body, 16, pfn + 2);
    put64(&mut body, 24, pfn + 1);
    assert_eq!(check(1, 1, &body, held), Ok(()));
    put64(&mut body, 24, 1);
    assert_eq!(check(1, 1, &body, held), Err(Refusal::Page));
}
