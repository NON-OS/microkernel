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

//! A FETCH 1 request's proof of work (plan/29, Tor's rule): a solution found
//! by walking nonces passes pow_check at its effort; any change to the field,
//! an effort under the agent's least, or another buyer key does not.

use nonos_equix::{HashX, Solution, Solver, MAX_SOLS};
use nonos_nox_chunks::{pow_challenge, pow_check, pow_ok, PowField, POW_FIELD_LEN};

const LISTING: [u8; 32] = [0x11; 32];
const PK: [u8; 32] = [0x33; 32];
const EFFORT: u32 = 32;

/// The first nonce, from zero up, with a solution meeting EFFORT.
fn solve() -> PowField {
    let mut solver = Solver::new().unwrap();
    for i in 0u32..512 {
        let mut nonce = [0u8; 16];
        nonce[..4].copy_from_slice(&i.to_le_bytes());
        let c = pow_challenge(&LISTING, &PK, &nonce, EFFORT);
        let Some(h) = HashX::new(&c) else { continue };
        let mut out = [Solution::default(); MAX_SOLS];
        let n = solver.solve(&h, &mut out);
        if let Some(s) = out[..n].iter().map(|s| s.to_bytes()).find(|s| pow_ok(&c, s, EFFORT)) {
            return PowField { nonce, effort: EFFORT, solution: s };
        }
    }
    panic!("no nonce in 512 met effort {EFFORT}");
}

#[test]
fn a_solution_meeting_its_effort_passes_and_nothing_else_does() {
    let f = solve();
    let field = f.encode();
    assert_eq!(field.len(), POW_FIELD_LEN);
    assert_eq!(PowField::parse(&field), Some(f));
    assert!(pow_check(&LISTING, &PK, &field, EFFORT));
    assert!(!pow_check(&LISTING, &PK, &field, EFFORT + 1), "under the agent's least");
    assert!(!pow_check(&LISTING, &[0x34; 32], &field, EFFORT), "another buyer key");
    assert!(!pow_check(&[0x12; 32], &PK, &field, EFFORT), "another listing");
    for at in [0, 15, 19, 20, 35] {
        let mut t = field;
        t[at] ^= 1;
        assert!(!pow_check(&LISTING, &PK, &t, 1), "byte {at} changed");
    }
    assert!(!pow_check(&LISTING, &PK, &field[..35], 1), "short field");
}

#[test]
fn the_challenge_and_the_effort_rule() {
    let c = pow_challenge(&LISTING, &PK, &[7; 16], 0x0102_0304);
    assert_eq!((c.len(), &c[..32], &c[32..64]), (84, &LISTING[..], &PK[..]));
    assert_eq!((&c[64..80], &c[80..]), (&[7u8; 16][..], &[1u8, 2, 3, 4][..]), "effort big endian");
    let f = solve();
    let c = pow_challenge(&LISTING, &PK, &f.nonce, f.effort);
    assert!(pow_ok(&c, &f.solution, 0) && pow_ok(&c, &f.solution, 1), "less effort always fits");
}
