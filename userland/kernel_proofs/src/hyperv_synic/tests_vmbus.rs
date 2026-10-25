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
//! enough, aimed at VP 0, naming only pages held where they must be: the
//! contact's on the root by the core, a GPADL's on the record of a channel
//! the host offered and the core gave a driver. An open must use that
//! channel's own GPADL.

use super::channels::{Channels, Retire};
use super::vmbus_out::{check, Holder, Refusal};

/// The core's pages, on the root.
const ROOT: u64 = 0x10_0000;
/// The channel driver's pages, on its record.
const RING: u64 = 0x80_0000;
const PAGES: u64 = 4;
const RECORD: u64 = 0x1_0000_00AA;
const RELID: u32 = 5;
pub(super) const SCSI: [u8; 16] = [
    0xd9, 0x63, 0x61, 0xba, 0xa1, 0x04, 0x29, 0x4d, 0xb6, 0x05, 0x72, 0xe2, 0xff, 0xb1, 0xdc, 0x7f,
];

fn within(base: u64, gpa: u64) -> bool {
    (base..base + PAGES * 4096).contains(&gpa) && gpa.is_multiple_of(4096)
}

fn held(h: Holder, gpa: u64) -> bool {
    match h {
        Holder::Root => within(ROOT, gpa),
        Holder::Record(RECORD) => within(RING, gpa),
        Holder::Record(_) => false,
    }
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

pub(super) fn offer(relid: u32, interface: [u8; 16], connection: u32) -> Vec<u8> {
    let mut m = msg(1, 196);
    m[8..24].copy_from_slice(&interface);
    put32(&mut m, 184, relid);
    put32(&mut m, 192, connection);
    m
}

/// A SCSI channel offered and given to `RECORD`.
fn world() -> Channels {
    let mut c = Channels::new();
    assert_eq!(c.take_host_message(&offer(RELID, SCSI, 0x2005)), Retire::None);
    assert!(c.bind(RELID, RECORD).is_ok());
    c
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

pub(super) fn gpadl_header(relid: u32, id: u32, ranges: u16, pfns: &[u64]) -> Vec<u8> {
    let mut m = msg(8, 28 + 8 * pfns.len());
    put32(&mut m, 8, relid);
    put32(&mut m, 12, id);
    put32(&mut m, 16, (ranges as u32) << 16 | (8 + 8 * pfns.len() as u32));
    put32(&mut m, 20, 4096 * pfns.len() as u32);
    for (i, p) in pfns.iter().enumerate() {
        put64(&mut m, 28 + 8 * i, *p);
    }
    m
}

fn gpadl_body(id: u32, pfns: &[u64]) -> Vec<u8> {
    let mut m = msg(9, 16 + 8 * pfns.len());
    put32(&mut m, 12, id);
    for (i, p) in pfns.iter().enumerate() {
        put64(&mut m, 16 + 8 * i, *p);
    }
    m
}

fn open(relid: u32, gpadl: u32, vcpu: u32) -> Vec<u8> {
    let mut m = msg(5, 148);
    put32(&mut m, 8, relid);
    put32(&mut m, 16, gpadl);
    put32(&mut m, 20, vcpu);
    m
}

/// Post `m` as the kernel does: checked, then noted once the host took it.
fn post(c: &mut Channels, m: &[u8]) -> Result<(), Refusal> {
    check(1, 1, m, c, held)?;
    c.take_guest_message(m);
    Ok(())
}

const RING_PFN: u64 = RING / 4096;
const ROOT_PFN: u64 = ROOT / 4096;

#[test]
fn only_channel_messages_on_a_channel_connection() {
    let c = world();
    let m = msg(3, 8);
    assert_eq!(check(1, 1, &m, &c, held), Ok(()));
    assert_eq!(check(4, 1, &m, &c, held), Ok(()));
    assert_eq!(check(2, 1, &m, &c, held), Err(Refusal::Route));
    assert_eq!(check(1, 2, &m, &c, held), Err(Refusal::Route));
}

#[test]
fn the_guest_s_messages_and_no_other() {
    let c = world();
    for (kind, len) in [(3, 8), (16, 8), (7, 12), (13, 12), (11, 16)] {
        assert_eq!(check(1, 1, &msg(kind, len), &c, held), Ok(()), "kind {kind}");
    }
    // Offers, the host's answers, hv_sock, modify-channel, anything unknown.
    for kind in [0, 1, 2, 4, 6, 10, 12, 15, 17, 18, 21, 22, 99, u32::MAX] {
        assert_eq!(check(1, 1, &msg(kind, 148), &c, held), Err(Refusal::Kind), "kind {kind}");
    }
}

#[test]
fn each_is_long_enough_for_its_fields() {
    let c = world();
    for (kind, len) in [(3, 8), (7, 12), (11, 16), (5, 28), (14, 40), (8, 28), (9, 16)] {
        let short = msg(kind, len - 1);
        assert_eq!(check(1, 1, &short, &c, held), Err(Refusal::Short), "kind {kind}");
    }
    assert_eq!(check(1, 1, &[3, 0, 0], &c, held), Err(Refusal::Short));
}

#[test]
fn a_contact_from_five_names_sint_two_and_the_core_s_monitor_pages() {
    let c = world();
    let ok = contact(5 << 16, 0, 2, ROOT, ROOT + 4096);
    assert_eq!(check(4, 1, &ok, &c, held), Ok(()));
    let bad = [
        (contact(5 << 16, 1, 2, ROOT, ROOT), Refusal::Target),
        (contact(5 << 16, 0, 3, ROOT, ROOT), Refusal::Target),
        (contact(5 << 16, 0, 2, 0x2000, ROOT), Refusal::Page),
        (contact(5 << 16, 0, 2, ROOT, 0x2000), Refusal::Page),
        // A channel's page is the driver's, never the core's to name.
        (contact(5 << 16, 0, 2, RING, ROOT), Refusal::Page),
    ];
    for (m, why) in bad {
        assert_eq!(check(4, 1, &m, &c, held), Err(why));
    }
}

#[test]
fn a_contact_before_five_names_the_core_s_interrupt_page() {
    let c = world();
    let win8 = 2 << 16 | 4;
    let ok = contact(win8, 0, ROOT + 8192, ROOT, ROOT + 4096);
    assert_eq!(check(1, 1, &ok, &c, held), Ok(()));
    let theirs = contact(win8, 0, 0x5000, ROOT, ROOT + 4096);
    assert_eq!(check(1, 1, &theirs, &c, held), Err(Refusal::Page), "the kernel's page, or anyone's");
}

#[test]
fn a_gpadl_names_only_its_channel_s_held_frames() {
    let c = world();
    let p = RING_PFN;
    assert_eq!(check(1, 1, &gpadl_header(RELID, 1, 1, &[p, p + 1, p + 3]), &c, held), Ok(()));
    let refused = [
        (gpadl_header(RELID, 1, 1, &[p, 0x42]), Refusal::Page),
        (gpadl_header(RELID, 1, 1, &[p + PAGES]), Refusal::Page),
        (gpadl_header(RELID, 1, 1, &[u64::MAX]), Refusal::Page),
        // A frame whose address wraps onto a held one.
        (gpadl_header(RELID, 1, 1, &[p + (1 << 52)]), Refusal::Page),
        // The core's own pages are not the channel's.
        (gpadl_header(RELID, 1, 1, &[ROOT_PFN]), Refusal::Page),
        (gpadl_header(RELID, 1, 2, &[p]), Refusal::Kind),
        (gpadl_header(RELID, 1, 1, &[]), Refusal::Short),
    ];
    for (m, why) in refused {
        assert_eq!(check(1, 1, &m, &c, held), Err(why));
    }
    let mut ragged = gpadl_header(RELID, 1, 1, &[p]);
    ragged.push(0);
    assert_eq!(check(1, 1, &ragged, &c, held), Err(Refusal::Short));
}

#[test]
fn a_gpadl_is_only_for_a_channel_given_to_a_driver() {
    let mut c = world();
    let p = RING_PFN;
    // Never offered.
    assert_eq!(check(1, 1, &gpadl_header(9, 1, 1, &[p]), &c, held), Err(Refusal::Channel));
    // Offered, but given to no driver.
    c.take_host_message(&offer(6, SCSI, 0x2006));
    assert_eq!(check(1, 1, &gpadl_header(6, 1, 1, &[p]), &c, held), Err(Refusal::Channel));
    // Withdrawn after it was given.
    let mut gone = world();
    gone.take_host_message(&{
        let mut m = msg(2, 12);
        put32(&mut m, 8, RELID);
        m
    });
    assert_eq!(check(1, 1, &gpadl_header(RELID, 1, 1, &[p]), &gone, held), Err(Refusal::Channel));
}

#[test]
fn a_gpadl_body_is_held_to_the_channel_its_header_named() {
    let mut c = world();
    let p = RING_PFN;
    // A body before any header names no channel.
    assert_eq!(check(1, 1, &gpadl_body(7, &[p]), &c, held), Err(Refusal::Channel));
    assert_eq!(post(&mut c, &gpadl_header(RELID, 7, 1, &[p])), Ok(()));
    assert_eq!(check(1, 1, &gpadl_body(7, &[p + 2, p + 1]), &c, held), Ok(()));
    assert_eq!(check(1, 1, &gpadl_body(7, &[p + 2, 1]), &c, held), Err(Refusal::Page));
    assert_eq!(check(1, 1, &gpadl_body(7, &[ROOT_PFN]), &c, held), Err(Refusal::Page));
    assert_eq!(check(1, 1, &gpadl_body(8, &[p]), &c, held), Err(Refusal::Channel));
}

#[test]
fn a_live_gpadl_id_is_never_made_again() {
    let mut c = world();
    let p = RING_PFN;
    assert_eq!(post(&mut c, &gpadl_header(RELID, 7, 1, &[p])), Ok(()));
    assert_eq!(post(&mut c, &gpadl_header(RELID, 7, 1, &[p])), Err(Refusal::Channel));
    // The guest's teardown alone does not free it: the host's word does.
    let mut teardown = msg(11, 16);
    put32(&mut teardown, 8, RELID);
    put32(&mut teardown, 12, 7);
    assert_eq!(post(&mut c, &teardown), Ok(()));
    assert_eq!(post(&mut c, &gpadl_header(RELID, 7, 1, &[p])), Err(Refusal::Channel));
    let mut torndown = msg(12, 12);
    put32(&mut torndown, 8, 7);
    assert_eq!(c.take_host_message(&torndown), Retire::Settle(RECORD));
    assert_eq!(post(&mut c, &gpadl_header(RELID, 7, 1, &[p])), Ok(()));
}

#[test]
fn a_full_gpadl_table_refuses_the_next_header() {
    let mut c = world();
    for id in 0..64 {
        assert_eq!(post(&mut c, &gpadl_header(RELID, id, 1, &[RING_PFN])), Ok(()), "{id}");
    }
    assert_eq!(post(&mut c, &gpadl_header(RELID, 64, 1, &[RING_PFN])), Err(Refusal::Channel));
}

#[test]
fn a_channel_opens_on_vp_zero_with_its_own_gpadl() {
    let mut c = world();
    assert_eq!(check(1, 1, &open(RELID, 7, 0), &c, held), Err(Refusal::Channel), "no GPADL yet");
    assert_eq!(post(&mut c, &gpadl_header(RELID, 7, 1, &[RING_PFN])), Ok(()));
    assert_eq!(check(1, 1, &open(RELID, 7, 0), &c, held), Ok(()));
    assert_eq!(check(1, 1, &open(RELID, 7, 1), &c, held), Err(Refusal::Target));
    assert_eq!(check(1, 1, &open(RELID, 8, 0), &c, held), Err(Refusal::Channel));
    // Another channel's GPADL, and a channel given to no driver.
    c.take_host_message(&offer(6, SCSI, 0x2006));
    assert_eq!(check(1, 1, &open(6, 7, 0), &c, held), Err(Refusal::Channel));
    let mut never = world();
    never.take_host_message(&offer(6, SCSI, 0x2006));
    assert_eq!(check(1, 1, &open(6, 7, 0), &never, held), Err(Refusal::Channel));
}
