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

//! The refusals that come after the reset, while the engines are known
//! stopped but before the controller is trusted: neither may turn bus
//! mastering on on its way out, or a controller left with nothing to do is
//! handed back able to reach memory.

use crate::constants::{GCAP, VMAJ};
use crate::error::HdaError;
use crate::proofs::fixtures::{alc236_hp, sim_codec};
use crate::proofs::lifecycle::attempt::attempt;
use crate::proofs::lifecycle::board::{board, LOW};
use crate::proofs::lifecycle::claim_tests::bus_master_on;
use crate::proofs::lifecycle::given_back::quiesced_then_released;

#[test]
fn rings_above_4_gib_are_refused_without_bus_mastering() {
    let high = 1u64 << 32;
    let b = board(vec![(0, sim_codec(&alc236_hp()))], &[high, high + 0x10_0000]);
    b.sim.bar.present16(GCAP as usize, 0x4400);
    let (r, log) = attempt(&b);
    assert_eq!(r.err(), Some(HdaError::DmaOutOfReach));
    assert!(bus_master_on(&log).is_empty(), "bus mastering went on for rings it cannot reach");
    quiesced_then_released(&log);
}

#[test]
fn a_controller_of_no_known_version_is_refused_without_bus_mastering() {
    let b = board(vec![(0, sim_codec(&alc236_hp()))], &LOW);
    b.sim.bar.present8(VMAJ as usize, 0);
    let (r, log) = attempt(&b);
    assert_eq!(r.err(), Some(HdaError::UnsupportedController));
    assert!(bus_master_on(&log).is_empty(), "bus mastering went on for a refused controller");
    quiesced_then_released(&log);
}
