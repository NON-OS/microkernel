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

//! The encoder refuses what the decoder would, so every blob it writes reads.

use nonos_marketplace_abi::{encode_and_sign, EncodeError, MarketplaceIndex};

use super::fixtures::sold;

fn refused(index: MarketplaceIndex) -> Option<EncodeError> {
    encode_and_sign(index, |_| [0; 64]).err()
}

#[test]
fn every_entry_needs_commerce_and_every_release_its_fields() {
    let mut no_commerce = sold();
    no_commerce.entries[0].commerce = None;
    assert_eq!(refused(no_commerce), Some(EncodeError::MissingCommerce));
    let mut no_ext = sold();
    no_ext.entries[0].releases[1].ext = None;
    assert_eq!(refused(no_ext), Some(EncodeError::MissingReleaseExt));
}

#[test]
fn malformed_fields_are_refused_before_anything_is_signed() {
    let mut policy = sold();
    policy.entries[0].releases[0].ext.as_mut().unwrap().guest_policy.pop();
    assert_eq!(refused(policy), Some(EncodeError::BadLength));
    let mut kind = sold();
    kind.entries[0].commerce.as_mut().unwrap().proof_reqs[0].kind = 7;
    assert_eq!(refused(kind), Some(EncodeError::UnknownKind));
    let mut anchors = sold();
    anchors.entries[0].releases[0].ext.as_mut().unwrap().proof_anchors = vec![[1; 32]; 9];
    assert_eq!(refused(anchors), Some(EncodeError::TooMany));
    let mut unpinned = sold();
    unpinned.entries[0].releases[0].ext.as_mut().unwrap().content_len = 0;
    assert_eq!(refused(unpinned), Some(EncodeError::Inconsistent));
}
