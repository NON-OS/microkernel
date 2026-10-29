// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The proofs' environment with the capsule's NBTC board behind it, as
//! run/env.rs and run/env_desk.rs keep it: bonds in the board's store, a
//! passkey and a battery level told to the board, and Settings' panel
//! looking at the board each time the link tells it something.

use std::cell::Cell;
use std::rc::Rc;

use super::settings::Settings;
use crate::env::TestEnv;
use crate::link::Env;
use crate::nbtc::{Board, Part};
use nonos_bt_bond::{BondRecord, MemoryKeys, Store};
use nonos_bt_hogp::host::Battery;
use nonos_bt_hogp::report::Input;
use nonos_bt_wire::addr::{AddrType, BdAddr};

pub struct BoardEnv {
    pub t: TestEnv,
    pub board: Board<MemoryKeys>,
    pub settings: Settings,
}

impl BoardEnv {
    pub fn new(shown: Rc<Cell<Option<u32>>>) -> Self {
        let board = Board::new(Store::open(MemoryKeys::default()).unwrap());
        Self { t: TestEnv::new(shown), board, settings: Settings::new() }
    }

    /// Settings looks, as its page does every poll.
    pub fn look(&mut self) {
        self.settings.look(&mut self.board, self.t.now);
    }
}

impl Env for BoardEnv {
    fn now_ms(&mut self) -> u64 {
        self.t.now_ms()
    }
    fn input(&mut self, input: Input, authenticated: bool) {
        self.t.input(input, authenticated);
    }
    fn tick(&mut self, now_ms: u64) {
        self.t.tick(now_ms);
    }
    fn say(&mut self, words: &[&[u8]]) {
        self.t.say(words);
    }
    fn battery(&mut self, b: Battery) {
        self.board.battery(b);
        self.look();
    }
    fn show_passkey(&mut self, passkey: u32) {
        self.t.passkeys.push(passkey);
        self.board.ask(Part::Show(passkey));
        self.look();
        // The user reads it off the page, and types it on the keyboard.
        self.t.shown.set(self.settings.shown_passkey());
    }
    fn bonded(&mut self, kind: AddrType, addr: &BdAddr) -> Option<BondRecord> {
        self.board.bonds.find(kind, addr).cloned()
    }
    fn any_bonds(&mut self) -> bool {
        !self.board.bonds.book().bonds().is_empty()
    }
    fn keep(&mut self, record: BondRecord) {
        self.board.bonds.keep(record).unwrap();
    }
    fn forget(&mut self, record: &BondRecord) {
        self.board.bonds.forget(record.identity_type, &record.identity).unwrap();
    }
}
