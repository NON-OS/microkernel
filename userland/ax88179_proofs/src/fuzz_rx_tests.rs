// NONOS Operating System (AGPL-3.0-or-later)
//! Random transfers, most with a trailer that points inside them so the
//! table is walked: parsed as a model written here parses them, and every
//! frame handed up is the transfer's own bytes at the place the model says.

use nonos_usbnet::Nic;

use crate::ax::link::SPANS_MAX;
use crate::ax::rx_header::parse;
use crate::harness::bound;
use crate::rx_model::{model, transfer};

#[test]
fn random_transfers_parse_as_the_model_and_never_past_their_table() {
    let mut seed = 0x2545_F491_4F6C_DD1D;
    for _ in 0..100_000 {
        let t = transfer(&mut seed);
        let mut got = vec![];
        let tally = parse(&t, &mut got, SPANS_MAX);
        let spans: Vec<(usize, usize)> =
            got.iter().map(|s| (s.at as usize, s.len as usize)).collect();
        assert_eq!(spans, model(&t), "{} bytes", t.len());
        let table = t.len().checked_sub(2).map_or(0, |e| u16::from_le_bytes([t[e], t[e + 1]]));
        for (at, len) in &spans {
            assert!(*at >= 2 && at + len <= table as usize && (14..=1514).contains(len));
        }
        assert!(!tally.lie || spans.is_empty());
    }
}

#[test]
fn random_transfers_through_the_driver_hand_up_only_their_own_bytes() {
    let (bus, mut ax, _) = bound();
    let mut seed = 0x9E37_79B9_7F4A_7C15;
    let mut out = [0u8; 1514];
    for _ in 0..20_000 {
        let t = transfer(&mut seed);
        bus.0.borrow_mut().bulk_in.push_back(Ok(Some(t.clone())));
        for (at, len) in model(&t) {
            assert_eq!(ax.recv(&mut out), Ok(Some(len)));
            assert_eq!(out[..len], t[at..at + len]);
        }
        assert_eq!(ax.recv(&mut out), Ok(None), "{} bytes: more than the model", t.len());
    }
}
