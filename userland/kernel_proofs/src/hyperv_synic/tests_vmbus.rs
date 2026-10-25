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

//! What may be posted to the host. The core: the bus's messages alone,
//! long enough, aimed at VP 0, its contact naming only its own pages on the
//! root. A channel's driver: its own channel's open, close and GPADLs
//! alone, the frames held on its record, a GPADL's body tied to its
//! header's channel, a live GPADL id never made again, an open only with
//! the channel's own GPADL.

use super::channels::{Channels, Retire, TargetError};
use super::vmbus_out::{check, check_channel, Refusal};

/// The core's pages, on the root.
const ROOT: u64 = 0x10_0000;
/// The channel driver's pages, on its record.
const RING: u64 = 0x80_0000;
const PAGES: u64 = 4;
pub(super) const RECORD: u64 = 0x1_0000_00AA;
pub(super) const RELID: u32 = 5;
pub(super) const SCSI: [u8; 16] = [
    0xd9, 0x63, 0x61, 0xba, 0xa1, 0x04, 0x29, 0x4d, 0xb6, 0x05, 0x72, 0xe2, 0xff, 0xb1, 0xdc, 0x7f,
];

fn within(base: u64, gpa: u64) -> bool {
    (base..base + PAGES * 4096).contains(&gpa) && gpa.is_multiple_of(4096)
}

fn root_held(gpa: u64) -> bool {
    within(ROOT, gpa)
}

fn ring_held(gpa: u64) -> bool {
    within(RING, gpa)
}

pub(super) fn msg(kind: u32, len: usize) -> Vec<u8> {
    let mut m = vec![0u8; len];
    m[0..4].copy_from_slice(&kind.to_le_bytes());
    m
}

pub(super) fn put32(m: &mut [u8], at: usize, v: u32) {
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

/// The bus up on connection 4, and a SCSI channel offered and given to
/// `RECORD`.
pub(super) fn world() -> Channels {
    let mut c = Channels::new();
    c.take_core_message(4, &msg(3, 8));
    assert_eq!(c.take_host_message(&offer(RELID, SCSI, 0x2005)).1, Retire::None);
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

pub(super) fn open(relid: u32, gpadl: u32, vcpu: u32) -> Vec<u8> {
    let mut m = msg(5, 148);
    put32(&mut m, 8, relid);
    put32(&mut m, 16, gpadl);
    put32(&mut m, 20, vcpu);
    m
}

pub(super) fn close(relid: u32) -> Vec<u8> {
    let mut m = msg(7, 12);
    put32(&mut m, 8, relid);
    m
}

fn teardown(relid: u32, id: u32) -> Vec<u8> {
    let mut m = msg(11, 16);
    put32(&mut m, 8, relid);
    put32(&mut m, 12, id);
    m
}

/// The driver's post as the kernel makes it: checked, then noted once the
/// host took it.
pub(super) fn post(c: &mut Channels, m: &[u8]) -> Result<(), Refusal> {
    check_channel(1, m, c, RECORD, ring_held)?;
    c.take_driver_message(m);
    Ok(())
}

fn refused(c: &Channels, m: &[u8]) -> Refusal {
    check_channel(1, m, c, RECORD, ring_held).expect_err("refused")
}

const RING_PFN: u64 = RING / 4096;
const ROOT_PFN: u64 = ROOT / 4096;

#[test]
fn the_core_posts_the_bus_s_messages_alone() {
    let m = msg(3, 8);
    assert_eq!(check(1, 1, &m, root_held), Ok(()));
    assert_eq!(check(4, 1, &m, root_held), Ok(()));
    assert_eq!(check(2, 1, &m, root_held), Err(Refusal::Route));
    assert_eq!(check(1, 2, &m, root_held), Err(Refusal::Route));
    for (kind, len) in [(3, 8), (16, 8), (13, 12)] {
        assert_eq!(check(1, 1, &msg(kind, len), root_held), Ok(()), "kind {kind}");
    }
    // A channel's messages are its driver's, never the core's; and the
    // host's own, hv_sock, modify-channel and anything unknown are no one's.
    for kind in [5, 7, 8, 9, 11, 0, 1, 2, 4, 6, 10, 12, 15, 17, 18, 21, 22, 99, u32::MAX] {
        assert_eq!(check(1, 1, &msg(kind, 148), root_held), Err(Refusal::Kind), "kind {kind}");
    }
    for (kind, len) in [(3, 8), (16, 8), (13, 12), (14, 40)] {
        assert_eq!(check(1, 1, &msg(kind, len - 1), root_held), Err(Refusal::Short), "{kind}");
    }
    assert_eq!(check(1, 1, &[3, 0, 0], root_held), Err(Refusal::Short));
}

#[test]
fn a_contact_from_five_names_sint_two_and_the_core_s_monitor_pages() {
    assert_eq!(check(4, 1, &contact(5 << 16, 0, 2, ROOT, ROOT + 4096), root_held), Ok(()));
    let bad = [
        (contact(5 << 16, 1, 2, ROOT, ROOT), Refusal::Target),
        (contact(5 << 16, 0, 3, ROOT, ROOT), Refusal::Target),
        (contact(5 << 16, 0, 2, 0x2000, ROOT), Refusal::Page),
        (contact(5 << 16, 0, 2, ROOT, 0x2000), Refusal::Page),
        // A channel's page is its driver's, never the core's to name.
        (contact(5 << 16, 0, 2, RING, ROOT), Refusal::Page),
    ];
    for (m, why) in bad {
        assert_eq!(check(4, 1, &m, root_held), Err(why));
    }
}

#[test]
fn a_contact_before_five_names_the_core_s_interrupt_page() {
    let win8 = 2 << 16 | 4;
    assert_eq!(check(1, 1, &contact(win8, 0, ROOT + 8192, ROOT, ROOT + 4096), root_held), Ok(()));
    let theirs = contact(win8, 0, 0x5000, ROOT, ROOT + 4096);
    assert_eq!(
        check(1, 1, &theirs, root_held),
        Err(Refusal::Page),
        "the kernel's page, or anyone's"
    );
}

#[test]
fn a_driver_posts_its_channel_s_messages_alone() {
    let c = world();
    assert_eq!(check_channel(2, &close(RELID), &c, RECORD, ring_held), Err(Refusal::Route));
    assert_eq!(check_channel(1, &close(RELID), &c, RECORD, ring_held), Ok(()));
    // The bus's messages are the core's; the host's are no one's.
    for kind in [3, 13, 14, 16, 0, 1, 2, 4, 6, 10, 12, 15, 17, 99] {
        assert_eq!(refused(&c, &msg(kind, 148)), Refusal::Kind, "kind {kind}");
    }
    for (kind, len) in [(7, 12), (5, 28), (8, 28), (9, 16), (11, 16)] {
        assert_eq!(refused(&c, &msg(kind, len - 1)), Refusal::Short, "kind {kind}");
    }
    // Another channel, a record that names no channel, a bus not up.
    assert_eq!(refused(&c, &close(6)), Refusal::Channel(TargetError::Channel));
    assert_eq!(
        check_channel(1, &close(RELID), &c, RECORD + 1, ring_held),
        Err(Refusal::Channel(TargetError::Gone))
    );
    let mut down = Channels::new();
    down.take_host_message(&offer(RELID, SCSI, 1));
    down.bind(RELID, RECORD).unwrap();
    assert_eq!(refused(&down, &close(RELID)), Refusal::Channel(TargetError::NoBus));
}

#[test]
fn a_gpadl_names_only_frames_held_on_its_record() {
    let c = world();
    let p = RING_PFN;
    assert_eq!(
        check_channel(1, &gpadl_header(RELID, 1, 1, &[p, p + 1, p + 3]), &c, RECORD, ring_held),
        Ok(())
    );
    let bad = [
        (gpadl_header(RELID, 1, 1, &[p, 0x42]), Refusal::Page),
        (gpadl_header(RELID, 1, 1, &[p + PAGES]), Refusal::Page),
        (gpadl_header(RELID, 1, 1, &[u64::MAX]), Refusal::Page),
        // A frame whose address wraps onto a held one.
        (gpadl_header(RELID, 1, 1, &[p + (1 << 52)]), Refusal::Page),
        // The core's pages are not the channel's.
        (gpadl_header(RELID, 1, 1, &[ROOT_PFN]), Refusal::Page),
        (gpadl_header(RELID, 1, 2, &[p]), Refusal::Kind),
        (gpadl_header(RELID, 1, 1, &[]), Refusal::Short),
        (gpadl_header(6, 1, 1, &[p]), Refusal::Channel(TargetError::Channel)),
    ];
    for (m, why) in bad {
        assert_eq!(refused(&c, &m), why);
    }
    let mut ragged = gpadl_header(RELID, 1, 1, &[p]);
    ragged.push(0);
    assert_eq!(refused(&c, &ragged), Refusal::Short);
}

#[test]
fn a_gpadl_body_is_held_to_the_channel_its_header_named() {
    let mut c = world();
    let p = RING_PFN;
    assert_eq!(refused(&c, &gpadl_body(7, &[p])), Refusal::Channel(TargetError::Channel));
    assert_eq!(post(&mut c, &gpadl_header(RELID, 7, 1, &[p])), Ok(()));
    assert_eq!(check_channel(1, &gpadl_body(7, &[p + 2, p + 1]), &c, RECORD, ring_held), Ok(()));
    assert_eq!(refused(&c, &gpadl_body(7, &[p + 2, 1])), Refusal::Page);
    assert_eq!(refused(&c, &gpadl_body(7, &[ROOT_PFN])), Refusal::Page);
    assert_eq!(refused(&c, &gpadl_body(8, &[p])), Refusal::Channel(TargetError::Channel));
    // Another channel's GPADL is not this driver's to continue.
    let mut two = world();
    two.take_host_message(&offer(6, SCSI, 0x2006));
    two.bind(6, RECORD + 1).unwrap();
    check_channel(1, &gpadl_header(6, 9, 1, &[p]), &two, RECORD + 1, ring_held).unwrap();
    two.take_driver_message(&gpadl_header(6, 9, 1, &[p]));
    assert_eq!(refused(&two, &gpadl_body(9, &[p])), Refusal::Channel(TargetError::Channel));
}

#[test]
fn a_live_gpadl_id_is_never_made_again() {
    let mut c = world();
    let p = RING_PFN;
    assert_eq!(post(&mut c, &gpadl_header(RELID, 7, 1, &[p])), Ok(()));
    assert_eq!(
        post(&mut c, &gpadl_header(RELID, 7, 1, &[p])),
        Err(Refusal::Channel(TargetError::Reused))
    );
    // The guest's teardown alone does not free it: the host's word does.
    assert_eq!(post(&mut c, &teardown(RELID, 7)), Ok(()));
    assert_eq!(
        post(&mut c, &gpadl_header(RELID, 7, 1, &[p])),
        Err(Refusal::Channel(TargetError::Reused))
    );
    let mut gone = msg(12, 12);
    put32(&mut gone, 8, 7);
    assert_eq!(c.take_host_message(&gone).1, Retire::Settle(RECORD));
    assert_eq!(post(&mut c, &gpadl_header(RELID, 7, 1, &[p])), Ok(()));
}

#[test]
fn a_full_gpadl_table_refuses_the_next_header() {
    let mut c = world();
    for id in 0..64 {
        assert_eq!(post(&mut c, &gpadl_header(RELID, id, 1, &[RING_PFN])), Ok(()), "{id}");
    }
    assert_eq!(
        post(&mut c, &gpadl_header(RELID, 64, 1, &[RING_PFN])),
        Err(Refusal::Channel(TargetError::Full))
    );
}

#[test]
fn a_channel_opens_on_vp_zero_with_its_own_gpadl_and_tears_down_its_own() {
    let mut c = world();
    assert_eq!(
        refused(&c, &open(RELID, 7, 0)),
        Refusal::Channel(TargetError::Channel),
        "no GPADL yet"
    );
    assert_eq!(post(&mut c, &gpadl_header(RELID, 7, 1, &[RING_PFN])), Ok(()));
    assert_eq!(check_channel(1, &open(RELID, 7, 0), &c, RECORD, ring_held), Ok(()));
    assert_eq!(refused(&c, &open(RELID, 7, 1)), Refusal::Target);
    assert_eq!(refused(&c, &open(RELID, 8, 0)), Refusal::Channel(TargetError::Channel));
    assert_eq!(refused(&c, &open(6, 7, 0)), Refusal::Channel(TargetError::Channel));
    assert_eq!(refused(&c, &teardown(RELID, 8)), Refusal::Channel(TargetError::Channel));
    assert_eq!(refused(&c, &teardown(6, 7)), Refusal::Channel(TargetError::Channel));
    assert_eq!(check_channel(1, &teardown(RELID, 7), &c, RECORD, ring_held), Ok(()));
}

#[test]
fn a_releasing_channel_takes_no_post_until_the_host_tears_down() {
    let mut c = world();
    post(&mut c, &gpadl_header(RELID, 7, 1, &[RING_PFN])).unwrap();
    assert!(c.release(RECORD));
    assert_eq!(refused(&c, &close(RELID)), Refusal::Channel(TargetError::Releasing));
    let mut gone = msg(12, 12);
    put32(&mut gone, 8, 7);
    assert_eq!(c.take_host_message(&gone).1, Retire::Settle(RECORD));
    assert_eq!(check_channel(1, &close(RELID), &c, RECORD, ring_held), Ok(()));
}
