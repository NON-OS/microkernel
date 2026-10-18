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

//! Version 3's last settle: a nonce is kept only with its chain.

use super::persist::sample;
use nonos_nox_meter::persist::{decode, encode};
use nonos_nox_meter::MeterError;

#[test]
fn the_last_settle_needs_its_chain() {
    let mut b = encode(&sample()).unwrap();
    assert_eq!(decode(&b).unwrap().last_settle, Some((1, 7)));
    b[8..16].copy_from_slice(&0u64.to_le_bytes());
    assert_eq!(decode(&b), Err(MeterError::Corrupt), "a nonce on no chain");
}
