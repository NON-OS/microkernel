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

use super::writer::Writer;
use crate::types::ReleaseExt;

/// The schema 3 release fields, in wire order. The index and the publisher's
/// signing bytes both write them through here, so the two cannot disagree.
pub(super) fn write(w: &mut Writer<'_>, x: &ReleaseExt) {
    w.fixed(&x.commerce_hash);
    w.lp_bytes(&x.guest_policy);
    w.fixed(&x.chunk_root);
    w.u64(x.content_len);
    w.u32(x.proof_anchors.len() as u32);
    for anchor in &x.proof_anchors {
        w.fixed(anchor);
    }
    w.fixed(&x.release_attest_sig);
}
