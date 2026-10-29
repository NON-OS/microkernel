/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The model's WMT: HCI vendor command 0xfc6f carrying direction 1, op,
//! the flag and data's length, flag and data. A patch section is looked at
//! (flag 0, a zero and its 52-octet spec, whose first words are its load
//! address and length), answered "held" (2), "in progress" (1) while the
//! semaphore is held, or "not yet" (0); then its octets come in blocks of
//! at most 250, flagged 1, 2 ... 3. Function control 1 turns it on.

use super::model::Model;

impl Model {
    pub fn command(&mut self, c: &[u8]) {
        let framed = c.len() >= 8 && c[..2] == [0x6f, 0xfc] && c[2] as usize == c.len() - 3;
        if !framed || c[3] != 1 || u16::from_le_bytes([c[5], c[6]]) as usize != c.len() - 7 {
            return self.wrong("a WMT command badly framed");
        }
        if self.pending.is_some() {
            return self.wrong("a WMT command before the last was answered");
        }
        let (op, flag, data) = (c[4], c[7], &c[8..]);
        let answer = match (op, flag) {
            (0x01, 0) if data.len() == 53 && data[0] == 0 => self.look(&data[1..]),
            (0x01, 1..=3) => self.block(flag, data),
            (0x06, 0) if data == [1] && self.loading.is_none() => {
                self.func_on = true;
                return self.pending = Some(vec![0xe4, 7, 2, op, 3, 0, 0, 0x04, 0x04]);
            }
            _ => return self.wrong("a WMT command the chip has no meaning for"),
        };
        self.pending = Some(vec![0xe4, 5, 2, op, 1, 0, answer]);
    }

    fn look(&mut self, spec: &[u8]) -> u8 {
        let addr = u32::from_le_bytes(spec[..4].try_into().unwrap());
        let len = u32::from_le_bytes(spec[4..8].try_into().unwrap());
        if self.loading.is_some() {
            self.wrong("a section looked at while another was half sent");
        }
        if let Some(status) = self.odd {
            return status;
        }
        if self.patched.contains_key(&addr) {
            return 2;
        }
        if self.held > 0 {
            self.held -= 1;
            return 1;
        }
        self.loading = Some((addr, len as usize, Vec::new()));
        0
    }

    fn block(&mut self, flag: u8, data: &[u8]) -> u8 {
        let Some((addr, len, mut got)) = self.loading.take() else {
            self.wrong("patch octets with no section looked at");
            return 0;
        };
        let ends = got.len() + data.len() == len;
        let want = if got.is_empty() { 1 } else { 2 + ends as u8 };
        if flag != want || data.len() > 250 || got.len() + data.len() > len {
            self.wrong("a patch block out of its phase or its section");
        }
        got.extend_from_slice(data);
        self.blocks.push((flag, data.len()));
        match got.len() >= len {
            true => drop(self.patched.insert(addr, got)),
            false => self.loading = Some((addr, len, got)),
        }
        0
    }
}
