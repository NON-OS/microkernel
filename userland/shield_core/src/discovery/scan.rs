//! Deciding which of the pool's outputs belong to this wallet, trying each one locally.
//! An opened ciphertext is kept only if it commits to the tree's leaf with a spend key we derive.

use super::output::PoolOutput;
use super::stats::ScanStats;
use super::words::{quad, words};
use crate::keys::Account;
use crate::notes::{
    commitment, commitment_bytes, open_note, NoteCipher, NoteRecord, NoteStatus, Opened,
};
use nonos_stark::air::{Poseidon, RATE};
use nonos_stark::field::Fp;
use stark_proofs::shield::note::POOL_LOG_ROUNDS;

/// Try every output against this account. Returns the notes that opened and the cost of the pass.
pub fn scan(account: &Account, outputs: &[PoolOutput]) -> (Vec<NoteRecord>, ScanStats) {
    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
    let mut found = Vec::new();
    let mut stats = ScanStats { outputs: outputs.len() as u64, ..ScanStats::default() };
    for out in outputs {
        let Some(cipher) = NoteCipher::decode(&out.cipher) else {
            stats.rejected = stats.rejected.saturating_add(1);
            continue;
        };
        stats.agreements = stats.agreements.saturating_add(1);
        let cm_words: [Fp; RATE] = quad(&out.cm);
        let plain = match open_note(&cipher, account.view(), &commitment_bytes(&cm_words)) {
            Opened::TagMiss => continue,
            Opened::AuthFail => {
                stats.tag_hits = stats.tag_hits.saturating_add(1);
                continue;
            }
            Opened::Note(plain) => {
                stats.tag_hits = stats.tag_hits.saturating_add(1);
                plain
            }
        };
        let note = plain.note();
        if commitment(&h, &note) != cm_words || note.spend_pk != words(account.spend_pk()) {
            stats.rejected = stats.rejected.saturating_add(1);
            continue;
        }
        stats.notes_found = stats.notes_found.saturating_add(1);
        found.push(NoteRecord {
            plain,
            leaf_index: out.leaf_index,
            cm: out.cm,
            status: NoteStatus::Unspent,
            found_at: out.leaf_index,
        });
    }
    (found, stats)
}
