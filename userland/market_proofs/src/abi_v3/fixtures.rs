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

use alloc::vec;

use nonos_marketplace_abi::{Commerce, MarketplaceIndex, ProofReq, ReleaseExt, SettleMode};
use nonos_nox_config::UNIT_INSTALL;

pub(crate) fn free_commerce() -> Commerce {
    Commerce {
        payee: [0; 20],
        bond_id: [0; 32],
        unit: UNIT_INSTALL,
        unit_price: 0,
        min_charge: 0,
        settle_mode: SettleMode::Free,
        proof_reqs: vec![],
        binding_sig: [0; 65],
    }
}

pub(crate) fn plain_ext() -> ReleaseExt {
    ReleaseExt {
        commerce_hash: [0; 32],
        guest_policy: vec![],
        chunk_root: [0; 32],
        content_len: 0,
        proof_anchors: vec![],
        release_attest_sig: [0; 65],
    }
}

/// A paid escrow listing with every schema 3 field set to something.
pub(crate) fn sold() -> MarketplaceIndex {
    let mut index = crate::release_tests::index();
    let entry = &mut index.entries[0];
    entry.description = "DESC".into();
    entry.commerce = Some(Commerce {
        payee: [0x5a; 20],
        bond_id: [0xb0; 32],
        unit_price: 4_000_000_000_000_000_000,
        min_charge: 1,
        settle_mode: SettleMode::Escrow,
        proof_reqs: vec![ProofReq { kind: 1, statement: [0x15; 32] }],
        binding_sig: [0x51; 65],
        ..free_commerce()
    });
    for (i, rel) in entry.releases.iter_mut().enumerate() {
        rel.ext = Some(ReleaseExt {
            commerce_hash: [0xc0; 32],
            guest_policy: vec![0x9a; 256],
            chunk_root: [0xcc + i as u8; 32],
            content_len: 4096 + i as u64,
            proof_anchors: vec![[0xa1; 32], [0xa2; 32]],
            release_attest_sig: [0x5a + i as u8; 65],
        });
    }
    index
}
