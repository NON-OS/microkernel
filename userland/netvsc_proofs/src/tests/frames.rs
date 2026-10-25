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

//! Frames both ways: out through a send buffer section behind an RNDIS
//! header, in from the receive buffer; each receive completed so the host
//! has its ranges back; sections freed by the host's completions.

use super::util::{frame, netvsp, pump, up};
use crate::constants::timing::SEND_MS;
use crate::link::LinkError;

const PEER: [u8; 6] = [0x02, 0xAA, 0, 0, 0, 9];

#[test]
fn a_frame_goes_out_whole_from_the_host_assigned_address() {
    let (mut l, s) = up();
    let f = frame(PEER, s.host_mac, 60);
    l.send_frame(&f, SEND_MS).unwrap();
    l.service().unwrap();
    assert_eq!(netvsp(&l).wire.out.last(), Some(&f));
    assert!(netvsp(&l).violations.is_empty());
}

#[test]
fn frames_come_in_and_every_range_is_given_back() {
    let (mut l, s) = up();
    for len in [14, 60, 1514, 100, 64] {
        l.host.netvsp.inbound.push_back(frame(s.host_mac, PEER, len));
    }
    pump(&mut l);
    let got: Vec<usize> = l.frames.drain(..).map(|f| f.len()).collect();
    assert_eq!(got, [14, 60, 1514, 100, 64]);
    assert!(netvsp(&l).violations.is_empty(), "{:?}", netvsp(&l).violations);
    // Every receive completed: the host can fill every range again.
    for _ in 0..200 {
        l.host.netvsp.inbound.push_back(frame(s.host_mac, PEER, 60));
        pump(&mut l);
        l.frames.clear();
    }
    assert!(netvsp(&l).violations.is_empty());
}

#[test]
fn a_frame_out_of_ethernet_bounds_is_passed_over() {
    let (mut l, s) = up();
    l.host.netvsp.inbound.push_back(frame(s.host_mac, PEER, 13));
    l.host.netvsp.inbound.push_back(frame(s.host_mac, PEER, 1515));
    l.host.netvsp.inbound.push_back(frame(s.host_mac, PEER, 42));
    pump(&mut l);
    assert_eq!(l.frames.iter().map(Vec::len).collect::<Vec<_>>(), [42]);
}

#[test]
fn a_range_past_the_receive_buffer_is_passed_over_and_still_completed() {
    let (mut l, s) = up();
    l.host.netvsp.bad_range = true;
    l.host.netvsp.inbound.push_back(frame(s.host_mac, PEER, 60));
    pump(&mut l);
    assert!(l.frames.is_empty());
    l.host.netvsp.bad_range = false;
    for _ in 0..200 {
        l.host.netvsp.inbound.push_back(frame(s.host_mac, PEER, 60));
        pump(&mut l);
        l.frames.clear();
    }
    assert!(netvsp(&l).violations.is_empty(), "the range was given back");
}

#[test]
fn sections_held_by_the_host_run_out_and_a_send_waits_its_bound() {
    let (mut l, s) = up();
    l.host.netvsp.withhold_tx = true;
    let sections = l.bufs.sections;
    for _ in 0..sections {
        l.send_frame(&frame(PEER, s.host_mac, 60), SEND_MS).unwrap();
    }
    let t0 = l.host.vmbus.now;
    assert_eq!(l.send_frame(&frame(PEER, s.host_mac, 60), SEND_MS), Err(LinkError::Busy));
    assert!(l.host.vmbus.now - t0 >= SEND_MS);
}

#[test]
fn a_frame_too_long_for_a_section_is_refused_before_the_ring() {
    let (mut l, s) = up();
    let n = netvsp(&l).wire.out.len();
    assert_eq!(l.send_frame(&frame(PEER, s.host_mac, 6144), SEND_MS), Err(LinkError::TooBig));
    assert_eq!(netvsp(&l).wire.out.len(), n);
}

#[test]
fn the_hosts_word_on_the_link_is_kept() {
    let (mut l, _) = up();
    assert_eq!(l.link, None);
    l.host.netvsp.indicate_link(false);
    pump(&mut l);
    assert_eq!(l.link, Some(false));
    l.host.netvsp.indicate_link(true);
    pump(&mut l);
    assert_eq!(l.link, Some(true));
}

#[test]
fn a_broken_host_ring_kills_the_link() {
    use nonos_vmbus::Shared;
    let (mut l, _) = up();
    l.rings.store32(32 * 4096, 0xFFFF_0000);
    assert_eq!(l.service(), Err(LinkError::Broken));
    assert!(l.dead);
    assert_eq!(l.send_frame(&[0u8; 60], SEND_MS), Err(LinkError::Broken));
}

#[test]
fn every_receive_is_given_back_so_frames_never_stop() {
    // 145 ranges in the receive buffer: past them, frames keep coming only
    // because each receive was completed (netvsp:5804-5817).
    let (mut l, s) = up();
    for i in 0..400u32 {
        l.host.netvsp.inbound.push_back(frame(s.host_mac, PEER, 60 + (i % 7) as usize));
        pump(&mut l);
        assert_eq!(l.frames.len(), 1, "frame {i} did not arrive");
        l.frames.clear();
    }
}

#[test]
fn a_section_the_host_completed_is_used_again() {
    let (mut l, s) = up();
    let n = 3 * l.bufs.sections;
    for i in 0..n {
        let mut f = frame(PEER, s.host_mac, 60);
        f[20] = i as u8;
        l.send_frame(&f, SEND_MS).unwrap();
        pump(&mut l);
    }
    let out = &netvsp(&l).wire.out;
    assert_eq!(out.len(), n, "every frame went out");
    assert!(out.iter().enumerate().all(|(i, f)| f[20] == i as u8), "each in order, whole");
}

#[test]
fn frames_in_flight_together_each_keep_their_own_section() {
    let (mut l, s) = up();
    l.host.netvsp.withhold_tx = true;
    for i in 0..l.bufs.sections {
        let mut f = frame(PEER, s.host_mac, 60 + i);
        f[20] = i as u8;
        l.send_frame(&f, SEND_MS).unwrap();
    }
    pump(&mut l);
    let tx = &netvsp(&l).tx;
    assert_eq!(tx.len(), l.bufs.sections);
    assert!(tx.iter().enumerate().all(|(i, f)| f[20] == i as u8 && f.len() == 60 + i));
    // Request ids: each section's own, above every protocol message's.
    let ids: Vec<u64> = netvsp(&l).ids.iter().rev().take(l.bufs.sections).copied().collect();
    assert!(ids.iter().all(|id| *id >= 1 << 48));
}

#[test]
fn frames_past_the_queue_are_dropped_and_counted() {
    let (mut l, s) = up();
    for _ in 0..70 {
        l.host.netvsp.inbound.push_back(frame(s.host_mac, PEER, 60));
    }
    for _ in 0..30 {
        use nonos_vmbus::Host;
        l.host.idle();
        l.service().unwrap();
    }
    assert_eq!((l.frames.len(), l.dropped), (64, 6));
}

/// The guest's ring made to look full: its read index just past its write.
fn fill(l: &mut super::util::NetLink) -> u32 {
    use nonos_vmbus::Shared;
    let w = l.rings.load32(0);
    l.rings.store32(4, (w + 8) % (31 * 4096));
    w
}

fn unfill(l: &mut super::util::NetLink, w: u32) {
    use nonos_vmbus::Shared;
    l.rings.store32(4, w);
}

#[test]
fn a_ring_that_stays_full_fails_a_send_at_its_bound_and_frees_its_section() {
    let (mut l, s) = up();
    l.host.netvsp.stalled = true;
    let w = fill(&mut l);
    let t0 = l.host.vmbus.now;
    assert_eq!(l.send_frame(&frame(PEER, s.host_mac, 60), SEND_MS), Err(LinkError::Silent));
    assert!(l.host.vmbus.now - t0 >= SEND_MS);
    unfill(&mut l, w);
    l.host.netvsp.stalled = false;
    // Every section free again: as many in flight as there are sections.
    l.host.netvsp.withhold_tx = true;
    for _ in 0..l.bufs.sections {
        l.send_frame(&frame(PEER, s.host_mac, 60), SEND_MS).unwrap();
    }
}

#[test]
fn a_ring_that_stays_full_fails_a_protocol_message_at_its_bound() {
    use crate::constants::timing::INIT_MS;
    let (mut l, _) = up();
    l.host.netvsp.stalled = true;
    let w = fill(&mut l);
    let t0 = l.host.vmbus.now;
    assert_eq!(l.call(&crate::nvsp::ndis_config(), INIT_MS), Err(LinkError::Silent));
    let waited = l.host.vmbus.now - t0;
    assert!((INIT_MS..INIT_MS * 2).contains(&waited), "{waited}");
    unfill(&mut l, w);
}

#[test]
fn every_section_is_freed_by_its_own_completion() {
    let (mut l, s) = up();
    let n = l.bufs.sections;
    for round in 0..3 {
        l.host.netvsp.withhold_tx = true;
        for i in 0..n {
            l.send_frame(&frame(PEER, s.host_mac, 60 + i), SEND_MS).unwrap_or_else(|e| panic!("round {round}, {i}: {e:?}"));
        }
        l.host.netvsp.release_tx();
        pump(&mut l);
    }
}

#[test]
fn a_failed_send_frees_the_section_it_had_taken_whichever_it_was() {
    let (mut l, s) = up();
    // Section 0 held by the host, so the failed send takes section 1.
    l.host.netvsp.withhold_tx = true;
    l.send_frame(&frame(PEER, s.host_mac, 60), SEND_MS).unwrap();
    pump(&mut l);
    l.host.netvsp.stalled = true;
    let w = fill(&mut l);
    assert_eq!(l.send_frame(&frame(PEER, s.host_mac, 60), SEND_MS), Err(LinkError::Silent));
    unfill(&mut l, w);
    l.host.netvsp.stalled = false;
    l.host.netvsp.release_tx();
    pump(&mut l);
    l.host.netvsp.withhold_tx = true;
    for _ in 0..l.bufs.sections {
        l.send_frame(&frame(PEER, s.host_mac, 60), SEND_MS).unwrap();
    }
}

#[test]
fn a_message_exactly_a_section_long_fits() {
    use crate::constants::nvsp::CONTROL_CHANNEL;
    let (mut l, _) = up();
    let section = l.bufs.section;
    assert_eq!(l.send_rndis(CONTROL_CHANNEL, &[&vec![0u8; section]], SEND_MS), Ok(()));
    assert_eq!(l.send_rndis(CONTROL_CHANNEL, &[&vec![0u8; section + 1]], SEND_MS), Err(LinkError::TooBig));
}

#[test]
fn a_range_ending_at_the_buffers_very_end_is_read() {
    use crate::constants::rndis::{OID_802_3_PERMANENT_ADDRESS, STATUS_SUCCESS};
    use crate::constants::timing::INIT_MS;
    let (mut l, s) = up();
    l.host.netvsp.at_end = true;
    let (status, mac) = l.control(&crate::rndis::query(9, OID_802_3_PERMANENT_ADDRESS), 9, INIT_MS).unwrap();
    assert_eq!((status, &mac[..]), (STATUS_SUCCESS, &s.host_mac[..]));
}
