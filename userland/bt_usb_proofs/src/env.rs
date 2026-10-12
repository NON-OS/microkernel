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

//! The capsule's environment as the proofs play it: a clock that moves a
//! millisecond each time it is read, the input ring, the log and the battery levels as lists,
//! the user who types the passkey shown, a fixed random stream, and the
//! bond store over memory, as the capsule keeps it without L33.

use std::cell::Cell;
use std::rc::Rc;

use crate::env_desk::User;
use crate::smp_sim::Rng;
use nonos_bt_bond::{MemoryKeys, Store};
use nonos_bt_hogp::host::Battery;
use nonos_bt_hogp::report::Input;
use nonos_bt_smp::pairing::Random;

pub struct TestEnv {
    pub now: u64,
    pub input: Vec<Input>,
    pub lines: Vec<String>,
    pub batteries: Vec<Battery>,
    pub passkeys: Vec<u32>,
    pub ticks: u64,
    /// Shared with the air: the peer's user types what is shown here.
    pub shown: Rc<Cell<Option<u32>>>,
    pub rng: Rng,
    pub bonds: Store<MemoryKeys>,
    pub user: User,
}

impl TestEnv {
    pub fn new(shown: Rc<Cell<Option<u32>>>) -> Self {
        let rng = Rng(0x2545_f491_4f6c_dd1d);
        let (input, lines, passkeys) = (Vec::new(), Vec::new(), Vec::new());
        let bonds = Store::open(MemoryKeys::default()).unwrap();
        let batteries = Vec::new();
        let user = User::new();
        Self { now: 0, input, lines, batteries, passkeys, ticks: 0, shown, rng, bonds, user }
    }

    pub fn said(&self, line: &str) -> bool {
        self.lines.iter().any(|l| l == line)
    }
}

impl Random for TestEnv {
    fn fill(&mut self, out: &mut [u8]) {
        self.rng.fill(out);
    }
}
