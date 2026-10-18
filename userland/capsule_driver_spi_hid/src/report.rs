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

//! One input report read whole: its header at the input header address,
//! then its body at the input body address; and a request written to the
//! output address. Reads and writes start with the firmware's opcode.

use alloc::vec::Vec;

use nonos_libc::mk_idle_ms;
use nonos_spi_hid::{body, header, read_frame, write_frame, Params, FRAME_HEAD, HEADER_LEN};

use crate::link::Link;
use crate::spi::transfer;

const ANSWER_READS: u32 = 50;
const ANSWER_MS: u64 = 10;

pub struct Report {
    pub kind: u8,
    pub id: u8,
    pub content: Vec<u8>,
}

/// The report the device holds, or `None` when it holds none it gives
/// whole. A header without its sync byte is no report; a fragment is read
/// and let go, so the device is not left holding it.
pub fn read(l: &Link) -> Option<Report> {
    let c = &l.choice;
    let rx = transfer(l.port, c.mode, c.hz, &read_frame(&c.params, HEADER_LEN)).ok()?;
    let h = header(rx.get(FRAME_HEAD..FRAME_HEAD + HEADER_LEN)?.try_into().ok()?)?;
    if h.body_len == 0 {
        return None;
    }
    let at_body = Params { input_address: c.body_address, ..c.params };
    let rx = transfer(l.port, c.mode, c.hz, &read_frame(&at_body, h.body_len)).ok()?;
    let b = body(rx.get(FRAME_HEAD..)?).filter(|_| h.last)?;
    Some(Report { kind: b.kind, id: b.id, content: b.content.to_vec() })
}

/// A request of `kind` for report `id` with `content`, written.
pub fn ask(l: &Link, kind: u8, id: u8, content: &[u8]) -> bool {
    let c = &l.choice;
    let Some(frame) = write_frame(&c.params, kind, id, content) else { return false };
    transfer(l.port, c.mode, c.hz, &frame).is_ok()
}

/// Reports read until one of `kind` comes; any other met meanwhile (a reset
/// announced, stale input) is let go. A device that never answers is given
/// up on after half a second.
pub fn await_kind(l: &Link, kind: u8) -> Option<Report> {
    for _ in 0..ANSWER_READS {
        match read(l) {
            Some(r) if r.kind == kind => return Some(r),
            Some(_) => {}
            None => {
                let _ = mk_idle_ms(ANSWER_MS);
            }
        }
    }
    None
}
