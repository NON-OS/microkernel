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

//! A verified TLS connection with keys the proofs chose, and its records.

use crate::browser::fetch::types::TlsCtx;
use crate::browser::tls13::flight::ClientFlight;
use crate::browser::tls13::TrafficKeys;

/// Application keys chosen for the proofs, for records sealed by hand.
pub fn app_keys() -> TrafficKeys {
    let (client_secret, server_secret) = ([1; 32], [2; 32]);
    let (client_key, client_iv, server_key, server_iv) = ([3; 32], [4; 12], [5; 32], [6; 12]);
    let handshake_secret = [0; 32];
    TrafficKeys {
        suite: 0x1303,
        handshake_secret,
        client_secret,
        server_secret,
        client_key,
        client_iv,
        server_key,
        server_iv,
    }
}

/// A verified TLS context holding `app_keys`, as a handshake leaves one.
pub fn settled() -> TlsCtx {
    let cf = ClientFlight { record: Vec::new(), handshake: Vec::new(), private: [0; 32] };
    let mut tls = TlsCtx::new(cf, 20260601000000);
    tls.settle(app_keys());
    tls
}

/// `body` as server application records of `size` bytes, from `seq`.
pub fn server_records(body: &[u8], size: usize, seq: u64) -> Vec<u8> {
    let k = app_keys();
    let mut out = Vec::new();
    for (i, part) in body.chunks(size).enumerate() {
        let n = seq + i as u64;
        let record = crate::browser::tls13::record_seal::seal(
            k.suite,
            &k.server_key,
            &k.server_iv,
            n,
            23,
            part,
        );
        out.extend_from_slice(&record.expect("seal"));
    }
    out
}
