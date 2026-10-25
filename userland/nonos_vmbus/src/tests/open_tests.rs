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

//! A channel opened and closed against the host model: the GPADL over every
//! page in header and bodies, the open naming the host's ring, and every
//! failure undone so the host keeps no page.

use std::vec;
use std::vec::Vec;

use crate::channel::{open, Layout, OpenError, ANSWER_MS};
use crate::mem::Buffer;
use crate::model::{answer, HostRing, Model, Script};
use crate::msg::{BODY_PFNS, HEADER_PFNS};
use crate::packet::{Header, PACKET_DATA_INBAND};
use crate::ring::PAGE;

const RELID: u32 = 5;
const GPADL: u32 = 0x1000;
const PFN: u64 = 0x8_0000;

fn pages(layout: Layout) -> Buffer {
    Buffer(vec![0xEE; layout.pages() * PAGE])
}

fn go(script: Script, layout: Layout) -> (Model, Buffer, Result<crate::Channel, OpenError>) {
    let mut h = Model::new(script);
    let mut m = pages(layout);
    let r = open(&mut h, &mut m, RELID, GPADL, PFN, layout, b"user");
    (h, m, r)
}

const SMALL: Layout = Layout {
    out_pages: 4,
    in_pages: 4,
};

#[test]
fn a_channel_opens_with_one_gpadl_over_every_page() {
    let (h, m, r) = go(Script::default(), SMALL);
    let c = r.unwrap();
    assert_eq!((c.relid, c.gpadl), (RELID, GPADL));
    assert_eq!(h.kinds(), [8, 5], "a header carries 8 pages; then the open");
    let pfns: Vec<u64> = (0..8).map(|i| PFN + i).collect();
    assert_eq!(h.gpadls, [(GPADL, pfns)]);
    assert_eq!(
        h.open,
        Some((RELID, GPADL, 4)),
        "the host's ring starts 4 pages in"
    );
    let open = &h.posted[1];
    assert_eq!(
        &open[12..16],
        &RELID.to_le_bytes(),
        "the open id is the relid, as Linux's"
    );
    assert_eq!(&open[28..32], b"user");
    // The rings were set empty, the rest of the pages untouched.
    assert_eq!(&m.0[..16], &[0u8; 16]);
    assert_eq!(&m.0[4 * PAGE..4 * PAGE + 16], &[0u8; 16]);
    assert_eq!(m.0[PAGE], 0xEE);
}

#[test]
fn a_large_gpadl_goes_in_a_header_and_numbered_bodies() {
    let big = Layout {
        out_pages: 32,
        in_pages: 64,
    };
    let (h, _, r) = go(Script::default(), big);
    r.unwrap();
    let bodies = (96 - HEADER_PFNS).div_ceil(BODY_PFNS);
    assert_eq!((HEADER_PFNS, BODY_PFNS), (26, 28));
    assert_eq!(h.kinds().len(), 1 + bodies + 1);
    for (n, b) in h.posted[1..=bodies].iter().enumerate() {
        assert_eq!(u32::from_le_bytes(b[0..4].try_into().unwrap()), 9);
        assert_eq!(
            u32::from_le_bytes(b[8..12].try_into().unwrap()),
            n as u32,
            "message number"
        );
        assert_eq!(&b[12..16], &GPADL.to_le_bytes());
    }
    let pfns: Vec<u64> = (0..96).map(|i| PFN + i).collect();
    assert_eq!(h.gpadls, [(GPADL, pfns)], "every page, in order");
}

#[test]
fn packets_flow_both_ways_once_open() {
    let (mut h, mut m, r) = go(Script::default(), SMALL);
    let c = r.unwrap();
    c.send(
        &mut h,
        &mut m,
        &Header::new(PACKET_DATA_INBAND, 1, 77),
        &[],
        &[b"hello"],
    )
    .unwrap();
    assert_eq!(h.signals, 1, "the host had read everything");
    let hout = HostRing {
        hdr: 0,
        len: (3 * PAGE) as u32,
    };
    let (_, _, txid, _, data) = hout.recv(&mut m).unwrap();
    assert_eq!((txid, &data[..5]), (77, &b"hello"[..]));
    let hin = HostRing {
        hdr: 4 * PAGE,
        len: (3 * PAGE) as u32,
    };
    hin.send(&mut m, 11, 77, b"done").unwrap();
    let mut buf = [0u8; 256];
    let (hdr, _) = c.recv(&mut h, &mut m, &mut buf).unwrap().unwrap();
    assert_eq!((hdr.ty, hdr.transaction_id), (11, 77));
    assert_eq!(c.recv(&mut h, &mut m, &mut buf), Ok(None));
}

#[test]
fn a_close_closes_then_takes_the_gpadl_back() {
    let (mut h, _, r) = go(Script::default(), SMALL);
    r.unwrap().close(&mut h).unwrap();
    assert_eq!(h.kinds(), [8, 5, 7, 11]);
    assert!(h.gpadls.is_empty() && h.open.is_none());
}

#[test]
fn a_close_the_host_never_confirms_is_said() {
    let (mut h, _, r) = go(
        Script {
            torndown: false,
            ..Script::default()
        },
        SMALL,
    );
    assert_eq!(r.unwrap().close(&mut h), Err(OpenError::Silent));
    assert!(h.now >= ANSWER_MS);
}

#[test]
fn a_refused_gpadl_opens_nothing() {
    let (h, _, r) = go(
        Script {
            gpadl_status: Some(0xC000_0001),
            ..Script::default()
        },
        SMALL,
    );
    assert_eq!(r, Err(OpenError::GpadlRefused(0xC000_0001)));
    assert_eq!(h.kinds(), [8], "no open, nothing to tear down");
}

#[test]
fn a_gpadl_never_answered_is_torn_down() {
    let (h, _, r) = go(
        Script {
            gpadl_status: None,
            ..Script::default()
        },
        SMALL,
    );
    assert_eq!(r, Err(OpenError::Silent));
    assert_eq!(h.kinds(), [8, 11]);
}

#[test]
fn a_refused_open_tears_its_gpadl_down() {
    let (h, _, r) = go(
        Script {
            open_status: Some(7),
            ..Script::default()
        },
        SMALL,
    );
    assert_eq!(r, Err(OpenError::OpenRefused(7)));
    assert_eq!(h.kinds(), [8, 5, 11]);
    assert!(h.gpadls.is_empty());
}

#[test]
fn an_open_never_answered_is_closed_and_torn_down() {
    let (h, _, r) = go(
        Script {
            open_status: None,
            ..Script::default()
        },
        SMALL,
    );
    assert_eq!(r, Err(OpenError::Silent));
    assert_eq!(h.kinds(), [8, 5, 7, 11]);
}

#[test]
fn a_busy_kernel_is_waited_for_and_a_refusal_is_not() {
    let (h, _, r) = go(
        Script {
            busy: 3,
            ..Script::default()
        },
        SMALL,
    );
    r.unwrap();
    assert!(h.now >= 30);
    let mut h = Model::new(Script::default());
    h.refuse = true;
    let r = open(&mut h, &mut pages(SMALL), RELID, GPADL, PFN, SMALL, &[]);
    assert_eq!(r, Err(OpenError::Refused));
}

#[test]
fn a_bad_layout_is_refused_before_anything_is_posted() {
    for layout in [
        Layout {
            out_pages: 1,
            in_pages: 4,
        },
        Layout {
            out_pages: 4,
            in_pages: 1,
        },
        Layout {
            out_pages: 300,
            in_pages: 300,
        },
    ] {
        let mut h = Model::new(Script::default());
        let mut m = Buffer(vec![0u8; 8 * PAGE]);
        assert_eq!(
            open(&mut h, &mut m, RELID, GPADL, PFN, layout, &[]),
            Err(OpenError::Layout)
        );
        assert!(h.posted.is_empty());
    }
    let mut h = Model::new(Script::default());
    let r = open(
        &mut h,
        &mut pages(SMALL),
        RELID,
        GPADL,
        PFN,
        SMALL,
        &[0u8; 121],
    );
    assert_eq!(r, Err(OpenError::Layout), "user data past 120 bytes");
}

#[test]
fn stale_answers_are_dropped_while_waiting() {
    let mut h = Model::new(Script::default());
    // Another GPADL's answer and an open of another channel come first.
    let mut stale = vec![0u8; 20];
    stale[0] = 10;
    stale[8..12].copy_from_slice(&RELID.to_le_bytes());
    stale[12..16].copy_from_slice(&(GPADL + 1).to_le_bytes());
    stale[16] = 1;
    h.answers.push_back(stale);
    let r = open(&mut h, &mut pages(SMALL), RELID, GPADL, PFN, SMALL, &[]);
    assert!(r.is_ok());
}

#[test]
fn the_smallest_rings_open() {
    let (_, _, r) = go(
        Script::default(),
        Layout {
            out_pages: 2,
            in_pages: 2,
        },
    );
    assert!(r.is_ok(), "a header page and a data page each way");
}

#[test]
fn user_data_of_120_bytes_fits() {
    let mut h = Model::new(Script::default());
    let r = open(
        &mut h,
        &mut pages(SMALL),
        RELID,
        GPADL,
        PFN,
        SMALL,
        &[7u8; 120],
    );
    assert!(r.is_ok());
    assert_eq!(&h.posted[1][28..148], &[7u8; 120]);
}

#[test]
fn a_kernel_busy_for_good_is_given_up_on() {
    let (h, _, r) = go(
        Script {
            busy: u32::MAX,
            ..Script::default()
        },
        SMALL,
    );
    assert_eq!(r, Err(OpenError::Silent));
    assert!(h.posted.is_empty() && h.now >= ANSWER_MS);
}

#[test]
fn another_channels_open_answer_is_not_this_ones() {
    let mut h = Model::new(Script::default());
    h.stale
        .push(answer(6, &[RELID + 1, RELID + 1, 0xC000_0001]));
    h.stale.push(answer(6, &[RELID, RELID + 1, 0xC000_0001]));
    let r = open(&mut h, &mut pages(SMALL), RELID, GPADL, PFN, SMALL, &[]);
    assert!(r.is_ok(), "{r:?}");
}

#[test]
fn another_gpadls_teardown_does_not_free_this_ones() {
    let (mut h, _, r) = go(
        Script {
            torndown: false,
            ..Script::default()
        },
        SMALL,
    );
    let c = r.unwrap();
    h.answers.push_back(answer(12, &[GPADL + 1]));
    assert_eq!(c.close(&mut h), Err(OpenError::Silent));
}
