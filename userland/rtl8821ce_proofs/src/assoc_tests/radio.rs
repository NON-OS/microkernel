// NONOS Operating System (AGPL-3.0-or-later)

//! A radio that records what a join sends, and the clock it runs on.

use nonos_wifi_core::mlme::{Entropy, JoinRequest, SAE_ENTROPY};
use nonos_wifi_core::rsn::JoinPolicy;

use super::beacons::OUR;
use crate::assoc::Radio;

// A radio that keeps what it was asked to send and never answers.
pub(super) struct RecordingRadio {
    pub(super) sent: Vec<Vec<u8>>,
}

impl Radio for RecordingRadio {
    fn send(&mut self, mpdu: &[u8]) -> bool {
        self.sent.push(mpdu.to_vec());
        true
    }
    fn recv(&mut self, _out: &mut [u8]) -> Option<usize> {
        None
    }
}

// A clock that moves `step` milliseconds each time it is read; the join reads it
// once to start and once per receive pass.
pub(super) fn clock(step: u64) -> impl FnMut() -> u64 {
    let mut t = 0u64;
    move || {
        let now = t;
        t += step;
        now
    }
}

// One millisecond a pass, so a budget of 32 is 32 passes as it was.
pub(super) fn passes() -> impl FnMut() -> u64 {
    clock(1)
}

pub(super) fn join_request(policy: JoinPolicy) -> JoinRequest<'static> {
    JoinRequest {
        our_mac: OUR,
        ssid: b"net",
        passphrase: b"password123",
        policy,
        entropy: Entropy { snonce: [0x11; 32], sae: [0x22; SAE_ENTROPY] },
    }
}

// The sequence number in a frame's sequence control field.
pub(super) fn seq_of(frame: &[u8]) -> u16 {
    u16::from_le_bytes([frame[22], frame[23]]) >> 4
}
