// NONOS Operating System (AGPL-3.0-or-later)

//! Drive one EAPOL-Key frame through the handshake (IEEE Std 802.11-2020,
//! 12.7.6 and 12.7.7). Every frame must be an RSN descriptor with Key Ack set,
//! no Request or Error, the key descriptor version of the negotiated AKM, and
//! a replay counter above the last one whose MIC verified; anything else is
//! dropped unanswered. Message 1 (no MIC) yields message 2, also when the AP
//! repeats message 1 because message 2 was lost. Message 3 must carry Install,
//! Secure and Encrypted Key Data, verify under the KCK, repeat the ANonce, and
//! hold the beacon's RSNE; it yields message 4. Once connected, a repeated
//! message 3 is answered with message 4 again without reinstalling any key,
//! and a group message 1 delivers the next GTK and yields group message 2.
//!
//! A frame that fails its MIC is dropped, not treated as the end of the
//! handshake: an unauthenticated frame must not be able to tear the join down.
//! Only an authentic message that is unacceptable (a downgraded RSNE, key data
//! without a valid group key) fails it.

use super::state::{State, Supplicant};
use crate::eapol::parse::{
    parse, DESCRIPTOR_RSN, KEY_INFO_ACK, KEY_INFO_ERROR, KEY_INFO_INDEX_MASK, KEY_INFO_MIC,
    KEY_INFO_PAIRWISE, KEY_INFO_REQUEST, KEY_INFO_VERSION_MASK,
};

pub use super::output::StepOutput;

impl Supplicant {
    pub fn step(&mut self, frame: &[u8]) -> StepOutput {
        if self.state == State::Failed {
            return StepOutput::none();
        }
        let Some(key) = parse(frame) else {
            return StepOutput::none();
        };
        let info = key.key_info;
        if key.descriptor_type != DESCRIPTOR_RSN
            || info & KEY_INFO_ACK == 0
            || info & (KEY_INFO_REQUEST | KEY_INFO_ERROR) != 0
            || info & KEY_INFO_VERSION_MASK != self.akm.key_version()
            || !self.replay_is_fresh(&key.replay_counter)
        {
            return StepOutput::none();
        }
        // Reply in the 802.1X version the authenticator spoke.
        self.eapol_version = match frame[0] {
            v @ 1..=3 => v,
            _ => crate::eapol::build::EAPOL_VERSION_DEFAULT,
        };
        let has_mic = info & KEY_INFO_MIC != 0;
        if info & KEY_INFO_PAIRWISE != 0 {
            if info & KEY_INFO_INDEX_MASK != 0 {
                return StepOutput::none();
            }
            return match (self.state, has_mic) {
                (State::Start | State::PtkDerived, false) => self.on_message1(&key),
                (State::PtkDerived, true) => self.on_message3(frame, &key),
                (State::Connected, false) => self.on_rekey_message1(&key),
                (State::Connected, true) if self.rekey_for(&key.nonce) => {
                    self.on_rekey_message3(frame, &key)
                }
                (State::Connected, true) => self.on_message3_again(frame, &key),
                _ => StepOutput::none(),
            };
        }
        if has_mic && self.state == State::Connected {
            return self.on_group1(frame, &key);
        }
        StepOutput::none()
    }
}
