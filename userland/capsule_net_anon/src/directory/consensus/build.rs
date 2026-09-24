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

//! Accumulating entries and signatures as the scan walks past them.

extern crate alloc;

use alloc::vec::Vec;

use super::document::Consensus;
use super::entry::Entry;
use super::pem;
use super::signature;

pub(super) fn set(current: &mut Option<Entry>, apply: impl FnOnce(&mut Entry)) {
    if let Some(entry) = current.as_mut() {
        apply(entry);
    }
}

pub(super) fn push(entries: &mut Vec<Entry>, entry: Option<Entry>) {
    if let Some(entry) = entry.filter(|e| e.complete()) {
        entries.push(entry);
    }
}

pub(super) fn collect(out: &mut Consensus, body: &[u8], at: usize, rest: &[u8]) {
    let Some((sha256, identity, signing_key)) = signature::parse_header(rest) else {
        return;
    };
    let Some(bytes) = pem::object_after(body, at) else {
        return;
    };
    out.signatures.push(signature::Signature { sha256, identity, signing_key, bytes });
}
