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

/*
 * Verify once per boot, not once per spawn.
 *
 * Signature verification and STARK verification are pure functions of
 * their inputs. This remembers the inputs of each one that returned
 * true, as a BLAKE3 key over every byte the verifier read, and answers
 * true again for exactly those bytes. Everything around the verifiers
 * still runs on every spawn: decoding, the revocation lists, the validity
 * windows, the ceiling, the namespace and endpoint checks, and the
 * measurement of the image, which is part of every attestation key, so
 * a changed image misses. A changed trust anchor or root is a different
 * public key or root in the key and misses too; `forget_all` also runs
 * whenever the enrolled roots change.
 *
 * Only acceptances are kept, in RAM, a bounded number, oldest out first.
 * A refusal is never remembered, so a remembered key can only ever turn a
 * verify the kernel already passed into the same answer.
 */

mod key;
mod table;

use spin::Mutex;

pub(crate) use key::key;
use table::Table;

/// Acceptances kept: enough for every capsule an image carries and the
/// libraries of the apps a session runs.
const KEPT: usize = 256;

static ACCEPTED: Mutex<Table<KEPT>> = Mutex::new(Table::new());

pub(crate) fn known(k: &[u8; 32]) -> bool {
    ACCEPTED.lock().contains(k)
}

pub(crate) fn remember(k: [u8; 32]) {
    ACCEPTED.lock().insert(k);
}

/// Drops every acceptance; the next verify of anything runs in full.
pub fn forget_all() {
    ACCEPTED.lock().clear();
}

/*
 * One signature, by the algorithm the certificate or manifest names. The
 * domain string keeps these keys apart from attestation keys.
 */
pub(crate) fn signature(
    alg: crate::crypto::asymmetric::alg_id::AlgId,
    pubkey: &[u8],
    msg: &[u8],
    sig: &[u8],
) -> Result<bool, crate::crypto::asymmetric::alg_id::AlgIdError> {
    let k = key(b"nonos.admission.signature.v1", &[&[alg as u8], pubkey, msg, sig]);
    if known(&k) {
        return Ok(true);
    }
    let verdict = crate::crypto::asymmetric::alg_id::verify(alg, pubkey, msg, sig);
    if let Ok(true) = verdict {
        remember(k);
    }
    verdict
}
