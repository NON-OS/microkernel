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

use std::fs;

use nonos_marketplace_abi::{SettleMode, SCHEMA_V3};

use crate::bootstrap_trust::{is_trusted, DEV_OPERATOR, DEV_SEED};
use crate::ingest::{load_verified, IngestError};
use crate::scratch::Scratch;
use crate::signed::signed;
use crate::verify::CryptoVerifier;

pub fn dev_signed(tag: &str) -> Vec<u8> {
    let dir = Scratch::new(tag);
    let seed = dir.path("seed");
    fs::write(&seed, DEV_SEED).expect("write the throwaway seed");
    signed(&dir, &seed, &DEV_OPERATOR)
}

#[test]
fn a_development_image_takes_its_own_catalogue() {
    assert!(is_trusted(&DEV_OPERATOR));
    let Ok(v) = load_verified(&dev_signed("image-takes"), &CryptoVerifier, 0) else {
        panic!("a development image refused the catalogue its seal signed");
    };
    assert!(v.signature_verified);
    assert!(v.publisher_signature_verified.iter().all(|&signed| signed));
    assert_eq!(v.publisher_signature_verified.len(), v.index.entries.len());
    assert_eq!(v.index.schema_version, SCHEMA_V3);
    let modes: Vec<_> = v.index.entries.iter().filter_map(|e| e.commerce.as_ref()).collect();
    assert_eq!(modes.len(), v.index.entries.len(), "schema 3: every listing has commerce");
    assert!(modes.iter().any(|c| c.settle_mode == SettleMode::Free));
    assert!(modes.iter().any(|c| c.settle_mode == SettleMode::Voucher && c.unit_price > 0));
    assert!(modes.iter().any(|c| c.settle_mode == SettleMode::Escrow && c.unit_price > 0));
}

/// A listing changed after signing is refused even under the trusted key.
#[test]
fn a_development_image_refuses_its_catalogue_altered() {
    let mut blob = dev_signed("image-altered");
    let text = b"Nothing is sold.";
    let at = blob.windows(text.len()).position(|w| w == text).expect("the free listing");
    blob[at] = b'S';
    let why = load_verified(&blob, &CryptoVerifier, 0).err();
    assert!(why == Some(IngestError::SignatureRefused));
}
