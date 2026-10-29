// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! Each command answered. The chip counts fragments on its own, as btrtl.c
//! numbers them: 0 up to 0x7f, then back to 1. A fragment whose index is
//! not the count, that is not 252 octets but the last, or that comes after
//! the last, is refused with Invalid HCI Command Parameters (0x12) and the
//! download is broken; so is a last fragment that does not complete the
//! image the ROM accepts.

use super::wire::complete;
use super::Rtl;
use crate::firmware::CONFIG;
use nonos_bt_wire::hci::opcode::READ_LOCAL_VERSION;
use nonos_bt_wire::hci::parse_command;

const INVALID: u8 = 0x12;

impl Rtl {
    pub(super) fn answer(&mut self, packet: &[u8]) -> Vec<u8> {
        let (op, p) = parse_command(packet).expect("a malformed command");
        self.sent.push((op.0, p.to_vec()));
        match op.0 {
            o if o == READ_LOCAL_VERSION.0 => complete(o, 0, &self.version()),
            0xfc6d => complete(0xfc6d, 0, &self.rom_reply.clone()),
            0xfc20 => {
                let status = self.fragment(p);
                let ret = self.download_reply.clone().unwrap_or_else(|| p[..1].to_vec());
                complete(0xfc20, status, &ret)
            }
            o => complete(o, 0, &[]),
        }
    }

    fn fragment(&mut self, p: &[u8]) -> u8 {
        let (index, data) = (p[0], &p[1..]);
        let last = index & 0x80 != 0;
        let wrong = if self.broken.is_some() || self.running.is_some() {
            Some("a fragment after the download ended")
        } else if index & 0x7f != self.counted {
            Some("a fragment out of order")
        } else if !last && data.len() != 252 {
            Some("a fragment short of 252 octets before the last")
        } else {
            None
        };
        if let Some(why) = wrong {
            self.broken = Some(why);
            return INVALID;
        }
        if self.refuse_at == Some(self.place) {
            return 0x01;
        }
        self.got.extend_from_slice(data);
        self.place += 1;
        self.counted = if self.counted == 0x7f { 1 } else { self.counted + 1 };
        if last {
            if self.got != self.accepts {
                self.broken = Some("an image this ROM does not accept");
                return INVALID;
            }
            self.running = Some(self.carried(CONFIG.len()));
        }
        0
    }
}
