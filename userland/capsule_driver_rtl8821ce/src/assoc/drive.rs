// NONOS Operating System (AGPL-3.0-or-later)

//! The join loop: transmit, receive, resend.

use super::frame::handle_frame;
use super::outcome::{ended, out_of_time};
use super::radio::{Counters, Radio};
use super::report::{report, Report};
use super::resend::resend_after;
use super::send::{open, transmit};
use alloc::vec::Vec;
use nonos_wifi_core::mlme::Mlme;

pub(super) fn drive<R: Radio>(
    radio: &mut R,
    mut mlme: Mlme,
    beacon: &[u8],
    budget_ms: u64,
    now_ms: &mut dyn FnMut() -> u64,
) -> Report {
    let mut c = Counters::default();
    let start = now_ms();
    let mut quiet_since = start;
    // The one sequence counter of every frame the join sends, management and
    // non-QoS data alike (802.11-2020, 10.3.2.14.2).
    let mut seq: u16 = 0;
    // The last frame transmitted, and since when it has gone unanswered. A Wi-Fi
    // link loses frames, and the state machine sends each frame once, so without
    // this a single lost authentication, association or handshake frame hangs the
    // whole join. The pending frame is resent once it has been quiet for
    // `RETX_AFTER_MS`. It is cleared whenever the state advances without producing
    // a reply, because that means we are now waiting for the peer to send next
    // (the access point starts the four-way handshake), and resending our last
    // frame would only churn.
    let mut pending = open(radio, &mut mlme, beacon, &mut seq, &mut c);

    let mut frame = [0u8; 2048];
    let mut eth = Vec::new();
    loop {
        if let Some(outcome) = ended(&mlme) {
            return report(&mlme, outcome, c);
        }
        let now = now_ms();
        if now.wrapping_sub(start) >= budget_ms {
            break;
        }
        if !pending.is_empty() && now.wrapping_sub(quiet_since) >= resend_after(&pending) {
            transmit(radio, &mut pending, &mut seq, &mut c);
            quiet_since = now;
        }
        let Some(n) = radio.recv(&mut frame) else {
            continue;
        };
        c.recv += 1;
        let rx = &frame[..n];
        let before = mlme.state();
        let tx = handle_frame(&mut mlme, rx, &mut eth, &mut c);
        match tx {
            // Produced a reply: transmit it and make it the frame to retransmit.
            Some(mut mpdu) => {
                transmit(radio, &mut mpdu, &mut seq, &mut c);
                pending = mpdu;
                quiet_since = now;
            }
            // No reply, but the state advanced: we now wait for the peer, so stop
            // resending the previous frame.
            None if mlme.state() != before => {
                pending.clear();
                quiet_since = now;
            }
            None => {}
        }
    }
    report(&mlme, out_of_time(&mlme), c)
}
