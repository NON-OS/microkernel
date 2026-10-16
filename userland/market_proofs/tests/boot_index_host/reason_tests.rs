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

//! A whole catalogue that ingest refuses is said with its reason, a stale one is not.

use crate::fixture::{load_file, said, MAX, NO_BASELINE};
use crate::ingest::IngestError;

#[test]
fn a_whole_catalogue_ingest_refuses_is_said_with_its_reason() {
    let why: [(IngestError, &[u8]); 3] = [
        (IngestError::Malformed, b"it does not decode"),
        (IngestError::UntrustedOperator, b"its operator key is not one this system trusts"),
        (IngestError::SignatureRefused, b"its signature did not verify"),
    ];
    for (refusal, reason) in why {
        let (_, store) = load_file(MAX, Some(refusal));
        let line =
            [&b"[MARKET] the operator's catalogue was refused: "[..], reason, b"\n"].concat();
        assert_eq!(said(), [NO_BASELINE.to_vec(), line]);
        assert!(store.installs().is_empty());
    }
}

#[test]
fn a_stale_catalogue_is_not_installed_and_not_said() {
    let (_, store) = load_file(MAX, Some(IngestError::StaleSerial));
    assert_eq!(said(), [NO_BASELINE]);
    assert!(store.installs().is_empty());
}
