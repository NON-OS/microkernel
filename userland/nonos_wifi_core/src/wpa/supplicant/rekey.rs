// NONOS Operating System (AGPL-3.0-or-later)

//! A pairwise rekey (IEEE Std 802.11-2020, 12.7.6): the AP runs the four-way
//! handshake again on a connected link. Its message 1 is answered under a new
//! SNonce and the PTK of the new ANonce, which waits while the keys in use
//! carry on, until a message 3 for that ANonce verifies under it; only then
//! does it replace them. A forged message 1 (it carries no MIC) costs one
//! message 2 and changes nothing. The new SNonce is HMAC-SHA256, keyed by the
//! current KCK and KEK, of a label, the new ANonce and the last SNonce: new
//! for every ANonce and unknown without the PTK, where the standard asks for
//! a random one and the supplicant holds no randomness after the join.

use super::message3::is_message3;
use super::state::Supplicant;
use super::step::StepOutput;
use crate::eapol::mic::verify_mic_kind;
use crate::eapol::parse::EapolKey;
use crate::wipe::wipe;
use crate::wpa::sha256::hmac_sha256_parts;

const LABEL: &[u8] = b"NONOS rekey SNonce";

/// The rekey a message 1 started: its ANonce, our SNonce, their PTK.
#[derive(Clone)]
pub(in crate::wpa::supplicant) struct Rekey {
    anonce: [u8; 32],
    snonce: [u8; 32],
    ptk: [u8; 48],
}

impl Drop for Rekey {
    fn drop(&mut self) {
        wipe(&mut self.snonce);
        wipe(&mut self.ptk);
    }
}

impl Supplicant {
    pub(super) fn on_rekey_message1(&mut self, key: &EapolKey<'_>) -> StepOutput {
        let snonce = hmac_sha256_parts(&self.ptk[..32], &[LABEL, &key.nonce, &self.snonce]);
        let ptk = self.akm.derive_ptk(&self.pmk, &self.aa, &self.spa, &key.nonce, &snonce);
        let mut r = Rekey { anonce: key.nonce, snonce, ptk };
        self.swap(&mut r);
        let reply = self.message2(&key.replay_counter);
        self.swap(&mut r);
        self.rekey = Some(r);
        StepOutput::reply(reply)
    }

    /// Whether a rekey for `anonce` is waiting for its message 3.
    pub(super) fn rekey_for(&self, anonce: &[u8; 32]) -> bool {
        self.rekey.as_ref().is_some_and(|r| r.anonce == *anonce)
    }

    pub(super) fn on_rekey_message3(&mut self, frame: &[u8], key: &EapolKey<'_>) -> StepOutput {
        let Some(mut r) = self.rekey.take() else { return StepOutput::none() };
        if !is_message3(key) || !verify_mic_kind(self.akm.mic_kind(), &r.ptk[..16], frame) {
            self.rekey = Some(r);
            return StepOutput::none();
        }
        self.swap(&mut r);
        self.anonce = key.nonce;
        drop(r);
        self.rx_replay = Some(key.replay_counter);
        let mut out = self.take_message3(key);
        out.new_pairwise_key = out.reply.is_some();
        out
    }

    // The pending keys for the ones in use, and back.
    fn swap(&mut self, r: &mut Rekey) {
        core::mem::swap(&mut self.snonce, &mut r.snonce);
        core::mem::swap(&mut self.ptk, &mut r.ptk);
    }
}
