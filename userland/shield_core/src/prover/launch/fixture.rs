//! A transfer to prove when nothing is funded, on the launch pool's circuit, point and trees.
//! Outputs are standard sizes, so the anonymity rules hold and nothing is marked weakened.

use super::request::SpendRequest;
use super::tree::pool_root;
use crate::keys::Account;
use crate::notes::{commitment, NotePlaintext};
use crate::prover::pool_hasher;

/// 0.001 NOX in note units at scale 10^9, the standard NOX size, and a fee under the relay cap.
const MILLI: u64 = 1_000_000;
const FEE: u64 = 1_000;

fn cm(plain: &NotePlaintext) -> [u64; 4] {
    commitment(&pool_hasher(), &plain.note()).map(|f| f.value())
}

/// The fixture: 0.002 and 0.001001 NOX in, 0.001 out to a payee, 0.002 back.
pub fn transfer(me: &Account) -> (SpendRequest, NotePlaintext, NotePlaintext) {
    let pk = me.address().spend_pk;
    let a = NotePlaintext { value: 2 * MILLI, asset_id: 1, blinding: [5, 6, 7, 8], spend_pk: pk };
    let b =
        NotePlaintext { value: MILLI + FEE, asset_id: 1, blinding: [9, 10, 11, 12], spend_pk: pk };
    let stranger = [1u64, 2, 3, 4];
    let leaves = vec![stranger, cm(&a), cm(&b)];
    let root = pool_root(&leaves);
    let request = SpendRequest {
        assoc_leaves: leaves.clone(),
        assoc_root: root,
        note_root: root,
        pool_leaves: leaves,
        input_pool_index: [1, 2],
        input_assoc_index: [1, 2],
        output_values: [MILLI, 2 * MILLI],
        output_spend_pk: [[21, 22, 23, 24], pk],
        public_amount: 0,
        fee: FEE,
        clearing_price: 0,
        asset_id: 1,
        recipient: [0; 20],
        not_before: 1_790_000_400,
        fee_recipient: [9; 20],
    };
    (request, a, b)
}
