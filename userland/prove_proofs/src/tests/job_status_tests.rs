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

//! A failed job's status is the kernel's errno, or PROVER for a return that
//! is no errno: zero, positive, or wider than an i32.

use nonos_proof_abi::wire::PROVER;

use crate::job_status::failed;

#[test]
fn an_errno_passes_and_nothing_else_does() {
    assert_eq!(failed(-3), -3, "ESRCH from MkSealRead live");
    assert_eq!(failed(-1), -1);
    assert_eq!(failed(i64::from(i32::MIN)), i32::MIN);
    for n in [0, 1, 33, i64::from(i32::MIN) - 1, -(1 << 32) - 3, i64::MIN, i64::MAX] {
        assert_eq!(failed(n), PROVER, "{n}");
    }
}
