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

//! NTRU draws the positions of every secret polynomial from the seeded RNG,
//! and refuses a key pair or an encapsulation when a draw fails.
//!
//! Only refusals are tested. The kernel's NTRU key generation never ends:
//! each secret has as many +1 as -1 coefficients, so it is zero at x = 1 and
//! has no inverse modulo x^N - 1, and the loop draws again forever (2,000,000
//! draws, about 2,400 tries, found none). That defect is older than this
//! change and is reported, not fixed, here.

use crate::crypto::pqc::ntru::{ntru_encaps, ntru_keygen, NTRU_N};
use crate::stand_in::fail_from;

#[test]
fn a_failed_draw_refuses_the_key_pair() {
    // A secret's positions take NTRU_N - 1 draws; fail at the first, inside
    // and at the last of the first secret.
    for from in [1, 400, NTRU_N as u64 - 1] {
        fail_from(Some(from));
        assert!(matches!(ntru_keygen(), Err("no randomness")), "draw {from}");
    }
}

#[test]
fn a_failed_draw_refuses_the_encapsulation() {
    // The message and the blinding polynomial are drawn before the key is
    // read, so any key will do. Fail in the first, then in the second.
    let pk = crate::crypto::pqc::ntru::NtruPublicKey { h: alloc::vec![0; NTRU_N] };
    for from in [1, NTRU_N as u64 - 1, NTRU_N as u64 + 5] {
        fail_from(Some(from));
        assert!(matches!(ntru_encaps(&pk), Err("no randomness")), "draw {from}");
    }
}
