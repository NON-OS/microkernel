// NONOS Operating System (AGPL-3.0-or-later)

//! The first message 3: checked, keyed, and answered with message 4.

use super::super::keys::{parse_group_keys, rsc_pn, wipe, UNWRAPPED_MAX};
use super::super::state::{Failure, State, Supplicant};
use super::super::step::StepOutput;
use crate::eapol::parse::EapolKey;

impl Supplicant {
    pub(in super::super) fn on_message3(&mut self, frame: &[u8], key: &EapolKey<'_>) -> StepOutput {
        if !self.authentic_message3(frame, key) {
            return StepOutput::none();
        }
        self.rx_replay = Some(key.replay_counter);
        self.take_message3(key)
    }

    /// Take an authentic message 3's keys and answer it with message 4.
    pub(in super::super) fn take_message3(&mut self, key: &EapolKey<'_>) -> StepOutput {
        let mut plain = [0u8; UNWRAPPED_MAX];
        let out = self.accept_message3(key, &mut plain);
        wipe(&mut plain);
        out
    }

    fn accept_message3(&mut self, key: &EapolKey<'_>, plain: &mut [u8; UNWRAPPED_MAX]) -> StepOutput {
        let Some(n) = self.unwrap_key_data(key.key_data, plain) else {
            self.fail(Failure::BadKeyData);
            return StepOutput::none();
        };
        let Some((kd, mut keys)) = parse_group_keys(&plain[..n]) else {
            self.fail(Failure::BadKeyData);
            return StepOutput::none();
        };
        // With management frame protection the IGTK must come too: without it
        // no deauthentication sent to all could ever be checked.
        if self.pmf && keys.igtk.is_none() {
            self.fail(Failure::BadKeyData);
            return StepOutput::none();
        }
        // The downgrade check: the elements the AP signs into message 3 must be
        // the ones its beacon showed before any key existed. Every supplicant
        // has them: none is built without the beacon's elements.
        if !self.ap_rsne.matches(kd.rsne) || !self.ap_rsnxe.matches(kd.rsnxe) {
            self.fail(Failure::IeMismatch);
            return StepOutput::none();
        }
        keys.rsc = rsc_pn(&key.key_rsc);
        self.transition_disable = kd.transition_disable.unwrap_or(0);
        self.store_group_keys(&keys);
        match self.message4(&key.replay_counter) {
            Some(frame) => {
                self.state = State::Connected;
                StepOutput::reply(Some(frame))
            }
            None => {
                self.fail(Failure::Internal);
                StepOutput::none()
            }
        }
    }
}
