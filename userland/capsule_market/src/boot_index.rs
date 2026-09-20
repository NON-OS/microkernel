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
use nonos_libc::mk_getpid;

use crate::ingest::load_verified;
use crate::store::Store;
use crate::verify::Verifier;

/// Where an operator drops a catalogue newer than the built-in one.
const PATH: &[u8] = b"/nonos/marketplace/index.bin";

/// A catalogue of every listing a machine could offer is still small next to
/// one package.
const MAX: u32 = 8 << 20;

/// The catalogue this image shipped with. Empty when the build had none,
/// which reads as "no baseline" rather than as a failure.
static BASELINE: &[u8] = include_bytes!("../../../nonos-data/marketplace/index.bin");

pub fn load<V: Verifier>(store: &mut Store, verifier: &V) {
    if !BASELINE.is_empty() {
        take(store, verifier, BASELINE);
    }
    if let Ok(blob) = read_file(mk_getpid(), PATH, MAX) {
        take(store, verifier, &blob);
    }
}

fn take<V: Verifier>(store: &mut Store, verifier: &V, blob: &[u8]) {
    /*
     * A serial no newer than the one already held is the ordinary outcome, not
     * an error: it means no operator has published since this image was built.
     */
    if let Ok(v) = load_verified(blob, verifier, store.last_serial()) {
        store.install(v.index, v.signature_verified, v.publisher_signature_verified);
    }
}
