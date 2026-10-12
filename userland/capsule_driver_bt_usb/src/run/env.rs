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

//! The link's environment in the capsule: the uptime clock, the input
//! ring through the USB HID driver's own code, the debug channel, and the
//! kernel's random source.

use super::say::{digits, say};
use crate::hid::Keys;
use crate::link::Env;
use nonos_bt_hogp::report::Input;
use nonos_bt_smp::pairing::Random;
use nonos_libc::{crypto_random, mk_exit, mk_uptime_ms};

const EXIT_NO_RANDOM: i32 = 3;

#[derive(Default)]
pub struct CapsuleEnv {
    pub keys: Keys,
}

impl Random for CapsuleEnv {
    /// Keys and nonces come from here. If the kernel's source fails there
    /// is nothing safe to pair with, so the capsule stops rather than go on
    /// with values an observer could guess.
    fn fill(&mut self, out: &mut [u8]) {
        if crypto_random(out.as_mut_ptr(), out.len()) < 0 {
            say(&[b"the kernel's random source failed; stopping"]);
            mk_exit(EXIT_NO_RANDOM);
        }
    }
}

impl Env for CapsuleEnv {
    fn now_ms(&mut self) -> u64 {
        mk_uptime_ms().max(0) as u64
    }
    fn input(&mut self, input: Input) {
        self.keys.on(input);
    }
    fn tick(&mut self, now_ms: u64) {
        self.keys.tick(now_ms);
    }
    fn say(&mut self, words: &[&[u8]]) {
        say(words);
    }
    fn show_passkey(&mut self, passkey: u32) {
        let n = digits(u64::from(passkey), 6);
        say(&[b"to pair, type ", &n, b" on the keyboard, then Enter"]);
    }
}
