// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The user's part through the board, as run/env_desk.rs gives it, with
//! Settings looking each time the link tells it something.

use super::board_env::BoardEnv;
use crate::link::{Answer, Asked, Desk};
use crate::nbtc::Part;
use nonos_bt_smp::pairing::Random;

impl Random for BoardEnv {
    fn fill(&mut self, out: &mut [u8]) {
        self.t.fill(out);
    }
}

impl Desk for BoardEnv {
    fn ready(&mut self, keyboard: bool, mouse: bool, keys_post: bool) {
        self.board.ready(keyboard, mouse, keys_post);
        self.look();
    }
    fn linked(&mut self, identity_type: u8, identity: [u8; 6], fresh: bool) {
        self.board.linked((identity_type, identity), fresh);
        self.look();
    }
    fn asked(&mut self, asked: Asked) {
        let part = match asked {
            Asked::Passkey => Part::Enter,
            Asked::Number(n) => Part::Confirm(n),
        };
        self.board.ask(part);
        self.look();
    }
    fn answer(&mut self) -> Option<Answer> {
        self.board.take_answer()
    }
    fn may_pair(&mut self) -> bool {
        self.board.may_pair(self.t.now)
    }
    fn hang_up(&mut self) -> bool {
        std::mem::take(&mut self.board.drop_link)
    }
    fn radio_off(&mut self) -> bool {
        !self.board.scan
    }
}
