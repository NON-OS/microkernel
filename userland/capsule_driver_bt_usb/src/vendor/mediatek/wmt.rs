/*
 * NONOS Operating System
 * Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)
 */

//! One WMT command and its answer (Linux v6.12 `btmtk_usb_hci_wmt_sync`).
//! The command is HCI vendor command 0xfc6f, sent as any HCI command is,
//! whose parameters are the WMT header (direction 1, op, the length of
//! the flag and data as a little-endian word, flag) and the data. The
//! answer is no Command Complete but vendor event 0xe4, read from the
//! default endpoint (vendor request 0x01, wValue 0x30, 64 octets), which
//! reads empty until it has come: asked again each millisecond, for 10 s.
//! Its WMT header must name the op asked; its flag is the answer.

use alloc::vec::Vec;

use nonos_bt_hci::usb::{Setup, COMMAND};

use super::fail::Fail;
use super::port::Port;

pub const OP_PATCH: u8 = 0x01;
pub const OP_FUNC_CTRL: u8 = 0x06;
const OPCODE: u16 = 0xfc6f;
const EVENT: u8 = 0xe4;
const ANSWER: Setup = Setup { request_type: 0xc0, request: 0x01, value: 0x30, index: 0 };
const WAIT_MS: usize = 10_000;

/// The HCI command packet for `op`, or `None` when its parameters would
/// pass 255 octets.
pub fn command(op: u8, flag: u8, data: &[u8]) -> Option<Vec<u8>> {
    let params = u8::try_from(5 + data.len()).ok()?;
    let mut c = Vec::with_capacity(3 + params as usize);
    c.extend_from_slice(&OPCODE.to_le_bytes());
    c.extend_from_slice(&[params, 1, op]);
    c.extend_from_slice(&(data.len() as u16 + 1).to_le_bytes());
    c.push(flag);
    c.extend_from_slice(data);
    Some(c)
}

/// The flag of an answer to `op`: event code, length, then direction, op,
/// length (two octets), flag, all within the event's own length.
pub fn answer(op: u8, ev: &[u8]) -> Option<u8> {
    let end = 2 + *ev.get(1)? as usize;
    let ev = ev.get(..end)?;
    (ev.len() >= 7 && ev[0] == EVENT && ev[3] == op).then(|| ev[6])
}

pub fn wmt(p: &mut impl Port, op: u8, flag: u8, data: &[u8]) -> Result<u8, Fail> {
    let c = command(op, flag, data).ok_or(Fail::Image)?;
    p.control_out(COMMAND, &c).map_err(|_| Fail::Usb)?;
    let mut ev = [0u8; 64];
    for _ in 0..WAIT_MS {
        match p.control_in(ANSWER, &mut ev) {
            Ok(0) => p.wait(1),
            Ok(n) => return answer(op, &ev[..n]).ok_or(Fail::Answer),
            Err(_) => return Err(Fail::Usb),
        }
    }
    Err(Fail::Silent)
}
