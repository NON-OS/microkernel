/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! The model's default endpoint: HCI commands (class request 0x20), the
//! WMT answer pipe (vendor request 0x01, wValue 0x30), the chip's register
//! read (0xC0/0x63) and the USB wrapper's read (0xDE/0x01) and write
//! (0x5E/0x02), each four octets. Anything else breaks the model.

use super::model::Model;
use super::port::Port;
use super::reg::{BT_MISC, RST_DONE, SUBSYS_RST};
use nonos_bt_hci::usb::Setup;

fn reg(s: Setup) -> u32 {
    (s.value as u32) << 16 | s.index as u32
}

impl Model {
    fn reg_in(&mut self, s: Setup) -> Option<u32> {
        let at = reg(s);
        let reset_begun = self.writes.iter().any(|w| w.0 == SUBSYS_RST);
        match (s.request_type, s.request) {
            (0xde, 0x01) if at == BT_MISC && reset_begun => {
                self.misc_looks += 1;
                let done = self.reset_takes.is_some_and(|n| self.misc_looks >= n);
                Some(if done { RST_DONE } else { 0 })
            }
            (0xc0, 0x63) | (0xde, 0x01) => Some(*self.regs.get(&at).unwrap_or(&0)),
            _ => None,
        }
    }
}

impl Port for Model {
    fn control_out(&mut self, s: Setup, data: &[u8]) -> Result<(), i32> {
        match (s.request_type, s.request, data.len()) {
            (0x20, 0, _) => self.command(data),
            (0x5e, 0x02, 4) => {
                let v = u32::from_le_bytes(data.try_into().unwrap());
                self.writes.push((reg(s), v));
                self.regs.insert(reg(s), v);
            }
            _ => self.wrong("a transfer out the chip has no meaning for"),
        }
        Ok(())
    }
    fn control_in(&mut self, s: Setup, out: &mut [u8]) -> Result<usize, i32> {
        if (s.request_type, s.request, s.value, s.index) == (0xc0, 0x01, 0x30, 0) {
            return Ok(self.answer(out));
        }
        let Some(v) = self.reg_in(s).filter(|_| out.len() == 4) else {
            self.wrong("a transfer in the chip has no meaning for");
            return Err(-5);
        };
        out.copy_from_slice(&v.to_le_bytes());
        Ok(4)
    }
    fn wait(&mut self, ms: u64) {
        self.clock_ms += ms;
    }
    fn file(&mut self, kind: u64, at: u64, out: &mut [u8]) -> i64 {
        let Some(f) = self.files.get(&kind) else { return -2 };
        let rest = f.get(at as usize..).unwrap_or(&[]);
        let n = rest.len().min(out.len());
        out[..n].copy_from_slice(&rest[..n]);
        n as i64
    }
    fn sha256(&mut self, data: &[u8]) -> Option<[u8; 32]> {
        self.digests += 1;
        Some(nonos_hash::sha256(data))
    }
}
