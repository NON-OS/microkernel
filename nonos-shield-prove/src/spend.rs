// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The spend's witness: the pool's trees rebuilt from the leaves the request
//! lists and held to the published roots, the seed notes found at the places
//! the request names, the balance checked, and the two created notes drawn.

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use stark_proofs::crypto::stark::air::{Poseidon, RATE};
use stark_proofs::crypto::stark::field::Fp;
use stark_proofs::shield::join::publics::NOT_BEFORE_GRID_S;
use stark_proofs::shield::join::{
    intent_parts_anchored, Anchor, AssocAnchor, IntentParts, Settle, Spend, Witnessed,
};
use stark_proofs::shield::key::{derive, Break};
use stark_proofs::shield::live_seed::seed_notes;
use stark_proofs::shield::member::{PoolTree, TREE_DEPTH};
use stark_proofs::shield::note::{note_parts, Note, POOL_LOG_ROUNDS};

use crate::json::{pack_u256, quad, try_address, try_unpack_digest, Json};

/// The notes a spend creates, with the secrets of those this spender owns.
pub struct Created {
    /// Four limbs per output, each the spend secret of that note; zero for
    /// an output keyed to a receiver.
    pub secrets: Vec<Fp>,
    pub notes: [Note; 2],
    pub owned: [bool; 2],
}

fn tree_of(h: &Poseidon, leaves: &[&str]) -> Result<PoolTree, String> {
    let mut t = PoolTree::with_depth(h.clone(), TREE_DEPTH);
    for l in leaves {
        t.insert(try_unpack_digest(l)?);
    }
    Ok(t)
}

fn opened(t: &PoolTree, index: usize) -> Witnessed {
    Witnessed { leaf_index: index, siblings: t.path(index).0 }
}

pub fn build(
    req_text: &str,
    seed_text: &str,
    words: &mut dyn FnMut(usize) -> Result<Vec<Fp>, String>,
) -> Result<(IntentParts, Created), String> {
    let req = Json(req_text);
    let pool_leaves = req.try_strings("pool_leaves")?;
    let assoc_leaves = req.try_strings("assoc_leaves")?;
    let pool_at = req.try_u64s("input_pool_index")?;
    let assoc_at = req.try_u64s("input_assoc_index")?;
    let note_root = try_unpack_digest(req.try_string("note_root")?)?;
    let assoc_root = try_unpack_digest(req.try_string("assoc_root")?)?;
    let recipient = try_address(req.try_string("recipient")?)?;
    let fee_recipient = if req.has("fee_recipient") {
        try_address(req.try_string("fee_recipient")?)?
    } else {
        [0u8; 20]
    };
    let clearing_price = req.try_u64("clearing_price")?;
    // The earliest settlement time, on the grid, so a wallet cannot publish
    // a value that names it.
    let not_before = req.try_u64("not_before")?;
    if not_before == 0 || not_before % NOT_BEFORE_GRID_S != 0 {
        return Err(format!(
            "not_before {not_before} is not a positive multiple of {NOT_BEFORE_GRID_S} seconds"
        ));
    }
    let public_amount = req.try_u64("public_amount")?;
    let fee = req.try_u64("fee")?;
    if fee != 0 && fee_recipient == [0u8; 20] {
        return Err("a nonzero fee needs a fee_recipient, the submitter it pays".to_string());
    }
    if fee == 0 && fee_recipient != [0u8; 20] {
        return Err("fee_recipient names a submitter but the fee is zero".to_string());
    }
    let out_values = req.try_u64s("output_values")?;
    if pool_at.len() != 2 || assoc_at.len() != 2 || out_values.len() != 2 {
        return Err(
            "input_pool_index, input_assoc_index and output_values each take exactly two entries"
                .to_string(),
        );
    }
    let (sks, inputs) = seed_notes(seed_text).map_err(|why| format!("seed file: {why}"))?;
    let h = Poseidon::new(POOL_LOG_ROUNDS, [Fp::ZERO; RATE]);
    let pool = tree_of(&h, &pool_leaves)?;
    let assoc = tree_of(&h, &assoc_leaves)?;
    if pool.root() != note_root {
        return Err(format!(
            "the {} pool leaves rebuild to {} and the pool published {}; the list is not the pool's",
            pool_leaves.len(),
            pack_u256(&pool.root()),
            pack_u256(&note_root)
        ));
    }
    if assoc.root() != assoc_root {
        return Err(format!(
            "the {} association leaves rebuild to {} and the registry published {}",
            assoc_leaves.len(),
            pack_u256(&assoc.root()),
            pack_u256(&assoc_root)
        ));
    }
    // A note worth zero is a dummy: in no tree, its index only names the
    // opening its walk runs along.
    for i in 0..2 {
        if inputs[i].value == 0 {
            continue;
        }
        let cm = note_parts(&inputs[i]).cm;
        let at = pool_at[i] as usize;
        if at >= pool_leaves.len() || try_unpack_digest(pool_leaves[at])? != cm {
            return Err(format!("seed note {i} is not the pool's leaf {at}"));
        }
        let at = assoc_at[i] as usize;
        if at >= assoc_leaves.len() || try_unpack_digest(assoc_leaves[at])? != cm {
            return Err(format!("seed note {i} is not the association set's leaf {at}"));
        }
    }
    let inn: u64 = inputs.iter().map(|n| n.value).sum();
    let outv: u64 = out_values.iter().sum();
    if inn != outv + public_amount + fee {
        return Err(format!(
            "does not balance: {inn} in against {outv} out plus {public_amount} public and {fee} fee"
        ));
    }
    let asset_id = inputs[0].asset_id;
    if inputs[1].asset_id != asset_id {
        return Err(
            "the two seed notes carry different assets; a transfer moves one asset".to_string(),
        );
    }
    // Per output, a receiver's spend key as the pool word, or "self".
    let to: Vec<Option<[Fp; RATE]>> = if req.has("output_spend_pk") {
        let named = req.try_strings("output_spend_pk")?;
        if named.len() != 2 {
            return Err(
                "output_spend_pk takes exactly two entries, a pool word or \"self\"".to_string(),
            );
        }
        named
            .iter()
            .map(|w| if *w == "self" { Ok(None) } else { try_unpack_digest(w).map(Some) })
            .collect::<Result<Vec<_>, String>>()?
    } else {
        alloc::vec![None, None]
    };
    let owned = [to[0].is_none(), to[1].is_none()];
    let w = words(4 * RATE)?;
    let (osk, oblind) = w.split_at(2 * RATE);
    let outputs: [Note; 2] = core::array::from_fn(|i| {
        let s: [Fp; RATE] = core::array::from_fn(|j| osk[i * RATE + j]);
        Note {
            value: out_values[i],
            asset_id,
            spend_pk: match to[i] {
                Some(pk) => quad(&pk),
                None => quad(&derive(&h, s).spend_pk),
            },
            blinding: quad(&oblind[i * RATE..]),
        }
    });
    let pool_open = [opened(&pool, pool_at[0] as usize), opened(&pool, pool_at[1] as usize)];
    let assoc_open = [opened(&assoc, assoc_at[0] as usize), opened(&assoc, assoc_at[1] as usize)];
    let parts = intent_parts_anchored(
        [Spend { note: &inputs[0], sk: sks[0] }, Spend { note: &inputs[1], sk: sks[1] }],
        [&outputs[0], &outputs[1]],
        public_amount,
        fee,
        Break::None,
        Settle { clearing_price, recipient, fee_recipient, not_before },
        None,
        TREE_DEPTH,
        Anchor::Published {
            openings: [&pool_open[0], &pool_open[1]],
            root: note_root,
            assoc: Some(AssocAnchor { openings: [&assoc_open[0], &assoc_open[1]], root: assoc_root }),
        },
    );
    let mut secrets = osk.to_vec();
    for (i, own) in owned.iter().enumerate() {
        if !own {
            for s in &mut secrets[i * RATE..(i + 1) * RATE] {
                *s = Fp::ZERO;
            }
        }
    }
    Ok((parts, Created { secrets, notes: outputs, owned }))
}
