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

//! The link's environment in the capsule: the uptime clock, the input ring
//! through the USB HID driver's code, the log, the random source, the desk.

use super::desk::Desk;
use super::say::{digits, say};
use crate::hid::Keys;
use crate::link::Env;
use crate::nbtc::Part;
use nonos_bt_bond::BondRecord;
use nonos_bt_hogp::{host::Battery, report::Input};
use nonos_bt_wire::addr::{AddrType, BdAddr};
use nonos_libc::mk_uptime_ms;

pub struct CapsuleEnv {
    pub keys: Keys,
    pub desk: Desk,
}

impl Env for CapsuleEnv {
    fn now_ms(&mut self) -> u64 {
        mk_uptime_ms().max(0) as u64
    }
    fn input(&mut self, input: Input, authenticated: bool) {
        self.keys.on(input, crate::link::trust::source_flags(authenticated));
    }
    fn tick(&mut self, now_ms: u64) {
        self.keys.tick(now_ms);
    }
    fn say(&mut self, words: &[&[u8]]) {
        say(words);
    }
    fn show_passkey(&mut self, passkey: u32) {
        say(&[b"to pair, type ", &digits(u64::from(passkey), 6), b" on the keyboard, then Enter"]);
        self.desk.board().ask(Part::Show(passkey));
    }
    fn battery(&mut self, b: Battery) {
        let (n, p) = (digits(u64::from(b.instance) + 1, 1), digits(u64::from(b.percent), 1));
        say(&[b"battery ", &n, b": ", &p, b"%"]);
        self.desk.board().battery(b);
    }
    fn bonded(&mut self, kind: AddrType, addr: &BdAddr) -> Option<BondRecord> {
        self.desk.board().bonds.find(kind, addr).cloned()
    }
    fn any_bonds(&mut self) -> bool {
        !self.desk.board().bonds.book().bonds().is_empty()
    }
    fn keep(&mut self, record: BondRecord) {
        match self.desk.board().bonds.keep(record) {
            Ok(()) => say(&[b"bond kept"]),
            Err(_) => say(&[b"bond not kept: no room, or the store refused it"]),
        }
    }
    fn forget(&mut self, record: &BondRecord) {
        let gone = self.desk.board().bonds.forget(record.identity_type, &record.identity);
        if gone.is_err() {
            say(&[b"a bond the peer dropped could not be forgotten"]);
        }
    }
}
