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

//! The catalogue this capsule starts with.

use nonos_app_skeleton::clients::vfs::read_file;
use nonos_libc::{mk_debug, mk_getpid};

use crate::ingest::{load_verified, IngestError};
use crate::store::Store;
use crate::verify::Verifier;

/// Where an operator drops a catalogue newer than the built-in one.
const PATH: &[u8] = b"/nonos/marketplace/index.bin";

/// A catalogue of every listing a machine could offer is still small next to
/// one package.
const MAX: u32 = 8 << 20;

/// The catalogue this image shipped with, written by tools/nonos-market-index.
/// Empty when the build had no operator seed, which reads as "no baseline"
/// rather than as a failure.
static BASELINE: &[u8] = include_bytes!("../../../target/market/index.bin");

pub fn load<V: Verifier>(store: &mut Store, verifier: &V) {
    match BASELINE.is_empty() {
        false => take(store, verifier, BASELINE, b"built-in"),
        true => say(b"[MARKET] this image was built with no catalogue; nothing to list\n"),
    }
    if let Ok(blob) = read_file(mk_getpid(), PATH, MAX) {
        take(store, verifier, &blob, b"operator's");
    }
}

fn take<V: Verifier>(store: &mut Store, verifier: &V, blob: &[u8], which: &[u8]) {
    /*
     * A serial no newer than the one already held is the ordinary outcome, not
     * an error: it means no operator has published since this image was built.
     * Any other refusal is said on the log: the window can only say that no
     * signed catalogue is held, and a catalogue that was there and refused
     * must not read as one that never was.
     */
    let why: &[u8] = match load_verified(blob, verifier, store.last_serial()) {
        Ok(v) => {
            store.install(v.index, v.signature_verified, v.publisher_signature_verified);
            return;
        }
        Err(IngestError::StaleSerial) => return,
        Err(IngestError::Malformed) => b"it does not decode",
        Err(IngestError::UntrustedOperator) => b"its operator key is not one this system trusts",
        Err(IngestError::SignatureRefused) => b"its signature did not verify",
    };
    say(&[b"[MARKET] the ", which, b" catalogue was refused: ", why, b"\n"].concat());
}

fn say(line: &[u8]) {
    let _ = mk_debug(line.as_ptr(), line.len());
}
