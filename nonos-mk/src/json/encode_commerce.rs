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

use nonos_marketplace_abi::{bond_id, Commerce, ProofReq, ReleaseExt};

use crate::error::CliError;
use crate::hosts::Host;

use super::hex_helpers::hex20;
use super::hex_more::{decimal, hex_bytes, hex_n};
use super::schema::{CommerceJson, ReleaseExtJson};
use super::units::{settle_mode, unit_word};

pub(super) fn to_commerce(c: &CommerceJson, publisher: &[u8; 32]) -> Result<Commerce, CliError> {
    let payee = hex20(&c.payee, "commerce.payee")?;
    let derived = bond_id(&Host, &payee, publisher);
    let given = hex_n::<32>(&c.bond_id, "commerce.bond_id")?;
    if given != [0; 32] && given != derived {
        return Err(CliError::Json("commerce.bond_id: not this payee's bond".into()));
    }
    let proof_reqs = c
        .proof_reqs
        .iter()
        .map(|r| Ok(ProofReq { kind: r.kind, statement: hex_n(&r.statement, "statement")? }))
        .collect::<Result<_, CliError>>()?;
    Ok(Commerce {
        payee,
        bond_id: derived,
        unit: unit_word(&c.unit)?,
        unit_price: decimal(&c.unit_price, "commerce.unit_price")?,
        min_charge: decimal(&c.min_charge, "commerce.min_charge")?,
        settle_mode: settle_mode(&c.settle_mode)?,
        proof_reqs,
        binding_sig: hex_n(&c.binding_sig, "commerce.binding_sig")?,
    })
}

pub(super) fn to_ext(x: &ReleaseExtJson) -> Result<ReleaseExt, CliError> {
    let proof_anchors =
        x.proof_anchors.iter().map(|a| hex_n(a, "proof_anchors")).collect::<Result<_, _>>()?;
    Ok(ReleaseExt {
        commerce_hash: hex_n(&x.commerce_hash, "ext.commerce_hash")?,
        guest_policy: hex_bytes(&x.guest_policy, "ext.guest_policy")?,
        chunk_root: hex_n(&x.chunk_root, "ext.chunk_root")?,
        content_len: x.content_len,
        proof_anchors,
        release_attest_sig: hex_n(&x.release_attest_sig, "ext.release_attest_sig")?,
    })
}
