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

use nonos_marketplace_abi::{Commerce, ReleaseExt};

use super::hex_more::hex_or_empty;
use super::schema::{CommerceJson, ProofReqJson, ReleaseExtJson};
use super::units::{settle_mode_str, unit_label};

pub(super) fn commerce_to_json(c: &Commerce) -> CommerceJson {
    CommerceJson {
        payee: format!("0x{}", hex::encode(c.payee)),
        bond_id: hex::encode(c.bond_id),
        unit: unit_label(&c.unit),
        unit_price: c.unit_price.to_string(),
        min_charge: c.min_charge.to_string(),
        settle_mode: settle_mode_str(c.settle_mode).into(),
        proof_reqs: c
            .proof_reqs
            .iter()
            .map(|r| ProofReqJson { kind: r.kind, statement: hex::encode(r.statement) })
            .collect(),
        binding_sig: hex_or_empty(&c.binding_sig),
    }
}

pub(super) fn ext_to_json(x: &ReleaseExt) -> ReleaseExtJson {
    ReleaseExtJson {
        commerce_hash: hex_or_empty(&x.commerce_hash),
        guest_policy: hex::encode(&x.guest_policy),
        chunk_root: hex_or_empty(&x.chunk_root),
        content_len: x.content_len,
        proof_anchors: x.proof_anchors.iter().map(hex::encode).collect(),
        release_attest_sig: hex_or_empty(&x.release_attest_sig),
    }
}
