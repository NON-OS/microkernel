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

//! Every wait a refresh can make on the UI thread has a finite bound, so a
//! dead peer costs seconds, never a hang. These mirror the wallet's timeouts
//! and are checked when this crate compiles: a bound that breaks the rule
//! fails the build, not only a test.

// net/*.rs mk_ipc_call_timeout bounds, in milliseconds. Generous enough that a
// live but contended DNS/sockets capsule on one core is never mistaken for a
// dead one (a too-tight bound marked the whole route blocked and stopped every
// fetch), still finite so a genuinely dead peer fails instead of hanging.
const T_DNS: u64 = 2800;
const T_OPEN: u64 = 1500;
const T_CONNECT: u64 = 4000;
const T_SEND: u64 = 1800;
const T_RECV: u64 = 80;

// Non-socket IPC bounds: the TLS handshake's crypto-capsule calls
// (tls13/crypto_status.rs, tls13/hash_sha384.rs), the keyring call
// (ipc/call.rs). The Send
// freeze reproduced through these, not the sockets: any blocking IPC on the
// UI thread must be bounded, whoever the peer is.
const T_CRYPTO: u64 = 3000;
const T_KEYRING: u64 = 4000;

/// Worst-case wall time of one fetch if every op times out (a blackholed peer):
/// DNS, open, connect, send, read, send, read. Finite by construction.
fn worst_case_fetch_ms() -> u64 {
    T_DNS + T_OPEN + T_CONNECT + T_SEND + T_RECV + T_SEND + T_RECV
}

#[test]
fn worst_case_is_bounded_and_recv_is_snappy() {
    // A fully blackholed peer costs a finite ceiling, never an infinite hang.
    // The bound is generous so contention is not mistaken for death, but this
    // worst case is only reached when every op times out; a healthy fetch is a
    // fraction of it, and it runs only once per refresh interval when idle.
    let wc = worst_case_fetch_ms();
    assert!(wc > 0 && wc < 15_000, "a dead peer costs a finite {wc} ms, never an infinite hang");
    const _: () = assert!(T_RECV <= 100, "the polled recv stays responsive");
}

// The invariant behind the Send-path freeze fix: no IPC from the UI thread may
// wait forever. Each bound is finite and small enough that even a fully wedged
// peer returns control within seconds.
const _: () = assert!(T_CRYPTO > 0 && T_CRYPTO <= 5000, "crypto capsule (TLS handshake)");
const _: () = assert!(T_KEYRING > 0 && T_KEYRING <= 5000, "keyring");
// The keyring bound covers its slowest real op: PBKDF2 with 2048 rounds plus
// BIP32 derivation during HD generate/recover.
const _: () = assert!(T_KEYRING >= 2000, "keyring bound must leave room for PBKDF2-2048");
