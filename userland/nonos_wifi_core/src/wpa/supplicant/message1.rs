// NONOS Operating System (AGPL-3.0-or-later)

//! Message 1 and the replay floor every handshake message is held to.

use super::state::{State, Supplicant};
use super::step::StepOutput;
use crate::eapol::parse::EapolKey;

impl Supplicant {
    // A replay counter must exceed the last one a verified MIC vouched for.
    // Message 1 carries no MIC, so it never raises the floor; it is held only
    // to the floor already set.
    pub(super) fn replay_is_fresh(&self, rc: &[u8; 8]) -> bool {
        match &self.rx_replay {
            Some(last) => u64::from_be_bytes(*rc) > u64::from_be_bytes(*last),
            None => true,
        }
    }

    // Message 1 -> derive the PTK, answer with message 2 (SNonce and the
    // station's RSNE, MIC keyed by the fresh KCK). A repeated message 1 before
    // message 3 is answered the same way, with the ANonce it carries: the AP
    // repeats it when message 2 was lost, and its replay counter advanced, so
    // the earlier message 2 would be discarded.
    pub(super) fn on_message1(&mut self, key: &EapolKey<'_>) -> StepOutput {
        self.anonce = key.nonce;
        self.ptk = self.akm.derive_ptk(&self.pmk, &self.aa, &self.spa, &self.anonce, &self.snonce);
        match self.message2(&key.replay_counter) {
            Some(frame) => {
                self.state = State::PtkDerived;
                StepOutput::reply(Some(frame))
            }
            None => {
                self.fail(super::state::Failure::Internal);
                StepOutput::none()
            }
        }
    }
}
