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
//! millisecond each time it is read, the input ring and the log as lists,
//! the user who types the passkey shown, and a fixed random stream.

use std::cell::Cell;
use std::rc::Rc;

use crate::link::Env;
use crate::smp_sim::Rng;
use nonos_bt_hogp::report::Input;
use nonos_bt_smp::pairing::Random;

pub struct TestEnv {
    pub now: u64,
    pub input: Vec<Input>,
    pub lines: Vec<String>,
    pub passkeys: Vec<u32>,
    pub ticks: u64,
    /// Shared with the air: the peer's user types what is shown here.
    pub shown: Rc<Cell<Option<u32>>>,
    pub rng: Rng,
}

impl TestEnv {
    pub fn new(shown: Rc<Cell<Option<u32>>>) -> Self {
        let rng = Rng(0x2545_f491_4f6c_dd1d);
        let (input, lines, passkeys) = (Vec::new(), Vec::new(), Vec::new());
        Self { now: 0, input, lines, passkeys, ticks: 0, shown, rng }
    }
}

impl Random for TestEnv {
    fn fill(&mut self, out: &mut [u8]) {
        self.rng.fill(out);
    }
}

impl Env for TestEnv {
    fn now_ms(&mut self) -> u64 {
        self.now += 1;
        self.now
    }
    fn input(&mut self, input: Input) {
        self.input.push(input);
    }
    fn tick(&mut self, _now_ms: u64) {
        self.ticks += 1;
    }
    fn say(&mut self, words: &[&[u8]]) {
        self.lines.push(String::from_utf8_lossy(&words.concat()).into_owned());
    }
    fn show_passkey(&mut self, passkey: u32) {
        self.passkeys.push(passkey);
        self.shown.set(Some(passkey));
    }
}
