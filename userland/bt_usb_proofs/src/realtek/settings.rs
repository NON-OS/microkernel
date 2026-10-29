// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! Settings' Bluetooth page against the capsule's NBTC: the page's own
//! panel (capsule_settings' `bluetooth::panel`), every request framed by
//! the shared crate and answered by the capsule's `nbtc::answer` for the
//! caller Settings, in the order service/poll.rs asks them. `seen` keeps
//! the words the page's rows read after each look.

use super::panel::key::BtKey;
use super::panel::{act, controller, status, Panel, Reach, Step};
use crate::nbtc::who::Caller;
use crate::nbtc::{answer, Board};
use nonos_bt_bond::MemoryKeys;
use nonos_bt_proto::ops::*;
use nonos_bt_proto::{read_reply, request, Asked, REPLY_MAX};

type Take = fn(&mut Panel, &[u8]) -> bool;
type B = Board<MemoryKeys>;

pub struct Settings {
    pub panel: Panel,
    /// After each look: the Controller and Status rows' words, and the ask.
    pub seen: Vec<(&'static str, &'static str, Asked)>,
}

impl Settings {
    pub fn new() -> Self {
        Self { panel: Panel::new(), seen: Vec::new() }
    }

    fn call(&mut self, b: &mut B, op: u16, body: &[u8], now: u64) -> (i32, Vec<u8>) {
        self.panel.request_id += 1;
        let (mut frame, mut tx) = ([0u8; 64], [0u8; REPLY_MAX]);
        let n = request(&mut frame, op, self.panel.request_id, body).unwrap();
        let got = answer(b, Caller::Settings, &frame[..n], now, &mut tx);
        let (s, data) = read_reply(&tx[..got], op, self.panel.request_id).expect("an NBTC reply");
        (s, data.to_vec())
    }

    pub fn look(&mut self, b: &mut B, now: u64) {
        let asks: [(u16, Take); 5] = [
            (OP_LINK, Panel::took_link),
            (OP_PASSKEY, Panel::took_asked),
            (OP_HEALTH, Panel::took_health),
            (OP_BONDS, Panel::took_bonds),
            (OP_BATTERY, Panel::took_levels),
        ];
        for (op, take) in asks {
            let (s, data) = self.call(b, op, &[], now);
            assert_eq!(s, E_OK, "the capsule refused Settings op {op}");
            assert!(take(&mut self.panel, &data), "Settings could not read op {op}");
        }
        self.panel.reach = Reach::Answered;
        let p = &self.panel;
        self.seen.push((controller(p).0, status(p).0, p.asked));
    }

    /// A key pressed on the page, its request answered, and a look after.
    pub fn press(&mut self, b: &mut B, key: BtKey, now: u64) -> i32 {
        let Step::Ask(req) = act(&mut self.panel, key) else { panic!("{key:?} asked nothing") };
        let (s, _) = self.call(b, req.op, req.body(), now);
        self.panel.did(&req, s, now as i64);
        self.look(b, now);
        s
    }

    /// The passkey the page shows the user to type, if it shows one.
    pub fn shown_passkey(&self) -> Option<u32> {
        match self.panel.asked {
            Asked::Show(n) => Some(n),
            _ => None,
        }
    }
}
