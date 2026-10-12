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

//! `Env` for the proofs' environment. Bonds go through the same
//! `nonos_bt_bond::Store` the capsule opens, over memory.

use crate::env::TestEnv;
use crate::link::Env;
use nonos_bt_bond::BondRecord;
use nonos_bt_hogp::host::Battery;
use nonos_bt_hogp::report::Input;
use nonos_bt_wire::addr::{AddrType, BdAddr};

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
    fn battery(&mut self, battery: Battery) {
        self.batteries.push(battery);
    }
    fn show_passkey(&mut self, passkey: u32) {
        self.passkeys.push(passkey);
        self.shown.set(Some(passkey));
    }
    fn bonded(&mut self, kind: AddrType, addr: &BdAddr) -> Option<BondRecord> {
        self.bonds.find(kind, addr).cloned()
    }
    fn keep(&mut self, record: BondRecord) {
        self.bonds.keep(record).unwrap();
    }
    fn forget(&mut self, record: &BondRecord) {
        assert!(self.bonds.forget(record.identity_type, &record.identity).unwrap());
    }
}
