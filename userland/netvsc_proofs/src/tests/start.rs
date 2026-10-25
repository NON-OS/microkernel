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

//! The start in netvsp's order: INIT, the NDIS configuration from version 2,
//! the NDIS version, the two buffers, then RNDIS; each version fallback,
//! and what the driver learns.

use super::util::{host, netvsp, opened, start, up};
use crate::constants::layout::SEND_BYTES;
use crate::constants::timing::INIT_MS;

#[test]
fn the_start_goes_in_netvsps_order_on_the_newest_version() {
    let (l, s) = up();
    assert_eq!(s.version, 0x60001);
    let n = netvsp(&l);
    // INIT, NDIS config, NDIS version, receive buffer, send buffer, then
    // RNDIS on SEND_RNDIS_PACKET.
    assert_eq!(n.nvsp_seen, [1, 125, 100, 101, 104, 107, 107, 107, 107]);
    // INITIALIZE, the address, the filter, the link.
    assert_eq!(n.rndis_seen, [(2, 0), (4, 0x0101_0101), (5, 0x0001_010E), (4, 0x0001_0114)]);
    assert_eq!(s.host_mac, n.mac);
    assert!(s.link_up);
    assert_eq!(n.filter, 0x1 | 0x4 | 0x8, "directed, all multicast, broadcast");
    // netvsp:5285: 6144-byte sections; 256 KiB holds 42.
    assert_eq!((l.bufs.section, l.bufs.sections), (6144, SEND_BYTES as usize / 6144));
}

#[test]
fn an_older_host_is_spoken_to_in_its_version_with_its_ndis() {
    for (only, ndis, config) in [(0x40000, (6, 1), true), (0x50000, (6, 30), true), (0x2, (6, 1), false)] {
        let (g, mut n) = host();
        n.versions = vec![only];
        let mut l = opened(g, n);
        assert_eq!(start(&mut l).map(|s| s.version), Ok(only));
        let n = netvsp(&l);
        assert!(n.violations.is_empty(), "{only:#x}: {:?}", n.violations);
        // NDIS config from version 2 alone (netvsp:5134, 5234-5240).
        assert_eq!(n.nvsp_seen.contains(&125), config, "{only:#x}");
        let tried = SUPPORTED_NEWEST_FIRST.iter().position(|v| *v == only).unwrap() + 1;
        assert_eq!(n.nvsp_seen.iter().filter(|t| **t == 1).count(), tried);
        assert_eq!(n.ndis_version, Some(ndis), "{only:#x}");
    }
}

const SUPPORTED_NEWEST_FIRST: [u32; 6] = [0x60001, 0x60000, 0x50000, 0x40000, 0x30002, 0x2];

#[test]
fn a_host_with_no_version_in_common_is_left_before_anything_else() {
    let (g, mut n) = host();
    n.versions = vec![0x70000];
    let mut l = opened(g, n);
    assert_eq!(start(&mut l), Err("no protocol version in common"));
    assert_eq!(netvsp(&l).nvsp_seen, [1, 1, 1, 1, 1, 1]);
}

#[test]
fn a_silent_host_fails_the_start_in_bounded_time() {
    let (g, mut n) = host();
    n.mute = true;
    let mut l = opened(g, n);
    let t0 = l.host.vmbus.now;
    assert_eq!(start(&mut l), Err("host silent at RNDIS INITIALIZE"));
    let waited = l.host.vmbus.now - t0;
    assert!((INIT_MS..INIT_MS * 2).contains(&waited), "{waited}");
}

#[test]
fn a_host_that_never_completes_init_fails_the_start_at_its_bound() {
    let (g, mut n) = host();
    n.mute_nvsp = true;
    let mut l = opened(g, n);
    let t0 = l.host.vmbus.now;
    assert_eq!(start(&mut l), Err("host silent at INIT"));
    let waited = l.host.vmbus.now - t0;
    assert!((INIT_MS..INIT_MS * 2).contains(&waited), "{waited}");
}

#[test]
fn protocol_messages_count_up_and_rndis_goes_in_its_sections_id() {
    // A protocol message's id counts up from 1; an RNDIS message's is its
    // send buffer section's, above all of them, and used again only once the
    // host completed the last (frames_in_flight_together... keeps them apart
    // while several are out).
    let (l, _) = up();
    let ids = &netvsp(&l).ids;
    assert_eq!(&ids[..5], &[1, 2, 3, 4, 5]);
    assert!(ids[5..].iter().all(|id| *id == 1 << 48), "{ids:?}");
}

#[test]
fn receive_sections_that_fill_the_buffer_exactly_are_taken_and_past_it_refused() {
    use crate::constants::layout::RECV_BYTES;
    let (g, mut n) = host();
    n.recv_section = Some((4096, (RECV_BYTES / 4096) as u32));
    let mut l = opened(g, n);
    assert!(start(&mut l).is_ok(), "64 sections of a page: the whole buffer");
    let (g, mut n) = host();
    n.recv_section = Some((4096, (RECV_BYTES / 4096) as u32 + 1));
    let mut l = opened(g, n);
    assert_eq!(start(&mut l), Err("receive sections past the buffer"));
}

#[test]
fn a_host_that_says_the_link_is_down_is_believed() {
    let (g, mut n) = host();
    n.media_disconnected = true;
    let mut l = opened(g, n);
    assert!(!start(&mut l).unwrap().link_up);
}

#[test]
fn the_three_gpadls_are_the_rings_the_receive_and_the_send_buffers() {
    use crate::model::{RECV_PFN, RINGS_PFN, SEND_PFN};
    let (l, _) = up();
    let g: Vec<(u32, u64, usize)> = l.host.vmbus.gpadls.iter().map(|(id, p)| (*id, p[0], p.len())).collect();
    assert_eq!(g, [(0x4E45_0011, RINGS_PFN, 64), (0x4E45_1011, RECV_PFN, 64), (0x4E45_2011, SEND_PFN, 64)]);
    use crate::constants::channel::{gpadl_recv, gpadl_rings, gpadl_send};
    assert_eq!((gpadl_rings(0x1_2345), gpadl_recv(0xFFF), gpadl_send(0x1000)), (0x4E45_0345, 0x4E45_1FFF, 0x4E45_2000));
}
