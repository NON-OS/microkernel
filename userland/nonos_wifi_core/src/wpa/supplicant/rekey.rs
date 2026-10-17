// NONOS Operating System (AGPL-3.0-or-later)

//! A pairwise rekey (IEEE Std 802.11-2020, 12.7.6). A message 1 on a connected
//! link is answered while the keys in use carry on; a message 3 naming a new
//! ANonce is checked under that ANonce's PTK, which takes over only if it
//! verifies. Nothing waits between the two, so a forged message 1 costs one
//! message 2 and changes nothing. The SNonce is HMAC-SHA256 under the current
//! KCK and KEK of a label, the ANonce, the seed (`seed.rs`), the rekeys
//! finished and the last SNonce.

use super::message3::is_message3;
use super::seed::Rekey;
use super::state::Supplicant;
use super::step::StepOutput;
use crate::eapol::mic::verify_mic_kind;
use crate::eapol::parse::EapolKey;

impl Supplicant {
    pub(super) fn on_rekey_message1(&mut self, key: &EapolKey<'_>) -> StepOutput {
        let mut r = self.rekey_to(&key.nonce);
        self.swap(&mut r);
        let reply = self.message2(&key.replay_counter);
        self.swap(&mut r);
        StepOutput::reply(reply)
    }

    /// Whether a message 3 for `anonce` would be a rekey's.
    pub(super) fn is_rekey(&self, anonce: &[u8; 32]) -> bool {
        *anonce != self.anonce
    }

    pub(super) fn on_rekey_message3(&mut self, frame: &[u8], key: &EapolKey<'_>) -> StepOutput {
        let mut r = self.rekey_to(&key.nonce);
        if !is_message3(key) || !verify_mic_kind(self.akm.mic_kind(), &r.ptk[..16], frame) {
            return StepOutput::none();
        }
        self.swap(&mut r);
        self.anonce = key.nonce;
        self.rekeys += 1;
        drop(r);
        self.rx_replay = Some(key.replay_counter);
        let mut out = self.take_message3(key);
        out.new_pairwise_key = out.reply.is_some();
        out
    }

    // The rekey's keys for the ones in use, and back.
    fn swap(&mut self, r: &mut Rekey) {
        core::mem::swap(&mut self.snonce, &mut r.snonce);
        core::mem::swap(&mut self.ptk, &mut r.ptk);
    }
}
