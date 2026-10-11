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

//! The gas limits the OS encodes with, pinned.

#[test]
fn approve_and_open_limits_are_the_plans() {
    assert_eq!((nonos_nox_config::APPROVE_GAS, nonos_nox_config::OPEN_GAS), (60_000, 200_000));
}

#[test]
fn the_fee_caps_are_1000_and_10_gwei() {
    let gwei = 1_000_000_000u128;
    assert_eq!(nonos_nox_config::MAX_FEE_PER_GAS, 1_000 * gwei);
    assert_eq!(nonos_nox_config::MAX_PRIORITY_FEE_PER_GAS, 10 * gwei);
}
