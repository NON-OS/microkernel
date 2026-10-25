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

//! Many threads at once take turns through the C, and all succeed.

use super::refusals::counter;
use nonos_mldsa65::{keypair, sign, verify};

#[test]
fn threads_take_turns_and_all_succeed() {
    let workers: Vec<_> = (0..8u8)
        .map(|t| {
            std::thread::spawn(move || {
                let (pk, sk) = keypair(&mut counter(t)).unwrap();
                let sig = sign(&sk, &[t; 99], &mut counter(t + 100)).unwrap();
                (0..5).all(|_| verify(&pk, &[t; 99], &sig))
            })
        })
        .collect();
    assert!(workers.into_iter().all(|w| w.join().unwrap()));
}
