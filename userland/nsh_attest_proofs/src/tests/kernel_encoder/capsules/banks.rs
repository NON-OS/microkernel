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

//! The banks the kernel's events carry, from those a firmware log declares
//! (`src/security/capsule_measure/declared.rs`): all of them or none.

use crate::kernel_measure::algs::{ALG_SHA1, ALG_SHA256, ALG_SHA384, ALG_SHA512};
use crate::kernel_measure::declared::from_declared;

fn declared(banks: &[(u16, usize)]) -> Option<Vec<u16>> {
    from_declared(banks.len(), |alg| banks.iter().find(|b| b.0 == alg).map(|b| b.1))
}

#[test]
fn every_declared_bank_is_carried_in_a_fixed_order() {
    assert_eq!(declared(&[(ALG_SHA256, 32)]), Some(vec![ALG_SHA256]));
    let pc = declared(&[(ALG_SHA256, 32), (ALG_SHA1, 20)]);
    assert_eq!(pc, Some(vec![ALG_SHA1, ALG_SHA256]));
    let all = [(ALG_SHA512, 64), (ALG_SHA384, 48), (ALG_SHA256, 32), (ALG_SHA1, 20)];
    assert_eq!(declared(&all), Some(vec![ALG_SHA1, ALG_SHA256, ALG_SHA384, ALG_SHA512]));
}

#[test]
fn a_bank_the_kernel_cannot_extend_refuses_them_all() {
    assert_eq!(declared(&[(ALG_SHA256, 32), (0x0012, 32)]), None, "an unknown bank");
    assert_eq!(declared(&[(ALG_SHA256, 32), (ALG_SHA1, 32)]), None, "SHA-1 at 32 bytes");
    assert_eq!(declared(&[(ALG_SHA256, 20)]), None, "SHA-256 at 20 bytes");
    assert_eq!(declared(&[(ALG_SHA384, 48)]), None, "no SHA-256");
    assert_eq!(declared(&[]), None, "no bank");
}
