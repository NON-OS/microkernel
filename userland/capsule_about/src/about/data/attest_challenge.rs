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

//! The nonce sent with an attestation request.

use nonos_libc::{mk_getpid, mk_time_millis, mk_uptime_ms};

// A challenge this capsule mixes from the clock, the uptime and its own pid. It
// is not from the entropy pool: this window holds no Crypto capability, and
// reaching for one to seed a display would be a worse trade than the weaker
// challenge. It is fresh enough for its one local job, which is to prove the TPM
// signed this request rather than replaying an earlier one. A remote verifier
// must supply its own, and the card says so.
pub(super) fn challenge() -> [u8; 32] {
    let mut c = [0u8; 32];
    let millis = mk_time_millis().max(0) as u64;
    let uptime = mk_uptime_ms().max(0) as u64;
    let pid = mk_getpid() as u64;
    c[0..8].copy_from_slice(&millis.to_be_bytes());
    c[8..16].copy_from_slice(&uptime.to_be_bytes());
    c[16..24].copy_from_slice(&pid.to_be_bytes());
    c[24..32].copy_from_slice(&(millis ^ uptime.rotate_left(17)).to_be_bytes());
    c
}
