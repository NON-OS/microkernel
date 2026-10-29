// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors (AGPL-3.0-or-later)

//! The project an epatch file is for, from its extension section
//! (btrtl.c, rtlbt_parse_firmware). The file ends with the signature
//! 51 04 fd 77; before it, instructions are read backwards from the end:
//! an opcode, then a length, then the data. Opcode 0xff ends them; opcode 0
//! of length 1 is the project id, its one data octet. Any other is passed
//! over: the next instruction ends its length plus three octets lower, as
//! btrtl.c steps. The walk stops above the 14-octet header and 3 octets
//! more. btrtl.c's table gives the RTL8821C project 10 (its LMP subversion
//! is 0x8821's, as the 8821A's project 2 is).

use crate::epatch::HEADER;
use crate::error::RtlError;

pub const EXTENSION_SIG: [u8; 4] = [0x51, 0x04, 0xfd, 0x77];
pub const PROJECT_8821C: u8 = 10;
const END: u8 = 0xff;
const PROJECT: u8 = 0;

pub fn project(fw: &[u8]) -> Result<u8, RtlError> {
    let body = fw.strip_suffix(&EXTENSION_SIG).ok_or(RtlError::Extension)?;
    let floor = HEADER + 3;
    let mut end = body.len();
    while end >= floor {
        let (op, len, data) = (body[end - 1], body[end - 2], body[end - 3]);
        if op == END {
            break;
        }
        if len == 0 {
            return Err(RtlError::NoProject);
        }
        if op == PROJECT && len == 1 {
            return Ok(data);
        }
        end = match end.checked_sub(3 + usize::from(len)) {
            Some(e) => e,
            None => break,
        };
    }
    Err(RtlError::NoProject)
}
