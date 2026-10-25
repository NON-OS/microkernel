//! A transfer and a withdrawal as the wallet builds them, read by the prover's
//! own request checks: roots, membership, the dummy beside a single note,
//! balance and ownership. Nothing is proved, so this runs in a moment.

#[path = "spend_refused_test.rs"]
mod refused;
#[path = "spend_taken_test.rs"]
mod taken;

use super::build::request;
use super::pick::pick;
use super::{Destination, Order};
use crate::custody::{generate_phrase, phrase_to_seed};
use crate::keys::Account;
use crate::net::asset::{Asset, NOX};
use crate::net::pool::LAUNCH;
use crate::notes::{commitment, NotePlaintext, NoteRecord, NoteStatus};
use crate::prover::launch::seed::seed_json;
use crate::prover::launch::tree::pool_root;
use crate::prover::pool_hasher;
use crate::wallet::anchor::Anchor;
use nonos_stark::field::Fp;

const MILLI: u64 = 1_000_000;

fn account() -> Account {
    Account::from_seed(&phrase_to_seed(&generate_phrase().expect("entropy")).expect("seed"))
        .expect("account")
}

/// 0.001 ETH in wei, the smallest standard ETH size.
const FINNEY: u64 = 1_000_000_000_000_000;

fn held(me: &Account, value: u64, leaf: u64) -> NoteRecord {
    held_of(me, NOX, value, leaf)
}

fn held_of(me: &Account, asset: Asset, value: u64, leaf: u64) -> NoteRecord {
    let plain = NotePlaintext {
        value,
        asset_id: asset.id,
        blinding: [leaf + 5, 6, 7, 8],
        spend_pk: me.address().spend_pk,
    };
    let cm = commitment(&pool_hasher(), &plain.note()).map(|f| f.value());
    NoteRecord { plain, leaf_index: leaf, cm, status: NoteStatus::Unspent, found_at: leaf }
}

fn accepted(me: &Account, notes: &[NoteRecord], to: Destination, amount: u64) {
    accepted_of(me, NOX, notes, to, amount);
}

fn accepted_of(me: &Account, asset: Asset, notes: &[NoteRecord], to: Destination, amount: u64) {
    let mut leaves: Vec<[u64; 4]> = vec![[1, 2, 3, 4]];
    leaves.extend(notes.iter().map(|n| n.cm));
    let anchor = Anchor { root: pool_root(&leaves), leaves };
    let refs: Vec<&NoteRecord> = notes.iter().collect();
    let order =
        Order { asset, to, amount, fee: 1_000, fee_to: [9; 20], not_before: 600, early: false };
    let picked =
        pick(&refs, &anchor, asset.id, amount + 1_000, &|_| true).expect("notes that cover it");
    let req = request(&LAUNCH, &anchor, &picked, &order, me.address().spend_pk)
        .expect("the pool's rules");
    let seed = seed_json(&me.sk().map(|f| f.value()), [&picked.notes[0], &picked.notes[1]]);
    let mut words = |k: usize| Ok(vec![Fp::from_u64(3); k]);
    stark_proofs::host::build_parts_with(&req.to_json(), &seed, &mut words)
        .expect("the prover refused it");
}
