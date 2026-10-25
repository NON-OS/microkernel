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

//! An escrow price is written exactly, never rounded (plan/27): the Shield
//! is asked to quote and fund that amount and no other.

use crate::order::fees_kind::exact;

const NOX: u128 = 1_000_000_000_000_000_000;

#[test]
fn whole_and_fractional_amounts_are_written_in_full() {
    assert_eq!(exact(0), "0");
    assert_eq!(exact(4 * NOX), "4");
    assert_eq!(exact(2_000 * NOX), "2000");
    assert_eq!(exact(NOX / 2), "0.5");
    assert_eq!(exact(4 * NOX + 1), "4.000000000000000001", "the last wei is kept");
    assert_eq!(exact(NOX + NOX / 100 * 7), "1.07");
    assert_eq!(exact(u128::MAX), "340282366920938463463.374607431768211455");
}
