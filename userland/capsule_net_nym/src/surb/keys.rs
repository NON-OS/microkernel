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

use spin::Mutex;

use super::store::{KeyStore, DIGEST_BYTES};
use super::types::SURB_KEY_BYTES;
use crate::crypto::hash::blake3;

/// Keys of the reply blocks handed out, by the digest a reply names them by.
/// See `store` for why they are kept until answered rather than in a ring.
static STORE: Mutex<KeyStore> = Mutex::new(KeyStore::new());

/// Keep a key so a reply sealed under it can be opened.
///
/// The digest is worked out once, here, rather than for every held key on
/// every packet that arrives: that was 512 hashes a packet, and a page is
/// hundreds of packets.
pub fn remember(key: [u8; SURB_KEY_BYTES]) {
    let mut digest = [0u8; DIGEST_BYTES];
    // A key whose digest cannot be worked out could never be matched, so it
    // is not kept.
    if blake3(&key, &mut digest).is_err() {
        return;
    }
    STORE.lock().remember(key, digest);
}

/// The key a reply names by `digest`, taken out of the store.
pub fn take(digest: &[u8]) -> Option<[u8; SURB_KEY_BYTES]> {
    let mut store = STORE.lock();
    if store.is_empty() {
        return None;
    }
    store.take(digest)
}

/// Keys held for blocks not yet answered on.
pub fn held() -> usize {
    STORE.lock().len()
}
