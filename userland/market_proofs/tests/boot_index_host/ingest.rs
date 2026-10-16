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

//! The capsule's IngestError, mounted, and a host stand-in for load_verified.
//
// The stand-in records each blob it is handed and answers with the refusal a
// test set, or, with none set, a catalogue one serial past the store's.

#[path = "../../../capsule_market/src/ingest/error.rs"]
mod error;

pub use error::IngestError;

use std::cell::{Cell, RefCell};

use crate::verify::Verifier;

pub struct Verified {
    pub index: u64,
    pub signature_verified: bool,
    pub publisher_signature_verified: Vec<bool>,
}

thread_local! {
    static BLOBS: RefCell<Vec<Vec<u8>>> = const { RefCell::new(Vec::new()) };
    static REFUSAL: Cell<Option<IngestError>> = const { Cell::new(None) };
}

pub fn load_verified<V: Verifier>(
    blob: &[u8],
    _verifier: &V,
    last_serial: u64,
) -> Result<Verified, IngestError> {
    BLOBS.with(|b| b.borrow_mut().push(blob.to_vec()));
    match REFUSAL.with(Cell::get) {
        Some(e) => Err(e),
        None => Ok(Verified {
            index: last_serial + 1,
            signature_verified: true,
            publisher_signature_verified: Vec::new(),
        }),
    }
}

/// Forgets the blobs handed in and sets the refusal the next loads answer with.
pub fn reset(refusal: Option<IngestError>) {
    BLOBS.with(|b| b.borrow_mut().clear());
    REFUSAL.with(|r| r.set(refusal));
}

pub fn blobs() -> Vec<Vec<u8>> {
    BLOBS.with(|b| b.borrow().clone())
}
