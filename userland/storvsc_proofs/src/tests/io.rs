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

//! Requests through the channel: the scan, reads and writes by page, the
//! commands with no data in band, the host's transfer limit; and the host
//! sending what the driver passes over, falling silent, or breaking its
//! ring.

use nonos_scsi::{read, write, Addr, Dir, Fault, Hba};

use super::util::{ops, seen, started, up, up_on, StorHba};
use crate::constants::layout::{DATA_OFF, PAGE};
use crate::constants::timing::COMMAND_MS;
use crate::constants::vstor::{INIT_ID, RESET_ID};
use crate::hba::next_id;
use crate::model::sim::Unit;
use crate::model::{Server, BASE_PFN};

const DATA_PFN: u64 = BASE_PFN + (DATA_OFF / PAGE) as u64;
const TUR: [u8; 6] = [0; 6];

fn lun0_listing(luns: Vec<u16>) -> Unit {
    let mut u = Unit::disk(2048, 512);
    u.luns = luns;
    u
}

#[test]
fn the_scan_finds_each_lun_and_passes_an_empty_target() {
    let u = up(vec![((1, 0), lun0_listing(vec![0, 5])), ((1, 5), Unit::disk(4096, 4096))]);
    assert_eq!(u.count(), 2);
    let at: Vec<Addr> = u.disks.iter().flatten().map(|d| d.at).collect();
    assert_eq!(at, [Addr { target: 1, lun: 0 }, Addr { target: 1, lun: 5 }]);
    // storvsp:610-661 then 666-737: target 0's REPORT LUNS is an empty
    // list, so LUN 0 alone is probed, and its INQUIRY says no device.
    let t0: Vec<u8> = seen(&u.hba).iter().filter(|s| s.op == 3 && s.packet[18] == 0).map(|s| s.packet[28]).collect();
    assert_eq!(t0, [0xA0, 0x12]);
}

#[test]
fn reads_and_writes_go_by_the_buffers_pages() {
    let mut u = up(vec![((0, 0), Unit::disk(2048, 512))]);
    let d = u.disks[0].unwrap();
    u.hba.buffer()[..1024].iter_mut().enumerate().for_each(|(i, b)| *b = i as u8 ^ 0xA5);
    write(&mut u.hba, &d, 7, 2).unwrap();
    let w = seen(&u.hba).last().unwrap().clone();
    assert_eq!((w.ty, w.packet[22], w.range), (9, 0, Some((1024, 0, vec![DATA_PFN]))));
    u.hba.buffer().fill(0);
    assert_eq!(read(&mut u.hba, &d, 7, 2), Ok(1024));
    assert!(u.hba.buffer()[..1024].iter().enumerate().all(|(i, b)| *b == i as u8 ^ 0xA5));
    let r = seen(&u.hba).last().unwrap().clone();
    assert_eq!((r.ty, r.packet[22]), (9, 1));
    let want = u.hba.host.server.units[&(0, 0)].block(8);
    assert_eq!(want[..512], u.hba.buffer()[512..1024]);
}

#[test]
fn a_whole_buffer_names_all_sixteen_pages() {
    let mut u = up(vec![((0, 0), Unit::disk(2048, 512))]);
    let d = u.disks[0].unwrap();
    assert_eq!(read(&mut u.hba, &d, 0, 128), Ok(65536));
    let pfns: Vec<u64> = (0..16).map(|i| DATA_PFN + i).collect();
    assert_eq!(seen(&u.hba).last().unwrap().range, Some((65536, 0, pfns)));
}

#[test]
fn a_command_with_no_data_goes_in_band() {
    let mut u = up(vec![((0, 0), Unit::disk(2048, 512))]);
    let at = Addr { target: 0, lun: 0 };
    assert_eq!(u.hba.exec(at, &TUR, Dir::None, 0).map(|o| o.status), Ok(0));
    let s = seen(&u.hba).last().unwrap();
    assert_eq!((s.ty, s.packet[22], s.range.is_none()), (6, 2, true));
}

#[test]
fn the_hosts_transfer_limit_bounds_the_buffer() {
    let mut s = Server::new(vec![((0, 0), Unit::disk(2048, 512))]);
    s.max_transfer = 8192;
    let mut u = up_on(s);
    assert_eq!(u.hba.buffer().len(), 8192);
    let d = u.disks[0].unwrap();
    assert!(read(&mut u.hba, &d, 0, 32).is_err(), "16 KiB is past it");
    assert_eq!(u.hba.exec(d.at, &[0x28, 0, 0, 0, 0, 0, 0, 0, 32, 0], Dir::In, 16384), Err(Fault::Failed));
    assert_eq!(read(&mut u.hba, &d, 0, 16), Ok(8192));
}

#[test]
fn ids_count_from_one_and_never_meet_the_starts_or_the_resets() {
    let mut u = up(vec![((0, 0), Unit::disk(2048, 512))]);
    let ids: Vec<u64> = seen(&u.hba).iter().filter(|s| s.op == 3).map(|s| s.txid).collect();
    assert_eq!(ids, (1..=ids.len() as u64).collect::<Vec<_>>());
    let at = u.disks[0].unwrap().at;
    u.hba.exec(at, &TUR, Dir::None, 0).unwrap();
    assert_eq!(seen(&u.hba).last().unwrap().txid, ids.len() as u64 + 1);
    assert_eq!((next_id(0), next_id(41), next_id(RESET_ID - 2)), (1, 42, RESET_ID - 1));
    assert_eq!((next_id(RESET_ID - 1), next_id(INIT_ID)), (1, 1), "wraps short of both");
}

#[test]
fn another_ids_completion_is_passed_over() {
    let mut u = up(vec![((0, 0), Unit::disk(2048, 512))]);
    u.hba.host.server.stale_first = true;
    let d = u.disks[0].unwrap();
    assert_eq!(read(&mut u.hba, &d, 3, 1), Ok(512));
    let want = u.hba.host.server.units[&(0, 0)].block(3);
    assert_eq!(u.hba.buffer()[..512], want[..]);
}

#[test]
fn the_hosts_own_packets_are_noted_and_passed_over() {
    for (op, changed) in [(11, true), (2, true), (12, false)] {
        let mut u = up(vec![((0, 0), Unit::disk(2048, 512))]);
        u.hba.host.server.unasked = Some(op);
        assert!(!u.hba.changed);
        u.hba.exec(Addr { target: 0, lun: 0 }, &TUR, Dir::None, 0).unwrap();
        assert_eq!(u.hba.changed, changed, "operation {op}");
    }
}

#[test]
fn a_packet_too_long_to_be_an_answer_is_skipped() {
    let mut u = up(vec![((0, 0), Unit::disk(2048, 512))]);
    u.hba.host.server.oversized = true;
    let d = u.disks[0].unwrap();
    assert_eq!(read(&mut u.hba, &d, 1, 1), Ok(512));
    assert!(!u.hba.dead);
}

#[test]
fn a_request_never_answered_resets_the_bus_and_the_next_goes_through() {
    let mut u = up(vec![((0, 0), Unit::disk(2048, 512))]);
    u.hba.host.server.silent = 1;
    let at = Addr { target: 0, lun: 0 };
    let t0 = u.hba.host.vmbus.now;
    assert_eq!(u.hba.exec(at, &TUR, Dir::None, 0), Err(Fault::Timeout));
    assert!(u.hba.host.vmbus.now - t0 >= COMMAND_MS);
    let last = seen(&u.hba).last().unwrap();
    assert_eq!((last.op, last.txid, last.ty), (6, RESET_ID, 6));
    assert_eq!(u.hba.host.server.resets, 1);
    assert!(!u.hba.dead);
    assert!(u.hba.exec(at, &TUR, Dir::None, 0).is_ok());
}

#[test]
fn a_host_that_answers_not_even_a_reset_is_given_up_on() {
    let mut u = up(vec![((0, 0), Unit::disk(2048, 512))]);
    u.hba.host.server.silent = 1;
    u.hba.host.server.silent_resets = true;
    let at = Addr { target: 0, lun: 0 };
    assert_eq!(u.hba.exec(at, &TUR, Dir::None, 0), Err(Fault::Timeout));
    assert!(u.hba.dead);
    let n = seen(&u.hba).len();
    assert_eq!(u.hba.exec(at, &TUR, Dir::None, 0), Err(Fault::Failed));
    assert_eq!(seen(&u.hba).len(), n, "nothing more is sent");
}

#[test]
fn a_broken_host_ring_kills_the_driver() {
    let mut h: StorHba = started(Server::new(vec![((0, 0), Unit::disk(2048, 512))]));
    // The host's ring's write index, past its data, and nothing more from
    // the host.
    use nonos_vmbus::Shared;
    h.rings.store32(32 * PAGE, 0xFFFF_0000);
    h.host.server.silent = 1;
    let t0 = h.host.vmbus.now;
    assert_eq!(h.exec(Addr { target: 0, lun: 0 }, &TUR, Dir::None, 0), Err(Fault::Failed));
    assert!(h.dead);
    assert!(h.host.vmbus.now - t0 < COMMAND_MS, "at once, not at the deadline");
    assert_eq!(ops(&h).last(), Some(&3), "no reset down a broken channel");
}

#[test]
fn a_command_that_cannot_be_sent_is_refused_before_the_ring() {
    let mut u = up(vec![((0, 0), Unit::disk(2048, 512))]);
    let n = seen(&u.hba).len();
    let at = Addr { target: 0, lun: 0 };
    assert_eq!(u.hba.exec(at, &[0u8; 17], Dir::None, 0), Err(Fault::Failed));
    assert_eq!(u.hba.exec(at, &[], Dir::None, 0), Err(Fault::Failed));
    assert_eq!(u.hba.exec(Addr { target: 0, lun: 256 }, &TUR, Dir::None, 0), Err(Fault::Failed));
    assert_eq!(u.hba.exec(at, &[0x28; 10], Dir::In, 65537), Err(Fault::Failed));
    assert_eq!(seen(&u.hba).len(), n);
}

#[test]
fn a_guest_ring_that_stays_full_times_out_and_the_driver_gives_up() {
    use nonos_vmbus::Shared;
    let mut h: StorHba = started(Server::new(vec![((0, 0), Unit::disk(2048, 512))]));
    // The host stops reading and its read index sits just past the guest's
    // write index: one byte short of the whole ring is unread.
    h.host.server.stalled = true;
    let write = h.rings.load32(0);
    h.rings.store32(4, (write + 8) % (31 * PAGE as u32));
    let n = seen(&h).len();
    let t0 = h.host.vmbus.now;
    assert_eq!(h.exec(Addr { target: 0, lun: 0 }, &TUR, Dir::None, 0), Err(Fault::Timeout));
    assert!(h.host.vmbus.now - t0 >= COMMAND_MS, "waited for room until the deadline");
    assert!(h.dead, "not even the reset found room");
    assert_eq!(seen(&h).len(), n);
}

#[test]
fn a_medium_error_comes_back_with_its_sense_and_nothing_moved() {
    // scsidisk:1012-1057: a CHECK CONDITION is SRB ERROR with sense and
    // nothing transferred; storvsp:1316 sets autosense with it.
    let mut u = up(vec![((0, 0), Unit::disk(2048, 512))]);
    let d = u.disks[0].unwrap();
    u.hba.host.server.units.get_mut(&(0, 0)).unwrap().fail_next = Some((0x3, 0x11, 0x00));
    let o = u.hba.exec(d.at, &[0x28, 0, 0, 0, 0, 4, 0, 0, 1, 0], Dir::In, 512).unwrap();
    assert_eq!((o.status, o.residual, o.sense[2] & 0x0F, o.sense[12]), (2, 512, 0x3, 0x11));
}

#[test]
fn a_command_to_an_absent_lun_on_a_present_target_is_no_target() {
    // storvsp:741-745: INVALID_LUN with CHECK and no sense.
    let mut u = up(vec![((0, 0), Unit::disk(2048, 512))]);
    let at = Addr { target: 0, lun: 3 };
    assert_eq!(u.hba.exec(at, &TUR, Dir::None, 0), Err(Fault::NoTarget));
}
