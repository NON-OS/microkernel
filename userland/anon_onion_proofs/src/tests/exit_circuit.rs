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

//! Circuits Net opens to its exits, one hop of real onion crypto each, with
//! the far end's keys mirrored as a relay runs them.

use crate::circuit::{Circuit, CircuitStage, Hop};

use super::exit_net::Net;

/// Relays 0 to 7: the web and nothing else (0, 3, 4, 6, 7), the web and
/// Monero's ports (1), a reject line that lets most out (2), and 18081 alone,
/// no web port (5). Nothing lets port 25 out.
pub const LINES: [&[u8]; 8] = [
    b"accept 80,443",
    b"accept 80,443,18080-18089",
    b"reject 25,119,135-139,445",
    b"accept 80,443",
    b"accept 80,443",
    b"accept 18081",
    b"accept 80,443",
    b"accept 80,443",
];

/// Df | Db | Kf | Kb with each pair swapped: the keys the far end runs.
fn mirror(k: &[u8; 72]) -> [u8; 72] {
    let mut m = *k;
    m[..20].copy_from_slice(&k[20..40]);
    m[20..40].copy_from_slice(&k[..20]);
    m[40..56].copy_from_slice(&k[56..72]);
    m[56..72].copy_from_slice(&k[40..56]);
    m
}

impl Net {
    /// Open a general circuit ending at relay `exit`, by id.
    pub fn circuit(&mut self, exit: usize) -> u32 {
        let id = 0x8000_0100 + self.far.len() as u32;
        let n = self.state.relays.len();
        let path = [exit + 1, exit + 2, exit].map(|i| self.state.relays[i % n].clone());
        self.state.circuits.push(Circuit::new(id, path.to_vec()));
        self.bring_up()
    }

    /// Bring the last circuit in the table up, as if its hops had answered:
    /// one drawn by circuit_tick, or one just pushed.
    pub fn bring_up(&mut self) -> u32 {
        let circuit = self.state.circuits.last_mut().expect("one drawn");
        let keys: [u8; 72] = core::array::from_fn(|i| (i as u8).wrapping_mul(7) ^ circuit.id as u8);
        (circuit.stage, circuit.opened_at, circuit.pending) = (CircuitStage::Open, self.now, None);
        circuit.hops = std::vec![Hop::new(&keys)];
        self.far.push((circuit.id, Hop::new(&mirror(&keys))));
        circuit.id
    }
}
