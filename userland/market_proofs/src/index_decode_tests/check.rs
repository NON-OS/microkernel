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

use nonos_marketplace_abi::{decode_index, MAX_ARCHES, MAX_CAPABILITIES, MAX_ENTRIES};
use nonos_marketplace_abi::{MAX_PUBLISHER, MAX_RELEASES, MAX_SIGNATURE};

/// What must hold for any blob.
pub(super) fn check(blob: &[u8]) -> bool {
    let Ok(decoded) = decode_index(blob) else { return false };
    assert!(blob.starts_with(decoded.signed_bytes), "the signed bytes are the blob's head");
    assert!(decoded.signed_bytes.len() + 4 + decoded.index.index_signature.len() <= blob.len());
    assert!(decoded.index.index_signature.len() <= MAX_SIGNATURE as usize);
    assert!(decoded.index.operator_id.len() <= MAX_PUBLISHER as usize);
    assert!(decoded.index.entries.len() <= MAX_ENTRIES as usize);
    for e in &decoded.index.entries {
        assert!(e.releases.len() <= MAX_RELEASES as usize);
        for r in &e.releases {
            assert!(r.supported_arches.len() <= MAX_ARCHES as usize);
            assert!(r.required_capabilities.len() <= MAX_CAPABILITIES as usize);
        }
    }
    true
}
