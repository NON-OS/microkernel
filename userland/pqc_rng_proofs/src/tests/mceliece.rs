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

//! McEliece draws its Goppa polynomial, its support, its permutation and
//! each encapsulation's error positions from the seeded RNG, and refuses
//! when a draw fails.
//!
//! A whole key pair is not tested. Once the polynomial and the support are
//! drawn, the kernel's `polynomial_gcd` indexes a zero polynomial's degree,
//! `usize::MAX` (`goppa/poly.rs`), and panics. That defect is older than this
//! change and is reported, not fixed, here.

use crate::crypto::pqc::mceliece::{mceliece_encaps, mceliece_keygen, McEliecePublicKey};
use crate::stand_in::fail_from;

#[test]
fn a_failed_draw_refuses_the_key_pair() {
    // The first draw is the polynomial's; by the 2000th the search has gone
    // through many polynomials or on into the support.
    for from in [1, 65, 2000] {
        fail_from(Some(from));
        assert!(matches!(mceliece_keygen(), Err("no randomness")), "draw {from}");
        assert_eq!(crate::stand_in::draws(), from, "it stopped at the failed draw");
    }
}

#[test]
fn a_failed_draw_refuses_the_encapsulation() {
    // The error positions are drawn before the key is read, so any key will do.
    let pk = McEliecePublicKey { t_matrix: alloc::vec![0u8; 0] };
    for from in [1, 32, 64] {
        fail_from(Some(from));
        assert!(matches!(mceliece_encaps(&pk), Err("no randomness")), "draw {from}");
    }
}
